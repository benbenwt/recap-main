use gst::prelude::{DeviceExt, DeviceMonitorExt, DeviceMonitorExtManual, GstObjectExt};

#[derive(Debug, Clone, PartialEq)]
pub struct Device(pub gst::Device);

impl Device {
    pub fn name(&self) -> String {
        self.0.display_name().to_string()
    }

    pub fn adaptor_name(&self) -> String {
        self.0.name().to_string()
    }
}

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.display_name())
    }
}

pub fn list_devices(
    class: Option<impl AsRef<str>>,
    caps: Option<gst::Caps>,
) -> Result<Vec<Device>, gst::glib::error::BoolError> {
    let monitor = gst::DeviceMonitor::new();
    monitor.add_filter(class.as_ref().map(|c| c.as_ref()), caps.as_ref());
    monitor.start()?;

    let devices = monitor.devices().into_iter().map(Device).collect();
    monitor.stop();
    Ok(devices)
}
