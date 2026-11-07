fn main() {
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("gnu") {
        if let Ok(out_dir) = std::env::var("OUT_DIR") {
            let out_path = std::path::Path::new(&out_dir);
            let manifest_rc = out_path.join("default-manifest.rc");
            let _ = std::fs::write(
                &manifest_rc,
                concat!(
                    "1 24\n{\n",
                    " \"<assembly xmlns=\"\"urn:schemas-microsoft-com:asm.v1\"\" manifestVersion=\"\"1.0\"\">\"\n",
                    " \"<dependency>\"\n",
                    " \"<dependentAssembly>\"\n",
                    " \"<assemblyIdentity type=\"\"win32\"\" name=\"\"Microsoft.Windows.Common-Controls\"\" version=\"\"6.0.0.0\"\" processorArchitecture=\"\"*\"\" publicKeyToken=\"\"6595b64144ccf1df\"\" language=\"\"*\"\"/>\"\n",
                    " \"</dependentAssembly>\"\n",
                    " \"</dependency>\"\n",
                    " \"</assembly>\"\n",
                    "}\n"
                ),
            );
            let dummy_manifest = out_path.join("default-manifest.o");
            let status = std::process::Command::new("windres")
                .arg(&manifest_rc)
                .arg("-o")
                .arg(&dummy_manifest)
                .status();
            if status.is_err() || !status.as_ref().map(|s| s.success()).unwrap_or(false) {
                let _ = std::process::Command::new("gcc")
                    .args(["-c", "-x", "c", "-o"])
                    .arg(&dummy_manifest)
                    .arg("NUL")
                    .status();
            }
            let norm = out_dir.replace('\\', "/");
            println!("cargo:rustc-link-arg=-B{}/", norm);
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
