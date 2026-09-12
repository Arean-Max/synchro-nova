use crate::DriverInfo;
use std::{
    collections::HashSet,
    ffi::c_void,
    path::{Path, PathBuf},
};

pub(crate) fn collect_drivers() -> Vec<DriverInfo> {
    let mut seen = HashSet::new();
    let mut drivers = Vec::new();
    collect_display_drivers(&mut drivers, &mut seen);
    collect_service_drivers(&mut drivers, &mut seen);
    if drivers.is_empty() {
        drivers.push(DriverInfo {
            name: "Unknown driver".to_string(),
            provider: "Unknown".to_string(),
            version: "Unknown".to_string(),
            date: "Unknown".to_string(),
            class_name: "System".to_string(),
            status: "Unknown".to_string(),
            path: String::new(),
            search_query: "Windows driver latest version".to_string(),
        });
    }
    drivers.truncate(48);
    drivers
}

#[cfg(target_os = "windows")]
fn collect_display_drivers(drivers: &mut Vec<DriverInfo>, seen: &mut HashSet<String>) {
    #[repr(C)]
    struct DisplayDeviceW {
        cb: u32,
        device_name: [u16; 32],
        device_string: [u16; 128],
        state_flags: u32,
        device_id: [u16; 128],
        device_key: [u16; 128],
    }

    #[link(name = "User32")]
    unsafe extern "system" {
        fn EnumDisplayDevicesW(
            device_name: *const u16,
            dev_num: u32,
            display_device: *mut DisplayDeviceW,
            flags: u32,
        ) -> i32;
    }

    for index in 0..16 {
        let mut device = DisplayDeviceW {
            cb: std::mem::size_of::<DisplayDeviceW>() as u32,
            device_name: [0; 32],
            device_string: [0; 128],
            state_flags: 0,
            device_id: [0; 128],
            device_key: [0; 128],
        };

        let ok = unsafe { EnumDisplayDevicesW(std::ptr::null(), index, &mut device, 0) != 0 };
        if !ok {
            break;
        }

        let fallback_name = clean_text(&crate::utf16z_to_string(&device.device_string));
        if fallback_name.is_empty() {
            continue;
        }
        let registry_path = registry_machine_path(&crate::utf16z_to_string(&device.device_key));
        let mut info = display_registry_info(&registry_path);
        if info.name == "Unknown" {
            info.name = fallback_name;
        }
        insert_driver(drivers, seen, info);
    }
}

#[cfg(not(target_os = "windows"))]
fn collect_display_drivers(_drivers: &mut Vec<DriverInfo>, _seen: &mut HashSet<String>) {}

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

    make_driver(
        &name, &provider, &version, &date, "Display", "Detected", path,
    )
}

#[cfg(target_os = "windows")]
fn collect_service_drivers(drivers: &mut Vec<DriverInfo>, seen: &mut HashSet<String>) {
    use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let services = match hklm.open_subkey("SYSTEM\\CurrentControlSet\\Services") {
        Ok(value) => value,
        Err(_) => return,
    };

    for name in services.enum_keys().flatten() {
        if drivers.len() >= 48 {
            break;
        }
        let key = match services.open_subkey(&name) {
            Ok(value) => value,
            Err(_) => continue,
        };
        let driver_type = key.get_value::<u32, _>("Type").unwrap_or(0);
        if driver_type & 0x3 == 0 {
            continue;
        }
        let raw_display_name = key
            .get_value::<String, _>("DisplayName")
            .unwrap_or_else(|_| name.clone());
        let image_path = key.get_value::<String, _>("ImagePath").unwrap_or_default();
        if !is_relevant_driver(&name, &raw_display_name, &image_path) {
            continue;
        }
        let display_name = service_display_name(&name, &raw_display_name, &image_path);
        let path = expand_driver_path(&image_path);
        let version = path
            .as_ref()
            .and_then(|item| file_version(item).ok())
            .unwrap_or_else(|| "Unknown".to_string());
        let status = service_start_label(key.get_value::<u32, _>("Start").unwrap_or(3));
        let path_text = path
            .as_ref()
            .map(|item| item.to_string_lossy().to_string())
            .unwrap_or(image_path);
        let info = make_driver(
            &display_name,
            "Windows",
            &version,
            "Unknown",
            "Kernel",
            status,
            &path_text,
        );
        insert_driver(drivers, seen, info);
    }
}

#[cfg(not(target_os = "windows"))]
fn collect_service_drivers(_drivers: &mut Vec<DriverInfo>, _seen: &mut HashSet<String>) {}

fn insert_driver(drivers: &mut Vec<DriverInfo>, seen: &mut HashSet<String>, info: DriverInfo) {
    let key = if info.class_name == "Display" {
        format!("display|{}|{}", normalize_key(&info.name), info.version)
    } else {
        format!(
            "{}|{}|{}",
            normalize_key(&info.name),
            info.version,
            normalize_key(&info.path)
        )
    };
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
    status: &str,
    path: &str,
) -> DriverInfo {
    let name = clean_text(name);
    let provider = fallback(clean_text(provider), "Unknown");
    let version = fallback(clean_text(version), "Unknown");
    let date = fallback(clean_text(date), "Unknown");
    let class_name = fallback(clean_text(class_name), "System");
    let status = fallback(clean_text(status), "Detected");
    let path = clean_text(path);
    DriverInfo {
        search_query: format!("{name} {version} driver latest version"),
        name,
        provider,
        version,
        date,
        class_name,
        status,
        path,
    }
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
fn service_display_name(service: &str, raw: &str, path: &str) -> String {
    let cleaned = clean_text(raw);
    if !cleaned.starts_with('@') && !cleaned.contains("%") {
        return cleaned;
    }

    let lower = service.to_ascii_lowercase();
    let known = [
        ("btha2dp", "Bluetooth A2DP Driver"),
        ("bthenum", "Bluetooth Enumerator Driver"),
        ("bthhf", "Bluetooth Hands-Free Driver"),
        ("bthmini", "Bluetooth Miniport Driver"),
        ("bthport", "Bluetooth Port Driver"),
        ("hidusb", "USB HID Driver"),
        ("kbdhid", "Keyboard HID Driver"),
        ("mouhid", "Mouse HID Driver"),
        ("hdaudbus", "High Definition Audio Bus Driver"),
        ("acpiaudio", "ACPI Audio Driver"),
        ("stornvme", "Microsoft NVMe Storage Driver"),
        ("storahci", "Microsoft AHCI Storage Driver"),
        ("ndis", "NDIS Network Driver"),
        ("dxgkrnl", "DirectX Graphics Kernel Driver"),
    ];
    if let Some((_, label)) = known.iter().find(|(needle, _)| lower.contains(needle)) {
        return (*label).to_string();
    }

    expand_driver_path(path)
        .and_then(|item| {
            item.file_stem()
                .map(|stem| stem.to_string_lossy().to_string())
        })
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| service.to_string())
}

#[cfg(target_os = "windows")]
fn registry_machine_path(path: &str) -> String {
    path.trim()
        .trim_start_matches("\\Registry\\Machine\\")
        .trim_start_matches("\\REGISTRY\\MACHINE\\")
        .to_string()
}

#[cfg(target_os = "windows")]
fn is_relevant_driver(service: &str, display: &str, path: &str) -> bool {
    let value = format!("{service} {display} {path}").to_lowercase();
    [
        "nvlddmkm", "amdkmdag", "amdkmdap", "igdkmd", "dxgkrnl", "rt640", "netwtw", "e2f", "ndis",
        "usb", "hid", "kbd", "mou", "audio", "hda", "stornvme", "iastor", "storahci", "amdpsp",
        "bth",
    ]
    .iter()
    .any(|needle| value.contains(needle))
}

#[cfg(target_os = "windows")]
fn service_start_label(value: u32) -> &'static str {
    match value {
        0 => "Boot",
        1 => "System",
        2 => "Auto",
        3 => "Manual",
        4 => "Disabled",
        _ => "Detected",
    }
}

#[cfg(target_os = "windows")]
fn expand_driver_path(value: &str) -> Option<PathBuf> {
    let raw = value.trim().trim_matches('"');
    if raw.is_empty() {
        return None;
    }

    let system_root = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_string());
    let normalized = raw
        .strip_prefix("\\??\\")
        .unwrap_or(raw)
        .replace("\\SystemRoot", &system_root)
        .replace("\\systemroot", &system_root)
        .replace("%SystemRoot%", &system_root)
        .replace("%systemroot%", &system_root);

    let path = if normalized
        .get(1..3)
        .map(|part| part == ":\\")
        .unwrap_or(false)
    {
        PathBuf::from(normalized)
    } else if normalized.to_lowercase().starts_with("system32\\") {
        PathBuf::from(system_root).join(normalized)
    } else {
        PathBuf::from(normalized)
    };

    if path.exists() {
        Some(path)
    } else {
        None
    }
}

#[cfg(target_os = "windows")]
fn file_version(path: &Path) -> Result<String, String> {
    #[repr(C)]
    struct VsFixedFileInfo {
        dw_signature: u32,
        dw_struc_version: u32,
        dw_file_version_ms: u32,
        dw_file_version_ls: u32,
        dw_product_version_ms: u32,
        dw_product_version_ls: u32,
        dw_file_flags_mask: u32,
        dw_file_flags: u32,
        dw_file_os: u32,
        dw_file_type: u32,
        dw_file_subtype: u32,
        dw_file_date_ms: u32,
        dw_file_date_ls: u32,
    }

    #[link(name = "Version")]
    unsafe extern "system" {
        fn GetFileVersionInfoSizeW(file_name: *const u16, handle: *mut u32) -> u32;
        fn GetFileVersionInfoW(
            file_name: *const u16,
            handle: u32,
            len: u32,
            data: *mut c_void,
        ) -> i32;
        fn VerQueryValueW(
            block: *const c_void,
            sub_block: *const u16,
            buffer: *mut *mut c_void,
            len: *mut u32,
        ) -> i32;
    }

    let file = wide_null(&path.to_string_lossy());
    let mut handle = 0;
    let size = unsafe { GetFileVersionInfoSizeW(file.as_ptr(), &mut handle) };
    if size == 0 {
        return Err("No version info".to_string());
    }

    let mut data = vec![0_u8; size as usize];
    let ok = unsafe {
        GetFileVersionInfoW(
            file.as_ptr(),
            handle,
            size,
            data.as_mut_ptr() as *mut c_void,
        ) != 0
    };
    if !ok {
        return Err("Version read failed".to_string());
    }

    let mut buffer = std::ptr::null_mut();
    let mut len = 0;
    let root = wide_null("\\");
    let ok = unsafe {
        VerQueryValueW(
            data.as_ptr() as *const c_void,
            root.as_ptr(),
            &mut buffer,
            &mut len,
        ) != 0
    };
    if !ok || buffer.is_null() || len == 0 {
        return Err("Version query failed".to_string());
    }

    let info = unsafe { &*(buffer as *const VsFixedFileInfo) };
    if info.dw_signature != 0xFEEF04BD {
        return Err("Invalid version block".to_string());
    }

    Ok(format!(
        "{}.{}.{}.{}",
        (info.dw_file_version_ms >> 16) & 0xffff,
        info.dw_file_version_ms & 0xffff,
        (info.dw_file_version_ls >> 16) & 0xffff,
        info.dw_file_version_ls & 0xffff
    ))
}

#[cfg(target_os = "windows")]
fn wide_null(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}
