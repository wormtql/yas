use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum UI {
    Desktop,
    Mobile,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Platform {
    Windows,
    MacOS,
    Linux,
}

impl Platform {
    pub fn current() -> Self {
        #[cfg(target_os = "windows")]
        return Self::Windows;

        #[cfg(target_os = "macos")]
        return Self::MacOS;

        #[cfg(target_os = "linux")]
        return Self::Linux;
    }
}
