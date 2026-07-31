use crate::DriverInfo;
use std::collections::HashSet;

pub(crate) fn collect_drivers() -> Vec<DriverInfo> {
    let mut seen = HashSet::new();
    let mut drivers = Vec::new();

    #[cfg(target_os = "windows")]
    {
        collect_display_drivers(&mut drivers, &mut seen);
        collect_pnp_class_drivers(&mut drivers, &mut seen);
        crate::driver_check::enrich_with_real_version_checks(&mut drivers);
    }

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
        latest_version: String::new(),
    }
}
