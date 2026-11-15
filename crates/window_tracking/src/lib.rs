#[cfg(windows)]
mod windows;

use std::path::PathBuf;

#[cfg(windows)]
pub use windows::enumerate_windows;

#[derive(Debug)]
pub struct Window {
    pub title: String,
    pub minimized: bool,
    pub focused: bool,
    pub process_id: u32,
    pub process_path: Option<PathBuf>,
}
