use window_handling::WindowInfo;
use windows::WindowSrc;

mod windows;

pub trait InputSrc<W = window_handling::Window> {
    fn get_input_options() -> Vec<W>;

    /// element MUST be named "src"
    fn create_element_from_src(
        src: &dyn WindowInfo,
    ) -> Result<gst::element_factory::ElementBuilder<'static>, anyhow::Error>;
}

#[cfg(target_os = "windows")]
pub type DefaultSrc = WindowSrc;

#[cfg(target_os = "linux")]
pub type DefaultSrc = FakeSrc;

#[cfg(target_os = "macos")]
pub type DefaultSrc = FakeSrc;

#[derive(Debug)]
pub struct FakeSrc;

impl InputSrc for FakeSrc {
    fn create_element_from_src(
        _src: &dyn WindowInfo,
    ) -> Result<gst::element_factory::ElementBuilder<'static>, anyhow::Error> {
        Err(anyhow::anyhow!(
            "FakeSrc is not implemented yet. Please use a different source."
        ))
    }

    fn get_input_options() -> Vec<window_handling::Window> {
        vec![]
    }
}
