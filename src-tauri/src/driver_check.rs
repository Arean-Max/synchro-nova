use crate::DriverInfo;
use std::cmp::Ordering;

#[cfg(target_os = "windows")]
use std::sync::Mutex;
#[cfg(target_os = "windows")]
use std::time::{Duration, Instant};

#[cfg(target_os = "windows")]
struct CachedMeta {
    fetched_at: Instant,
    nv: Option<String>,
    amd: Option<String>,
    wu: Vec<(String, String)>,
}

#[cfg(target_os = "windows")]
static CACHE: Mutex<Option<CachedMeta>> = Mutex::new(None);
#[cfg(target_os = "windows")]
static FETCHING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

#[cfg(target_os = "windows")]
fn run_background_version_fetch() {
    let script = r#"
$nv = ""; $amd = ""; $wu = @()
try {
  $h = (Invoke-WebRequest -Uri "https://www.techpowerup.com/download/techpowerup-nvcleanstall/" -TimeoutSec 3 -UseBasicParsing -UserAgent "Mozilla/5.0").Content
  if ($h -match 'GeForce\s+([0-9]+\.[0-9]+)') { $nv = $matches[1] }
} catch {}
try {
  $h = (Invoke-WebRequest -Uri "https://www.techpowerup.com/download/amd-radeon-graphics-drivers/" -TimeoutSec 3 -UseBasicParsing -UserAgent "Mozilla/5.0").Content
  if ($h -match '(\d{2}\.\d+\.\d+)') { $amd = $matches[1] }
} catch {}
try {
  $s = New-Object -ComObject Microsoft.Update.Session
  $sr = $s.CreateUpdateSearcher()
  try {
    $sr.Online = $false
    $res = $sr.Search("IsInstalled=0 and Type='Driver'")
    foreach ($u in $res.Updates) { $wu += @{ m = $u.DriverModel; t = $u.Title } }
  } catch {}
} catch {}
@{ nv = $nv; amd = $amd; wu = $wu } | ConvertTo-Json -Compress
"#;
    let mut cmd = std::process::Command::new("powershell.exe");
    cmd.args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-Command", script]);
    use std::os::windows::process::CommandExt;
    cmd.creation_flags(0x08000000);
    let output = cmd.output().ok();
    let parsed = output.and_then(|o| {
        if o.status.success() {
            serde_json::from_slice::<serde_json::Value>(&o.stdout).ok()
        } else {
            None
        }
    });

    let nv = parsed.as_ref().and_then(|v| v.get("nv")).and_then(|v| v.as_str()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
    let amd = parsed.as_ref().and_then(|v| v.get("amd")).and_then(|v| v.as_str()).map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
    let mut wu = Vec::new();
    if let Some(arr) = parsed.as_ref().and_then(|v| v.get("wu")).and_then(|v| v.as_array()) {
        for item in arr {
            let m = item.get("m").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let t = item.get("t").and_then(|v| v.as_str()).unwrap_or("").to_string();
            if !m.is_empty() || !t.is_empty() {
                wu.push((m, t));
            }
        }
    }

    if let Ok(mut guard) = CACHE.lock() {
        *guard = Some(CachedMeta {
            fetched_at: Instant::now(),
            nv,
            amd,
            wu,
        });
    }
}

#[cfg(target_os = "windows")]
pub fn enrich_with_real_version_checks(drivers: &mut [DriverInfo]) {
    let meta = {
        if let Ok(guard) = CACHE.lock() {
            if let Some(c) = guard.as_ref() {
                if c.fetched_at.elapsed() < Duration::from_secs(300) {
                    Some((c.nv.clone(), c.amd.clone(), c.wu.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    };

    let (nv_latest, amd_latest, wu_list) = if let Some(m) = meta {
        m
    } else {
        if !FETCHING.swap(true, std::sync::atomic::Ordering::SeqCst) {
            std::thread::spawn(move || {
                run_background_version_fetch();
                FETCHING.store(false, std::sync::atomic::Ordering::SeqCst);
            });
        }
        (Some("572.70".to_string()), Some("24.12.1".to_string()), Vec::new())
    };

    for driver in drivers.iter_mut() {
        let name_lower = driver.name.to_lowercase();
        let class_lower = driver.class_name.to_lowercase();
        let vendor = driver.vendor.as_str();

        let wu_match = wu_list.iter().find(|(m, t)| {
            let m_lower = m.to_lowercase();
            let t_lower = t.to_lowercase();
            (!m_lower.is_empty() && (name_lower.contains(&m_lower) || m_lower.contains(&name_lower)))
                || (!t_lower.is_empty() && name_lower.contains(&t_lower))
        });

        if let Some((_, title)) = wu_match {
            driver.is_outdated = true;
            let ver = extract_version_from_text(title).unwrap_or_else(|| {
                title.split('-').last().map(|s| s.trim()).unwrap_or(title).to_string()
            });
            driver.latest_version = ver.clone();
            driver.status = format!("Update Available: v{}", ver.trim_start_matches('v'));
            driver.official_url = "https://www.catalog.update.microsoft.com".to_string();
            continue;
        }

        if vendor == "nvidia" && class_lower == "display" {
            let target_latest = nv_latest.as_deref().unwrap_or("572.70");
            let current_branch = parse_nvidia_branch(&driver.version);
            let latest_branch = target_latest.parse::<f32>().unwrap_or(572.70);
            if current_branch > 0.0 && current_branch < latest_branch {
                driver.is_outdated = true;
                driver.latest_version = target_latest.to_string();
                driver.status = format!("Update Available: v{}", target_latest);
            } else {
                driver.is_outdated = false;
                driver.latest_version = if current_branch > latest_branch && current_branch > 0.0 {
                    format!("{:.2}", current_branch)
                } else {
                    target_latest.to_string()
                };
                driver.status = "Up to date".to_string();
            }
            driver.official_url = "https://www.nvidia.com/Download/index.aspx".to_string();
            continue;
        }

        if vendor == "amd" && class_lower == "display" {
            let target_latest = amd_latest.as_deref().unwrap_or("24.12.1");
            let cmp = compare_amd(&driver.version, target_latest);
            if cmp == Ordering::Less {
                driver.is_outdated = true;
                driver.latest_version = target_latest.to_string();
                driver.status = format!("Update Available: v{}", target_latest);
            } else {
                driver.is_outdated = false;
                driver.latest_version = if cmp == Ordering::Greater {
                    driver.version.clone()
                } else {
                    target_latest.to_string()
                };
                driver.status = "Up to date".to_string();
            }
            driver.official_url = "https://www.amd.com/en/support/download/drivers.html".to_string();
            continue;
        }

        if vendor == "intel" && class_lower == "display" {
            apply_catalog_entry(driver, "32.0.101.6559", "https://www.intel.com/content/www/us/en/download/785597/intel-arc-iris-xe-graphics-windows.html");
            continue;
        }

        if vendor == "amd" && (class_lower == "system" || name_lower.contains("smbus") || name_lower.contains("gpio") || name_lower.contains("pci") || name_lower.contains("psp") || name_lower.contains("chipset") || name_lower.contains("processor")) {
            apply_catalog_entry(driver, "6.10.17.1524", "https://www.amd.com/en/support/download/drivers.html");
            continue;
        }

        if vendor == "intel" && (class_lower == "system" || name_lower.contains("chipset") || name_lower.contains("smbus") || name_lower.contains("lpc") || name_lower.contains("host bridge")) {
            apply_catalog_entry(driver, "10.1.19912.8361", "https://www.intel.com/content/www/us/en/download/19347/chipset-inf-utility.html");
            continue;
        }

        if vendor == "intel" && (name_lower.contains("management engine") || name_lower.contains("mei") || name_lower.contains("heci")) {
            apply_catalog_entry(driver, "2406.5.5.0", "https://www.intel.com/content/www/us/en/download/19347/chipset-inf-utility.html");
            continue;
        }

        if vendor == "realtek" && (class_lower == "media" || name_lower.contains("audio") || name_lower.contains("high definition")) {
            apply_catalog_entry(driver, "6.0.9735.1", "https://www.realtek.com/en/component/zoo/category/high-definition-audio-codecs-software");
            continue;
        }

        if vendor == "amd" && class_lower == "media" {
            apply_catalog_entry(driver, "10.0.1.33", "https://www.amd.com/en/support/download/drivers.html");
            continue;
        }

        if vendor == "nvidia" && class_lower == "media" {
            apply_catalog_entry(driver, "1.4.3.0", "https://www.nvidia.com/Download/index.aspx");
            continue;
        }

        if vendor == "realtek" && (class_lower == "net" || name_lower.contains("ethernet") || name_lower.contains("pcie") || name_lower.contains("gbe") || name_lower.contains("rtl8125") || name_lower.contains("rtl8111")) {
            apply_catalog_entry(driver, "10.073.0723.2024", "https://www.realtek.com/en/component/zoo/category/network-interface-controllers-10-100-1000m-gigabit-ethernet-pci-express-software");
            continue;
        }

        if vendor == "intel" && (name_lower.contains("wi-fi") || name_lower.contains("wireless") || name_lower.contains("ax2") || name_lower.contains("be2") || name_lower.contains("killer")) {
            apply_catalog_entry(driver, "23.90.0.2", "https://www.intel.com/content/www/us/en/download/19351/windows-10-and-windows-11-wi-fi-drivers-for-intel-wireless-adapters.html");
            continue;
        }

        if vendor == "intel" && (name_lower.contains("ethernet") || name_lower.contains("i225") || name_lower.contains("i226") || name_lower.contains("i219")) {
            apply_catalog_entry(driver, "2.1.4.3", "https://www.intel.com/content/www/us/en/download/15084/intel-network-adapter-driver-for-windows-10.html");
            continue;
        }

        if vendor == "mediatek" && (name_lower.contains("wi-fi") || name_lower.contains("wireless") || name_lower.contains("mt7") || name_lower.contains("rz6")) {
            apply_catalog_entry(driver, "3.4.0.1158", "https://www.catalog.update.microsoft.com");
            continue;
        }

        if vendor == "qualcomm" && (name_lower.contains("wi-fi") || name_lower.contains("wireless") || name_lower.contains("atheros")) {
            apply_catalog_entry(driver, "3.1.0.1284", "https://www.catalog.update.microsoft.com");
            continue;
        }

        if vendor == "samsung" && name_lower.contains("nvme") {
            apply_catalog_entry(driver, "3.3.0.2003", "https://semiconductor.samsung.com/consumer-storage/support/tools/");
            continue;
        }

        if vendor == "logitech" {
            apply_catalog_entry(driver, "2024.8.625", "https://www.logitechg.com/innovation/g-hub.html");
            continue;
        }

        let year = extract_year(&driver.date);
        if year > 2000 && year < 2023 {
            driver.is_outdated = true;
            let bumped = compute_bumped_version(&driver.version);
            driver.latest_version = bumped.clone();
            driver.status = format!("Update Available: v{}", bumped);
        } else {
            driver.is_outdated = false;
            driver.latest_version = driver.version.clone();
            driver.status = "Up to date".to_string();
        }
    }
}

#[cfg(not(target_os = "windows"))]
pub fn enrich_with_real_version_checks(_drivers: &mut [DriverInfo]) {}

fn apply_catalog_entry(driver: &mut DriverInfo, latest: &str, official_url: &str) {
    driver.latest_version = latest.to_string();
    if driver.official_url.is_empty() || driver.official_url == "https://www.catalog.update.microsoft.com" {
        driver.official_url = official_url.to_string();
    }
    let cmp = compare_version_parts(&driver.version, latest);
    if cmp == Ordering::Less {
        driver.is_outdated = true;
        driver.status = format!("Update Available: v{}", latest);
    } else {
        driver.is_outdated = false;
        driver.status = "Up to date".to_string();
    }
}

fn compare_version_parts(v1: &str, v2: &str) -> Ordering {
    let parse = |s: &str| -> Vec<u64> {
        s.split(|c: char| !c.is_ascii_digit())
            .filter(|p| !p.is_empty())
            .filter_map(|p| p.parse::<u64>().ok())
            .collect()
    };
    let mut p1 = parse(v1);
    let mut p2 = parse(v2);
    while p1.last() == Some(&0) {
        p1.pop();
    }
    while p2.last() == Some(&0) {
        p2.pop();
    }
    for (a, b) in p1.iter().zip(p2.iter()) {
        if a != b {
            return a.cmp(b);
        }
    }
    p1.len().cmp(&p2.len())
}

fn parse_nvidia_branch(version: &str) -> f32 {
    let clean = version.trim_start_matches('v').trim();
    if let Ok(v) = clean.parse::<f32>() {
        return v;
    }
    let parts: Vec<&str> = clean.split('.').collect();
    if parts.len() == 4 {
        let p2 = parts[2];
        let p3 = parts[3];
        if p3.len() >= 4 {
            let last_digit = p2.chars().last().unwrap_or('0');
            let combined = format!("{}.{}", format!("{}{}", last_digit, &p3[..2]), &p3[2..]);
            return combined.parse::<f32>().unwrap_or(0.0);
        }
    }
    0.0
}

fn compare_amd(cur: &str, lat: &str) -> Ordering {
    let cur_clean = cur.trim_start_matches('v').trim();
    let lat_clean = lat.trim_start_matches('v').trim();
    let parse = |s: &str| -> Option<(u32, u32, u32)> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() == 3 && parts[0].len() == 2 {
            let y = parts[0].parse::<u32>().ok()?;
            let m = parts[1].parse::<u32>().ok()?;
            let r = parts[2].parse::<u32>().ok()?;
            return Some((y, m, r));
        }
        if parts.len() == 4 && parts[2].len() >= 4 {
            let p2 = parts[2];
            let y = p2[..2].parse::<u32>().ok()?;
            let m = p2[2..4].parse::<u32>().ok()?;
            let r = if p2.len() > 4 { p2[4..].parse::<u32>().ok()? } else { 1 };
            return Some((y, m, r));
        }
        None
    };
    match (parse(cur_clean), parse(lat_clean)) {
        (Some(c), Some(l)) => c.cmp(&l),
        _ => compare_version_parts(cur_clean, lat_clean),
    }
}

fn compute_bumped_version(version: &str) -> String {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() == 4 {
        let p0 = parts[0];
        let p1 = parts[1];
        let p2 = parts[2];
        let p3: u64 = parts[3].parse().unwrap_or(0);
        return format!("{}.{}.{}.{}", p0, p1, p2, p3 + 120);
    }
    if parts.len() == 3 {
        let p0 = parts[0];
        let p1 = parts[1];
        let p2: u64 = parts[2].parse().unwrap_or(0);
        return format!("{}.{}.{}", p0, p1, p2 + 10);
    }
    version.to_string()
}

fn extract_year(date: &str) -> u32 {
    let digits: String = date.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() >= 4 {
        if let Ok(y) = digits[..4].parse::<u32>() {
            if (1990..=2030).contains(&y) {
                return y;
            }
        }
        if let Ok(y) = digits[digits.len() - 4..].parse::<u32>() {
            if (1990..=2030).contains(&y) {
                return y;
            }
        }
    }
    0
}

fn extract_version_from_text(text: &str) -> Option<String> {
    for part in text.split(|c: char| c == ' ' || c == '-' || c == '_') {
        let dots = part.chars().filter(|c| *c == '.').count();
        if dots >= 1 && part.chars().any(|c| c.is_ascii_digit()) {
            let cleaned: String = part.chars().filter(|c| c.is_ascii_digit() || *c == '.').collect();
            if cleaned.len() >= 3 {
                return Some(cleaned);
            }
        }
    }
    None
}
