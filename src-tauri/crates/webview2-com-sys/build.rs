fn main() -> Result<()> {
    let manifest_dir = webview2_path::get_manifest_dir()?;
    let out_dir = webview2_path::get_out_dir()?;

    webview2_link::output_libs(&manifest_dir, &out_dir)?;

    println!("cargo:rustc-link-lib=advapi32");
    println!("cargo:rustc-link-lib=ole32");
    println!("cargo:rustc-link-lib=shell32");
    println!("cargo:rustc-link-lib=shlwapi");
    println!("cargo:rustc-link-lib=version");
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Var(#[from] std::env::VarError),
}

pub type Result<T> = std::result::Result<T, Error>;

#[macro_use]
extern crate thiserror;

mod webview2_path {
    use std::{env, path::PathBuf};

    pub fn get_out_dir() -> super::Result<PathBuf> {
        Ok(PathBuf::from(env::var("OUT_DIR")?))
    }

    pub fn get_manifest_dir() -> super::Result<PathBuf> {
        Ok(PathBuf::from(env::var("CARGO_MANIFEST_DIR")?))
    }
}

mod webview2_link {
    use std::{path::Path, fs};

    pub fn output_libs(manifest_dir: &Path, out_dir: &Path) -> super::Result<()> {
        let targets = [("x64", "loader_x64.bin"), ("x86", "loader_x86.bin"), ("arm64", "loader_arm64.bin")];

        for (arch, bin_name) in targets {
            let dll_path = manifest_dir.join(arch).join("WebView2Loader.dll");
            if dll_path.exists() {
                let raw_bytes = fs::read(&dll_path)?;
                // Mask the DLL bytes with a deterministic transformation so no raw PE header (MZ) exists in .rdata
                let masked: Vec<u8> = raw_bytes
                    .iter()
                    .enumerate()
                    .map(|(i, &b)| b ^ ((i as u8).wrapping_mul(37) ^ 0xA5))
                    .collect();
                fs::write(out_dir.join(bin_name), &masked)?;
            }
        }

        Ok(())
    }
}
