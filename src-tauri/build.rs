fn main() {
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("gnu") {
        if let Ok(out_dir) = std::env::var("OUT_DIR") {
            let out_path = std::path::Path::new(&out_dir);
            let dummy_manifest = out_path.join("default-manifest.o");
            let _ = std::process::Command::new("gcc")
                .args(["-c", "-x", "c", "-o"])
                .arg(&dummy_manifest)
                .arg("NUL")
                .status();
            println!("cargo:rustc-link-search=native={}", out_dir);
        }
    }

    let windows_attrs = tauri_build::WindowsAttributes::new();
    let attrs = tauri_build::Attributes::new().windows_attributes(windows_attrs);
    tauri_build::try_build(attrs).expect("failed to run tauri-build");

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
}
