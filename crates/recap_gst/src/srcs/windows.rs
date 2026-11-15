use anyhow::Context;
use window_handling::{EnumWindows, Window, WindowInfo, raw_window_handle::RawWindowHandle};

use super::InputSrc;

#[derive(Debug)]
pub struct WindowSrc;

impl InputSrc for WindowSrc {
    fn get_input_options() -> Vec<Window> {
        Window::enum_windows()
            .into_iter()
            .filter(|w| w.title().is_some())
            .collect()
    }

    fn create_element_from_src(
        src: &dyn WindowInfo,
    ) -> Result<gst::element_factory::ElementBuilder<'static>, anyhow::Error> {
        // Change from using ? operator to map_err to convert HandleError to anyhow::Error
        let window_handle = src.window_handle().context("Failed to get window handle")?;

        let RawWindowHandle::Win32(hwnd) = window_handle.as_raw() else {
            return Err(anyhow::anyhow!("Failed to get window handle"));
        };
        let src = gst::ElementFactory::make("d3d12screencapturesrc")
            .property("show-cursor", true)
            .property_from_str("capture-api", "wgc")
            .property_from_str("window-capture-mode", "client")
            .property("window-handle", hwnd.hwnd.get() as u64);

        Ok(src)
    }
}
