fn patch_webview2_dll(path: &std::path::Path) {
    if let Ok(mut bytes) = std::fs::read(path) {
        if bytes.len() > 0x150 && &bytes[0..2] == b"MZ" {
            let e_lfanew = u32::from_le_bytes([bytes[0x3c], bytes[0x3d], bytes[0x3e], bytes[0x3f]]) as usize;
            if e_lfanew + 168 <= bytes.len() {
                let magic = u16::from_le_bytes([bytes[e_lfanew + 24], bytes[e_lfanew + 25]]);
                let sec_dir_offset = if magic == 0x20b {
                    e_lfanew + 24 + 112 + 4 * 8
                } else {
                    e_lfanew + 24 + 96 + 4 * 8
                };
                if sec_dir_offset + 8 <= bytes.len() {
                    let rva = u32::from_le_bytes([
                        bytes[sec_dir_offset],
                        bytes[sec_dir_offset + 1],
                        bytes[sec_dir_offset + 2],
                        bytes[sec_dir_offset + 3],
                    ]);
                    if rva != 0 {
                        // Strip IMAGE_DIRECTORY_ENTRY_SECURITY so Windows loads it as standard unsigned user-mode DLL
                        bytes[sec_dir_offset..sec_dir_offset + 8].fill(0);
                        let _ = std::fs::write(path, bytes);
                    }
                }
            }
        }
    }
}

fn patch_all_webview2(dir: &std::path::Path) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                // Avoid descending into deeply nested non-build directories
                let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if name != ".git" && name != "node_modules" {
                    patch_all_webview2(&p);
                }
            } else if p.file_name().and_then(|n| n.to_str()) == Some("WebView2Loader.dll") {
                patch_webview2_dll(&p);
            }
        }
    }
}

fn main() {
    tauri_build::build();

    // Dynamically locate MinGW system libraries for x86_64-pc-windows-gnu without hardcoded paths.
    // Resolves through environment variables (MINGW_PREFIX, MINGW_HOME) or the active GCC toolchain in PATH.
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("gnu") {
        if let Ok(prefix) = std::env::var("MINGW_PREFIX") {
            println!("cargo:rustc-link-search=native={}/lib", prefix);
            println!("cargo:rustc-link-search=native={}/x86_64-w64-mingw32/lib", prefix);
        } else if let Ok(home) = std::env::var("MINGW_HOME") {
            println!("cargo:rustc-link-search=native={}/lib", home);
            println!("cargo:rustc-link-search=native={}/x86_64-w64-mingw32/lib", home);
        } else if let Ok(path) = std::env::var("PATH") {
            for p in std::env::split_paths(&path) {
                let gcc_path = p.join("gcc.exe");
                if gcc_path.exists() {
                    if let Some(parent) = p.parent() {
                        let lib1 = parent.join("lib");
                        let lib2 = parent.join("x86_64-w64-mingw32").join("lib");
                        if lib1.exists() {
                            println!("cargo:rustc-link-search=native={}", lib1.display());
                        }
                        if lib2.exists() {
                            println!("cargo:rustc-link-search=native={}", lib2.display());
                        }
                    }
                    break;
                }
            }
        }
    }

    // Ensure all generated WebView2Loader.dll files are patched against 0xc000012f
    patch_all_webview2(std::path::Path::new("target"));
}
