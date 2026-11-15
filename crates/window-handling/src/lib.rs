pub use raw_window_handle;
use raw_window_handle::HasWindowHandle;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "windows")]
pub type Window = windows::Window;

#[cfg(target_os = "windows")]
pub type Monitor = windows::Monitor;

type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to open process")]
    OpenProcess,
    #[error("String empty")]
    StringEmpty,
    #[error("Failed to get window rect")]
    GetWindowRect,

    #[error(transparent)]
    Monitor(#[from] MonitorError),
}

#[derive(Debug, thiserror::Error)]
pub enum MonitorError {
    #[error("Failed to get monitor video mode")]
    GetMonitorVideoMode,
    #[error("Failed to get monitor DPI")]
    GetMonitorDPI,
    #[error("Failed to get monitor position")]
    GetMonitorPosition,
    #[error("Failed to get monitor size")]
    GetMonitorSize,
}

#[derive(Debug)]
pub enum WindowEvent {}

pub trait WindowInfo<M: MonitorInfo = Monitor>:
    HasWindowHandle + std::fmt::Debug + Send + Sync
{
    fn size(&self) -> Result<(i32, i32)>;
    fn position(&self) -> Result<(i32, i32)>;
    fn title(&self) -> Option<String>;
    fn minimized(&self) -> bool;
    fn focused(&self) -> bool;
    fn process_id(&self) -> u32;
    fn process_path(&self) -> Result<String>;
    fn current_monitor(&self) -> Result<M>;
    fn dpi(&self) -> Result<u32>;
    fn scale_factor(&self) -> f64;
}

pub trait EnumWindows<T: WindowInfo> {
    fn enum_windows() -> Vec<T>;

    fn forground_window() -> T;
}

pub trait WindowEventStream {
    fn stream() -> impl futures::Stream<Item = WindowEvent> + Send;
}

pub trait MonitorInfo {
    fn id(&self) -> u128;
    fn native_id(&self) -> u64;
    fn name(&self) -> Option<String>;
    fn position(&self) -> Result<(i32, i32)>;
    fn dpi(&self) -> Result<u32>;
    fn scale_factor(&self) -> f64;
    fn size(&self) -> Result<(i32, i32)>;
    fn bit_depth(&self) -> Result<u32>;
    fn refresh_rate(&self) -> Result<u32>;
}
