use anyhow::{Error, anyhow};
use gst::prelude::*;
use std::path::Path;
use tracing::{error, info};

use crate::list_devices::{Device, list_devices};
use crate::utils::SendEos;

/// Manages the state of a gst audio recording pipeline.
#[derive(derive_more::Debug)]
pub struct Recorder {
    pipeline: gst::Pipeline,
}

impl Recorder {
    /// Creates a new Recorder instance for a specific audio device.
    ///
    /// # Arguments
    /// * `device` - An optional `AudioDevice` to capture from. If `None`, the default device is used.
    /// * `output` - The output file path for the recorded audio.
    /// * `volume` - Audio volume level. Value can be between 0.0 and 10.0 with 1.0 being no change.
    pub fn new(device: &Device, output: impl AsRef<Path>, volume: f64) -> Result<Self, Error> {
        let audio_source = device.0.create_element(Some("src"))?;

        let volume = gst::ElementFactory::make("volume")
            .name("volume")
            .property("volume", volume)
            .build()?;

        let converter = gst::ElementFactory::make("audioconvert")
            .name("convert")
            .build()?;
        let encoder = gst::ElementFactory::make("lamemp3enc")
            .name("encoder")
            .build()?;
        let file_sink = gst::ElementFactory::make("filesink").name("sink").build()?;
        file_sink.set_property(
            "location",
            &output
                .as_ref()
                .to_str()
                .ok_or_else(|| anyhow!("Invalid output path"))?
                .to_string(),
        );

        // 2. Create the pipeline and add elements
        let pipeline = gst::Pipeline::with_name("recording-pipeline");
        pipeline.add_many([&audio_source, &volume, &converter, &encoder, &file_sink])?;

        // 3. Link the elements together
        gst::Element::link_many([&audio_source, &volume, &converter, &encoder, &file_sink])?;

        Ok(Self { pipeline })
    }

    pub fn start_spawned(self) {
        std::thread::spawn(move || {
            self.start_recording().unwrap();
            self.watch_bus();
        });
    }

    /// Starts the recording process.
    pub fn start_recording(&self) -> Result<(), Error> {
        info!("Starting pipeline...");
        self.pipeline
            .set_state(gst::State::Playing)
            .map_err(|_| anyhow!("Failed to start pipeline."))?;

        Ok(())
    }

    /// Stops the recording and finalizes the output file.
    pub fn stop_recording(&self) -> Result<(), Error> {
        info!("Stopping pipeline...");
        self.pipeline
            .set_state(gst::State::Null)
            .map_err(|_| anyhow!("Failed to stop pipeline."))?;
        info!("Pipeline stopped.");
        Ok(())
    }

    pub fn eos_sender(&self) -> SendEos {
        SendEos {
            pipeline: gst::glib::clone::Downgrade::downgrade(&self.pipeline),
        }
    }

    /// Lists all available audio input (microphone) devices.
    /// This is a static method and can be called without a `Recorder` instance.
    pub fn list_microphone_devices() -> Result<Vec<Device>, Error> {
        let mut names = std::collections::HashSet::new();
        let devices = list_devices(Some("Audio/Source"), None)?
            .into_iter()
            .filter(|device| {
                let name = device.name();
                names.insert(name)
            })
            .collect();
        Ok(devices)
    }

    /// Attaches a bus watcher to handle messages like errors or EOS.
    /// This should be run in a separate thread if you don't want to block.
    pub fn watch_bus(&self) {
        let bus = self.pipeline.bus().unwrap();
        for msg in bus.iter_timed(gst::ClockTime::NONE) {
            use gst::MessageView;

            match msg.view() {
                MessageView::Error(err) => {
                    error!(
                        "Error from element {:?}: {}",
                        err.src().map(|s| s.path_string()),
                        err.error()
                    );
                    error!("Debugging info: {:?}", err.debug());
                    break;
                }
                MessageView::Eos(..) => {
                    info!("End-of-stream reached.");
                    break;
                }
                _ => (),
            }
        }
    }
}
