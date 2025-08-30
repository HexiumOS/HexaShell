use std::path::PathBuf;

pub fn home() -> Option<PathBuf> {
    // First, try HOME (common on Unix)
    if let Some(home) = std::env::var_os("HOME") {
        if !home.is_empty() {
            return Some(PathBuf::from(home));
        }
    }

    // Then try USERPROFILE (Windows)
    if let Some(userprofile) = std::env::var_os("USERPROFILE") {
        if !userprofile.is_empty() {
            return Some(PathBuf::from(userprofile));
        }
    }

    None
}
