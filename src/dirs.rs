use std::path::PathBuf;

pub fn home() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    std::env::var_os("HOME")
        .and_then(|h| if h.is_empty() { None } else { Some(h) })
        .map(PathBuf::from)
        .or_else(|| {
            #[cfg(windows)]
            {
                std::env::var_os("USERPROFILE")
                    .and_then(|h| if h.is_empty() { None } else { Some(h) })
                    .map(PathBuf::from)
            }
            #[cfg(not(windows))]
            {
                None
            }
        })
}
