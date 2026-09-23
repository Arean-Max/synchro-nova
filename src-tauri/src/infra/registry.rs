use std::fmt;
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ, KEY_WRITE};
use winreg::RegKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootKey {
    Hkcu,
    Hklm,
}

impl RootKey {
    pub fn to_hkey(self) -> winreg::HKEY {
        match self {
            RootKey::Hkcu => HKEY_CURRENT_USER,
            RootKey::Hklm => HKEY_LOCAL_MACHINE,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            RootKey::Hkcu => "HKCU",
            RootKey::Hklm => "HKLM",
        }
    }
}

#[derive(Debug, Clone)]
pub enum RegistryValue {
    Dword(u32),
    String(String),
}

#[derive(Debug)]
pub enum RegistryError {
    OpenKeyFailed {
        root: &'static str,
        path: String,
        source: std::io::Error,
    },
    CreateKeyFailed {
        root: &'static str,
        path: String,
        source: std::io::Error,
    },
    GetValueFailed {
        root: &'static str,
        path: String,
        name: String,
        source: std::io::Error,
    },
    SetValueFailed {
        root: &'static str,
        path: String,
        name: String,
        source: std::io::Error,
    },
    DeleteValueFailed {
        root: &'static str,
        path: String,
        name: String,
        source: std::io::Error,
    },
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpenKeyFailed { root, path, source } => {
                write!(f, "Failed to open registry key {}\\{}: {}", root, path, source)
            }
            Self::CreateKeyFailed { root, path, source } => {
                write!(f, "Failed to create registry key {}\\{}: {}", root, path, source)
            }
            Self::GetValueFailed { root, path, name, source } => {
                write!(f, "Failed to get value {}\\{}\\{}: {}", root, path, name, source)
            }
            Self::SetValueFailed { root, path, name, source } => {
                write!(f, "Failed to set value {}\\{}\\{}: {}", root, path, name, source)
            }
            Self::DeleteValueFailed { root, path, name, source } => {
                write!(f, "Failed to delete value {}\\{}\\{}: {}", root, path, name, source)
            }
        }
    }
}

impl std::error::Error for RegistryError {}

pub struct SafeRegistry;

impl SafeRegistry {
    pub fn get_dword(root: RootKey, path: &str, name: &str) -> Result<u32, RegistryError> {
        let hkey = RegKey::predef(root.to_hkey());
        let key = hkey.open_subkey_with_flags(path, KEY_READ).map_err(|source| {
            RegistryError::OpenKeyFailed {
                root: root.as_str(),
                path: path.to_string(),
                source,
            }
        })?;
        key.get_value::<u32, _>(name).map_err(|source| {
            RegistryError::GetValueFailed {
                root: root.as_str(),
                path: path.to_string(),
                name: name.to_string(),
                source,
            }
        })
    }

    pub fn get_string(root: RootKey, path: &str, name: &str) -> Result<String, RegistryError> {
        let hkey = RegKey::predef(root.to_hkey());
        let key = hkey.open_subkey_with_flags(path, KEY_READ).map_err(|source| {
            RegistryError::OpenKeyFailed {
                root: root.as_str(),
                path: path.to_string(),
                source,
            }
        })?;
        key.get_value::<String, _>(name).map_err(|source| {
            RegistryError::GetValueFailed {
                root: root.as_str(),
                path: path.to_string(),
                name: name.to_string(),
                source,
            }
        })
    }

    pub fn set_dword(root: RootKey, path: &str, name: &str, value: u32) -> Result<(), RegistryError> {
        let hkey = RegKey::predef(root.to_hkey());
        let (key, _) = hkey.create_subkey(path).map_err(|source| {
            eprintln!("[SafeRegistry] Failed to create/open subkey {}\\{}: {}", root.as_str(), path, source);
            RegistryError::CreateKeyFailed {
                root: root.as_str(),
                path: path.to_string(),
                source,
            }
        })?;
        key.set_value(name, &value).map_err(|source| {
            eprintln!("[SafeRegistry] Failed to set DWORD {}\\{}\\{} = {}: {}", root.as_str(), path, name, value, source);
            RegistryError::SetValueFailed {
                root: root.as_str(),
                path: path.to_string(),
                name: name.to_string(),
                source,
            }
        })
    }

    pub fn set_string(root: RootKey, path: &str, name: &str, value: &str) -> Result<(), RegistryError> {
        let hkey = RegKey::predef(root.to_hkey());
        let (key, _) = hkey.create_subkey(path).map_err(|source| {
            eprintln!("[SafeRegistry] Failed to create/open subkey {}\\{}: {}", root.as_str(), path, source);
            RegistryError::CreateKeyFailed {
                root: root.as_str(),
                path: path.to_string(),
                source,
            }
        })?;
        key.set_value(name, &value).map_err(|source| {
            eprintln!("[SafeRegistry] Failed to set string {}\\{}\\{} = '{}': {}", root.as_str(), path, name, value, source);
            RegistryError::SetValueFailed {
                root: root.as_str(),
                path: path.to_string(),
                name: name.to_string(),
                source,
            }
        })
    }

    pub fn delete_value(root: RootKey, path: &str, name: &str) -> Result<(), RegistryError> {
        let hkey = RegKey::predef(root.to_hkey());
        let key = hkey.open_subkey_with_flags(path, KEY_WRITE).map_err(|source| {
            RegistryError::OpenKeyFailed {
                root: root.as_str(),
                path: path.to_string(),
                source,
            }
        })?;
        key.delete_value(name).map_err(|source| {
            eprintln!("[SafeRegistry] Failed to delete value {}\\{}\\{}: {}", root.as_str(), path, name, source);
            RegistryError::DeleteValueFailed {
                root: root.as_str(),
                path: path.to_string(),
                name: name.to_string(),
                source,
            }
        })
    }

    pub fn read_existing_value(root: RootKey, path: &str, name: &str) -> Option<RegistryValue> {
        if let Ok(v) = Self::get_dword(root, path, name) {
            return Some(RegistryValue::Dword(v));
        }
        if let Ok(s) = Self::get_string(root, path, name) {
            return Some(RegistryValue::String(s));
        }
        None
    }
}
