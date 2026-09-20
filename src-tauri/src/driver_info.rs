use crate::DriverInfo;
use std::collections::HashSet;

pub(crate) fn collect_drivers() -> Vec<DriverInfo> {
    let mut seen = HashSet::new();
    let mut drivers = Vec::new();

    #[cfg(target_os = "windows")]
    {
        // 1. Primary active display adapters first
        collect_display_drivers(&mut drivers, &mut seen);

        // 2. Comprehensive PnP registry class drivers
        collect_pnp_class_drivers(&mut drivers, &mut seen);
    }

    if drivers.is_empty() {
        drivers.push(DriverInfo {
            name: "System Driver".to_string(),
            provider: "Microsoft".to_string(),
            version: "10.0.26100".to_string(),
            date: "2026".to_string(),
            class_name: "System".to_string(),
            status: "Up to date".to_string(),
            path: String::new(),
            search_query: "Windows drivers latest".to_string(),
            hardware_id: String::new(),
            vendor: "microsoft".to_string(),
            is_outdated: false,
            official_url: "https://www.catalog.update.microsoft.com".to_string(),
        });
    }

    // Sort drivers logically:
    // 1. Class priority (Display -> Media -> Net -> Storage -> System -> Peripherals)
    // 2. Outdated drivers first within category
    // 3. Known vendors before generic/microsoft
    // 4. Alphabetical by name
    drivers.sort_by(|a, b| {
        let class_rank = |c: &str| match c.to_lowercase().as_str() {
            "display" => 0,
            "media" => 1,
            "net" => 2,
            "storage" | "scsiadapter" | "diskdrive" => 3,
            "system" | "processor" => 4,
            _ => 5,
        };
        let r_a = class_rank(&a.class_name);
        let r_b = class_rank(&b.class_name);
        if r_a != r_b {
            return r_a.cmp(&r_b);
        }

        let out_a = if a.is_outdated { 0 } else { 1 };
        let out_b = if b.is_outdated { 0 } else { 1 };
        if out_a != out_b {
            return out_a.cmp(&out_b);
        }

        let vendor_rank = |v: &str| match v {
            "nvidia" | "amd" => 0,
            "intel" => 1,
            "realtek" => 2,
            "logitech" | "qualcomm" | "mediatek" => 3,
            "microsoft" => 5,
            _ => 4,
        };
        let v_a = vendor_rank(&a.vendor);
        let v_b = vendor_rank(&b.vendor);
        if v_a != v_b {
            return v_a.cmp(&v_b);
        }

        a.name.cmp(&b.name)
    });

    drivers.truncate(60);
    drivers
}

#[cfg(target_os = "windows")]
fn collect_display_drivers(drivers: &mut Vec<DriverInfo>, seen: &mut HashSet<String>) {
    for index in 0..16 {
        let mut device = crate::ffi::DisplayDeviceW::default();
        let ok = unsafe {
            crate::ffi::winapi::EnumDisplayDevicesW(std::ptr::null(), index, &mut device, 0) != 0
        };
        if !ok {
            break;
        }

        let fallback_name = clean_text(&crate::utf16z_to_string(&device.device_string));
        if fallback_name.is_empty() {
            continue;
        }
        let registry_path = registry_machine_path(&crate::utf16z_to_string(&device.device_key));
        let mut info = display_registry_info(&registry_path);
        if info.name == "Unknown" || info.name.is_empty() {
            info.name = fallback_name;
        }
        insert_driver(drivers, seen, info);
    }
}

#[cfg(target_os = "windows")]
fn display_registry_info(path: &str) -> DriverInfo {
    use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = path.trim();
    let reg = hklm.open_subkey(key).ok();
    let name = reg
        .as_ref()
        .and_then(|item| item.get_value::<String, _>("DriverDesc").ok())
        .or_else(|| {
            reg.as_ref().and_then(|item| {
                item.get_value::<String, _>("HardwareInformation.AdapterString")
                    .ok()
            })
        })
        .unwrap_or_else(|| "Unknown".to_string());
    let provider = reg
        .as_ref()
        .and_then(|item| item.get_value::<String, _>("ProviderName").ok())
        .unwrap_or_else(|| "Unknown".to_string());
    let version = reg
        .as_ref()
        .and_then(|item| item.get_value::<String, _>("DriverVersion").ok())
        .unwrap_or_else(|| "Unknown".to_string());
    let date = reg
        .as_ref()
        .and_then(|item| item.get_value::<String, _>("DriverDate").ok())
        .unwrap_or_else(|| "Unknown".to_string());
    let matching_id = reg
        .as_ref()
        .and_then(|item| item.get_value::<String, _>("MatchingDeviceId").ok())
        .unwrap_or_default();

    make_driver(
        &name,
        &provider,
        &version,
        &date,
        "Display",
        path,
        &matching_id,
    )
}

#[cfg(target_os = "windows")]
fn collect_pnp_class_drivers(drivers: &mut Vec<DriverInfo>, seen: &mut HashSet<String>) {
    use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let class_root = match hklm.open_subkey("SYSTEM\\CurrentControlSet\\Control\\Class") {
        Ok(k) => k,
        Err(_) => return,
    };

    for class_guid in class_root.enum_keys().flatten() {
        if drivers.len() >= 120 {
            break;
        }

        let class_key = match class_root.open_subkey(&class_guid) {
            Ok(k) => k,
            Err(_) => continue,
        };

        let raw_class: String = class_key
            .get_value::<String, _>("Class")
            .unwrap_or_else(|_| "System".to_string());
        let class_name = normalize_class(&raw_class);

        // Filter for meaningful hardware classes
        if !is_relevant_class(&class_name) {
            continue;
        }

        for sub_name in class_key.enum_keys().flatten() {
            if drivers.len() >= 120 {
                break;
            }
            if !sub_name.chars().all(|c| c.is_ascii_digit()) {
                continue;
            }

            let dev_key = match class_key.open_subkey(&sub_name) {
                Ok(k) => k,
                Err(_) => continue,
            };

            let desc: String = match dev_key.get_value::<String, _>("DriverDesc") {
                Ok(v) => clean_text(&v),
                Err(_) => continue,
            };

            if desc.is_empty() || desc == "Unknown" {
                continue;
            }

            let version: String = match dev_key.get_value::<String, _>("DriverVersion") {
                Ok(v) => clean_text(&v),
                Err(_) => continue,
            };

            if version.is_empty() {
                continue;
            }

            let provider: String = dev_key
                .get_value::<String, _>("ProviderName")
                .unwrap_or_else(|_| "Unknown".to_string());
            let date: String = dev_key
                .get_value::<String, _>("DriverDate")
                .unwrap_or_else(|_| "Unknown".to_string());
            let matching_id: String = dev_key
                .get_value::<String, _>("MatchingDeviceId")
                .unwrap_or_default();
            let inf_path: String = dev_key
                .get_value::<String, _>("InfPath")
                .unwrap_or_default();

            if is_ignorable_device(&desc, &provider, &class_name) {
                continue;
            }

            let info = make_driver(
                &desc,
                &provider,
                &version,
                &date,
                &class_name,
                &inf_path,
                &matching_id,
            );

            insert_driver(drivers, seen, info);
        }
    }
}

fn insert_driver(drivers: &mut Vec<DriverInfo>, seen: &mut HashSet<String>, info: DriverInfo) {
    let key = normalize_key(&info.name);
    if key.is_empty() {
        return;
    }
    if seen.insert(key) {
        drivers.push(info);
    }
}

fn make_driver(
    name: &str,
    provider: &str,
    version: &str,
    date: &str,
    class_name: &str,
    path: &str,
    hardware_id: &str,
) -> DriverInfo {
    let name = clean_text(name);
    let provider = fallback(clean_text(provider), "Unknown");
    let version = fallback(clean_text(version), "Unknown");
    let date = fallback(clean_date(date), "Unknown");
    let class_name = fallback(clean_text(class_name), "System");
    let path = clean_text(path);
    let hardware_id = clean_text(hardware_id);

    let vendor = detect_vendor(&provider, &name, &hardware_id);
    let is_outdated = check_if_outdated(&date, &vendor, &class_name);
    let status = if is_outdated {
        "Update Available".to_string()
    } else {
        "Up to date".to_string()
    };
    let official_url = resolve_official_url(&vendor, &name, &hardware_id);

    DriverInfo {
        search_query: format!("{name} {version} driver download official"),
        name,
        provider,
        version,
        date,
        class_name,
        status,
        path,
        hardware_id,
        vendor,
        is_outdated,
        official_url,
    }
}

fn detect_vendor(provider: &str, name: &str, hardware_id: &str) -> String {
    let p = provider.to_lowercase();
    let n = name.to_lowercase();
    let id = hardware_id.to_lowercase();

    if p.contains("nvidia") || n.contains("geforce") || n.contains("nvidia") || id.contains("ven_10de") {
        "nvidia".to_string()
    } else if p.contains("advanced micro devices")
        || p.contains("amd")
        || n.contains("radeon")
        || n.contains("ryzen")
        || id.contains("ven_1002")
        || id.contains("ven_1022")
    {
        "amd".to_string()
    } else if p.contains("intel") || n.contains("intel") || id.contains("ven_8086") {
        "intel".to_string()
    } else if p.contains("realtek") || n.contains("realtek") || id.contains("ven_10ec") {
        "realtek".to_string()
    } else if p.contains("logitech") || n.contains("logitech") || id.contains("vid_046d") {
        "logitech".to_string()
    } else if p.contains("qualcomm") || n.contains("qualcomm") || id.contains("ven_168c") {
        "qualcomm".to_string()
    } else if p.contains("mediatek") || n.contains("mediatek") || id.contains("ven_14c3") {
        "mediatek".to_string()
    } else if p.contains("broadcom") || n.contains("broadcom") || id.contains("ven_14e4") {
        "broadcom".to_string()
    } else if p.contains("samsung") || n.contains("samsung") {
        "samsung".to_string()
    } else if p.contains("asus") || n.contains("asus") || n.contains("asustek") {
        "asus".to_string()
    } else if p.contains("microsoft") {
        "microsoft".to_string()
    } else {
        "generic".to_string()
    }
}

fn check_if_outdated(date: &str, vendor: &str, class_name: &str) -> bool {
    // Microsoft built-in and generic system drivers are updated via Windows Update
    if vendor == "microsoft" || vendor == "generic" {
        return false;
    }

    let year = extract_year(date);
    if year == 0 {
        return false;
    }

    let class_lower = class_name.to_lowercase();
    if class_lower == "display" {
        // GPUs release updates frequently; anything older than 2024 is candidate
        year < 2024
    } else if class_lower == "net" || class_lower == "media" {
        // Network / Audio: older than 2023
        year < 2023
    } else {
        // Chipset, storage, peripherals: older than 2021
        year < 2021
    }
}

fn is_ignorable_device(name: &str, provider: &str, class_name: &str) -> bool {
    let n = name.to_lowercase();
    let p = provider.to_lowercase();

    // 1. Virtual, debug, or software devices
    if n.contains("wan miniport")
        || n.contains("kernel debug")
        || n.contains("directshow")
        || n.contains("ras async")
        || n.contains("remote ndis")
        || n.contains("virtual")
        || n.contains("composite bus")
        || n.contains("pnp-software")
        || n.contains("pnp software")
        || n.contains("terminal server")
        || n.contains("root audio")
        || n.contains("volume manager")
        || n.contains("volume snapshot")
        || n.contains("generic volume")
        || n.contains("generic pnp monitor")
        || n.contains("generic non-pnp monitor")
        || n.contains("motherboard resources")
        || n.contains("system timer")
        || n.contains("interrupt controller")
        || n.contains("numeric data processor")
        || n.contains("acpi fan")
        || n.contains("acpi processor")
        || n.contains("acpi power")
        || n.contains("acpi thermal")
        || n.contains("pci memory controller")
        || n.contains("legacy device")
        || n.contains("print queue")
        || n.contains("software device")
        || n.contains("usbncm host device")
        || n.contains("power engine plug-in")
        || n.contains("platform monitoring technology")
        || n.contains("pawnio")
        || n.contains("usb composite device")
        || n.contains("usb root hub")
        || n.contains("acpi x64-based pc")
        || n.contains("computer device")
        || n.contains("standard ps/2")
        || n.contains("microsoft ps/2")
        || n.contains("standard sata ahci")
        || n.contains("pci standard host")
        || n.contains("pci standard isa")
        || n.contains("pci standard pci")
        || n.contains("pci standard ram")
        || n.contains("standard dual channel")
        || n.contains("pci-to-pci bridge")
        || n.contains("standard nvm express controller")
        || n.contains("storage spaces")
        || n.contains("audio endpoint")
    {
        return true;
    }

    // 2. Class-specific filtering
    let cls = class_name.to_lowercase();
    if cls == "system" && (p.contains("microsoft") || p == "unknown") {
        return true;
    }

    if cls == "net" && p.contains("microsoft") {
        return true;
    }

    if (cls == "storage" || cls == "diskdrive") && (n == "disk drive" || n == "cd-rom drive") {
        return true;
    }

    if cls == "peripherals" && (p.contains("microsoft") || n.contains("hid-compliant") || n.contains("hid keyboard")) {
        return true;
    }

    false
}

fn extract_year(date: &str) -> u32 {
    for part in date.split(|c: char| c == '-' || c == '/' || c == '.' || c.is_whitespace()) {
        if part.len() == 4 && part.chars().all(|c| c.is_ascii_digit()) {
            if let Ok(y) = part.parse::<u32>() {
                if (1990..=2030).contains(&y) {
                    return y;
                }
            }
        }
    }
    0
}

fn resolve_official_url(vendor: &str, name: &str, hardware_id: &str) -> String {
    match vendor {
        "nvidia" => {
            let n = name.to_lowercase();
            if n.contains("rtx") || n.contains("gtx") || n.contains("geforce") {
                "https://www.nvidia.com/en-us/geforce/drivers/".to_string()
            } else {
                "https://www.nvidia.com/Download/index.aspx".to_string()
            }
        }
        "amd" => "https://www.amd.com/en/support/download/drivers.html".to_string(),
        "intel" => "https://www.intel.com/content/www/us/en/support/detect.html".to_string(),
        "logitech" => "https://support.logi.com/".to_string(),
        _ => {
            let query = if !hardware_id.is_empty() {
                hardware_id.replace('&', "%26")
            } else {
                name.replace(' ', "+")
            };
            format!("https://www.catalog.update.microsoft.com/Search.aspx?q={query}")
        }
    }
}

fn normalize_class(class: &str) -> String {
    let lower = class.to_lowercase();
    if lower.contains("display") || lower.contains("video") || lower.contains("graphics") {
        "Display".to_string()
    } else if lower.contains("net") {
        "Net".to_string()
    } else if lower.contains("media") || lower.contains("audio") || lower.contains("sound") {
        "Media".to_string()
    } else if lower.contains("scsi") || lower.contains("storage") || lower.contains("disk") || lower.contains("hdc") {
        "Storage".to_string()
    } else if lower.contains("bluetooth") {
        "Bluetooth".to_string()
    } else if lower.contains("mouse") || lower.contains("keyboard") || lower.contains("hid") || lower.contains("input") {
        "Peripherals".to_string()
    } else {
        "System".to_string()
    }
}

fn is_relevant_class(class: &str) -> bool {
    matches!(
        class,
        "Display" | "Net" | "Media" | "Storage" | "Bluetooth" | "Peripherals" | "System"
    )
}

fn clean_date(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return "Unknown".to_string();
    }
    trimmed.replace('-', "/")
}

fn fallback(value: String, fallback: &str) -> String {
    if value.is_empty() {
        fallback.to_string()
    } else {
        value
    }
}

fn clean_text(value: &str) -> String {
    value
        .chars()
        .filter(|ch| !ch.is_control())
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(180)
        .collect()
}

fn normalize_key(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .collect::<String>()
        .to_ascii_lowercase()
}

#[cfg(target_os = "windows")]
fn registry_machine_path(path: &str) -> String {
    path.trim()
        .trim_start_matches("\\Registry\\Machine\\")
        .trim_start_matches("\\REGISTRY\\MACHINE\\")
        .to_string()
}
