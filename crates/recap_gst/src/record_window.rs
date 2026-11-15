//! this module contains the gst pipeline for recording a window

use std::{ops::Range, path::PathBuf, sync::Arc};

use tracing::{error, info};

use crate::{srcs::InputSrc, utils::SendEos};
use gst::{
    element_error,
    glib::{
        self,
        object::{Cast, ObjectExt},
    },
    prelude::{
        DeviceExt, ElementExt as _, ElementExtManual as _, GstBinExtManual as _, GstObjectExt as _,
        PadExt as _, PadExtManual as _,
    },
};
use rand::Rng;
use window_handling::WindowInfo;

#[derive(Debug, Clone)]
pub struct OutputOptions {
    pub width: Size,
    pub height: Size,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum Event {
    NewData,
    FinishedEncoding(Option<gst::ClockTime>),
}

#[derive(derive_more::Debug)]
pub struct PipelineBuilder<'a> {
    inference_enabled: bool,
    src: Option<&'a dyn WindowInfo>,
    #[debug(skip)]
    inference_callback: Option<
        Box<
            dyn FnMut(&gst_app::AppSink, gst::Sample) -> Result<(), gst::FlowError>
                + Send
                + 'static,
        >,
    >,
    #[debug(skip)]
    on_event_callback: Option<Arc<dyn Fn(Event) + Send + Sync + 'static>>,
    fps: i32,
    output_options: Option<OutputOptions>,
    audio_device: Option<crate::list_devices::Device>,
    /// Audio volume level. Value can be between 0.0 and 10.0 with 1.0 being no change.
    audio_volume: f64,
}

impl<'a> Default for PipelineBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> PipelineBuilder<'a> {
    pub fn new() -> Self {
        Self {
            inference_enabled: false,
            src: None,
            inference_callback: None,
            on_event_callback: None,
            fps: 20,
            output_options: None,
            audio_device: None,
            audio_volume: 1.0,
        }
    }

    pub fn fps(mut self, fps: i32) -> Self {
        self.fps = fps;
        self
    }

    pub fn output_options(mut self, options: OutputOptions) -> Self {
        self.output_options = Some(options);
        self
    }

    pub fn on_event<F>(mut self, callback: F) -> Self
    where
        F: Fn(Event) + Send + Sync + 'static,
    {
        self.on_event_callback = Some(Arc::new(callback));
        self
    }

    pub fn enable_inference(mut self, enabled: bool) -> Self {
        self.inference_enabled = enabled;
        self
    }

    pub fn with_inference_callback<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&gst_app::AppSink, gst::Sample) -> Result<(), gst::FlowError> + Send + 'static,
    {
        self.inference_callback = Some(Box::new(callback));
        self
    }

    pub fn input_src(mut self, src: &'a dyn WindowInfo) -> Self {
        self.src = Some(src);
        self
    }

    pub fn audio_input(mut self, device: Option<crate::list_devices::Device>) -> Self {
        self.audio_device = device;
        self
    }

    /// Sets audio volume level. Value can be between 0.0 and 10.0 with 1.0 being no change.
    pub fn audio_volume(mut self, volume: f64) -> Self {
        self.audio_volume = volume;
        self
    }

    pub fn build(self) -> anyhow::Result<PipelineBus> {
        let file_options = self
            .output_options
            .ok_or(anyhow::anyhow!("Output options are required"))?;

        let on_event_callback = self
            .on_event_callback
            .ok_or(anyhow::anyhow!("On event callback is required"))?;

        let src_target = self
            .src
            .ok_or(anyhow::anyhow!("Input source is required"))?;

        let pipeline = gst::Pipeline::default();

        let src = crate::srcs::DefaultSrc::create_element_from_src(src_target)?
            .name("src")
            .build()?;

        let input_filter_caps = &gst_video::VideoCapsBuilder::new()
            .framerate(gst::Fraction::new(self.fps, 1))
            .build();

        let input_filter = gst::ElementFactory::make("capsfilter")
            .property("caps", input_filter_caps)
            .build()?;

        let tee = gst::ElementFactory::make("tee").build()?;

        // add the src, video_rate, and tee to the pipeline and link them
        pipeline.add_many([&src, &input_filter, &tee])?;
        gst::Element::link_many([&src, &input_filter, &tee])?;

        // file path
        let file_queue = gst::ElementFactory::make("queue").build()?;

        file_queue.connect_closure(
            "overrun",
            false,
            glib::closure!(move |queue: &gst::Element| {
                element_error!(queue, gst::ResourceError::Failed, ("file_queue is full"));
                error!("file_queue is full: {}", queue.name());
            }),
        );

        // let video_profile = gst_pbutils::EncodingVideoProfile::builder(
        //     &gst_video::VideoCapsBuilder::for_encoding("video/x-h264")
        //         .field("profile", "high")
        //         .build(),
        // )
        // .restriction(
        //     &gst_video::VideoCapsBuilder::new()
        //         .width(640)
        //         .height(480)
        //         .build(),
        // )
        // .presence(0)
        // .build();

        // let container_profile: gst_pbutils::EncodingContainerProfile =
        //     gst_pbutils::EncodingContainerProfile::builder(
        //         &gst::Caps::builder("video/quicktime")
        //             .field("variant", "iso")
        //             .build(),
        //     )
        //     .add_profile(video_profile)
        //     .build();

        // let encoder = gst::ElementFactory::make("encodebin")
        //     .property("profile", &container_profile)
        //     .build()?;

        // // add the file elements to the pipeline and link them
        //  pipeline.add_many(&[&file_queue, &encoder, &file_sink])?;
        //  gst::Element::link_many(&[&file_queue, &encoder, &file_sink])?;

        // let file_sink = gst::ElementFactory::make("filesink")
        // .property("location", &file_options.path)
        // .build()?;

        let file_convertscale = gst::ElementFactory::make("videoconvertscale").build()?;

        let encoder_filter_caps = &gst_video::VideoCapsBuilder::new()
            .set_height_width(file_options.width.clone(), file_options.height.clone())
            .framerate(gst::Fraction::new(self.fps, 1))
            .pixel_aspect_ratio(gst::Fraction::new(1, 1))
            .build();

        let encoder_filter = gst::ElementFactory::make("capsfilter")
            .property("caps", encoder_filter_caps)
            .build()?;

        let encoder_queue = create_queue("encoder_queue")?;

        encoder_queue.connect_closure(
            "overrun",
            false,
            glib::closure!(move |queue: &gst::Element| {
                element_error!(queue, gst::ResourceError::Failed, ("encoder_queue is full"));
                error!("encoder_queue is full: {}", queue.name());
            }),
        );

        let encoder = Encoder::RngX264;
        tracing::info!(module = "capture::encoder", "Encoder: {:?}", encoder);

        let encoder = encoder.build()?;

        let encoder_caps = gst_video::VideoCapsBuilder::for_encoding("video/x-h264")
            // 10% chance of high-4:4:4 profile this will mean almost 100% color fidelity
            .field_if("profile", "high-4:4:4", rand::rng().random_bool(0.1))
            .build();

        let encoder_caps = gst::ElementFactory::make("capsfilter")
            .property("caps", &encoder_caps)
            .build()?;

        let h264parse = gst::ElementFactory::make("h264parse").build()?;

        let h264timestamper = gst::ElementFactory::make("h264timestamper").build()?;

        let video_file_queue = create_queue("video_file_queue")?;
        let video_path = &[
            &file_queue,
            &file_convertscale,
            &encoder_filter,
            &encoder_queue,
            &encoder,
            &encoder_caps,
            &h264parse,
            &h264timestamper,
            &video_file_queue,
        ];

        // add the file elements to the pipeline and link them
        pipeline.add_many(video_path)?;
        gst::Element::link_many(video_path)?;

        let mp4mux = gst::ElementFactory::make("mp4mux").build()?;

        let file_sink = gst::ElementFactory::make("filesink")
            .property("location", &file_options.path)
            .build()?;

        let muxer_elements = &[&mp4mux, &file_sink];

        // add the muxer and sink to the pipeline and link them
        pipeline.add_many(muxer_elements)?;
        gst::Element::link_many(muxer_elements)?;

        // link the tee to the file queue
        let tee_file_pad = tee.request_pad_simple("src_%u").unwrap();
        let queue_file_pad = file_queue.static_pad("sink").unwrap();
        tee_file_pad.link(&queue_file_pad)?;

        // Link video queue to the muxer's video pad
        let mux_video_pad = mp4mux
            .request_pad_simple("video_%u")
            .expect("Failed to get video pad from muxer");
        let video_queue_src_pad = video_file_queue
            .static_pad("src")
            .expect("Failed to get src pad from video queue");
        video_queue_src_pad.link(&mux_video_pad)?;

        if let Some(audio_device) = self.audio_device {
            let audio_src = audio_device.0.create_element(None)?;

            let audio_volume = gst::ElementFactory::make("volume")
                .property("volume", self.audio_volume)
                .build()?;

            let audio_convert = gst::ElementFactory::make("audioconvert").build()?;

            let audio_enc = gst::ElementFactory::make("avenc_aac").build()?;

            let audio_queue = create_queue("audio_queue")?;
            let audio_path = &[
                &audio_src,
                &audio_volume,
                &audio_convert,
                &audio_enc,
                &audio_queue,
            ];
            pipeline.add_many(audio_path)?;
            gst::Element::link_many(audio_path)?;

            // Link audio queue to the muxer's audio pad
            let mux_audio_pad = mp4mux
                .request_pad_simple("audio_%u")
                .expect("Failed to get audio pad from muxer");
            let audio_queue_src_pad = audio_queue
                .static_pad("src")
                .expect("Failed to get src pad from audio queue");
            audio_queue_src_pad.link(&mux_audio_pad)?;
        }

        // inference pipeline
        if self.inference_enabled {
            let mut callback = self
                .inference_callback
                .ok_or(anyhow::anyhow!("Inference callback is required"))?;

            let inference_queue = gst::ElementFactory::make("queue").build()?;

            let inference_convertscale = gst::ElementFactory::make("videoconvertscale").build()?;

            let inference_sink = gst_app::AppSink::builder()
                .sync(false)
                .drop(true)
                .wait_on_eos(false)
                .async_(false)
                .caps(
                    &gst_video::VideoCapsBuilder::new()
                        .format(gst_video::VideoFormat::Rgb)
                        .pixel_aspect_ratio(gst::Fraction::new(1, 1))
                        .set_height_width(file_options.width, file_options.height)
                        .build(),
                )
                .build();

            inference_sink.set_callbacks(
                gst_app::AppSinkCallbacks::builder()
                    .new_sample(move |appsink| {
                        let sample = appsink.pull_sample().map_err(|_| gst::FlowError::Error)?;
                        callback(appsink, sample)?;

                        Ok(gst::FlowSuccess::Ok)
                    })
                    .build(),
            );

            pipeline.add_many([
                &inference_queue,
                &inference_convertscale,
                inference_sink.upcast_ref(),
            ])?;
            gst::Element::link_many([
                &inference_queue,
                &inference_convertscale,
                inference_sink.upcast_ref(),
            ])?;

            // link the tee to the inference queue
            let tee_inference_pad = tee.request_pad_simple("src_%u").unwrap();
            let queue_inference_pad = inference_queue.static_pad("sink").unwrap();
            tee_inference_pad.link(&queue_inference_pad)?;
        }

        // callback on new data arrived in src
        // new frame in src element
        let src_pad = src
            .static_pad("src")
            .ok_or(anyhow::anyhow!("Failed to get src pad from src element"))?;
        src_pad.add_probe(gst::PadProbeType::BUFFER, {
            let on_event_callback = on_event_callback.clone();
            move |_, _| {
                on_event_callback(Event::NewData);
                gst::PadProbeReturn::Ok
            }
        });

        let encoder_sink = encoder.static_pad("src").ok_or(anyhow::anyhow!(
            "Failed to get src pad from encoder element"
        ))?;
        encoder_sink.add_probe(gst::PadProbeType::BUFFER, {
            let on_event_callback = on_event_callback.clone();
            move |_, info| {
                let timestamp = info.buffer().and_then(|b| b.pts());
                on_event_callback(Event::FinishedEncoding(timestamp));
                gst::PadProbeReturn::Ok
            }
        });

        Ok(PipelineBus {
            bus: pipeline.bus().unwrap(),
            pipeline,
        })
    }
}

fn create_queue(name: &str) -> anyhow::Result<gst::Element> {
    let queue = gst::ElementFactory::make("queue")
        .property("max-size-time", 30 * 1000 * 1000 * 1000u64) // 30 seconds
        .property("max-size-buffers", 20 * 30u32) // 20 fps * 30 seconds
        .property("max-size-bytes", 500_000_000u32) // 500 MB
        .name(name)
        .build()?;

    queue.connect_closure(
        "overrun",
        false,
        glib::closure!(move |queue: &gst::Element| {
            element_error!(
                queue,
                gst::ResourceError::Failed,
                ("{} is full", queue.name())
            );
            error!("{} is full: {}", queue.name(), queue.name());
        }),
    );

    Ok(queue)
}

#[derive(Debug)]
pub struct PipelineBus {
    bus: gst::Bus,
    pipeline: gst::Pipeline,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PipelineStats {}

impl PipelineBus {
    pub fn run_till_end(&self) -> anyhow::Result<PipelineStats> {
        self.pipeline.set_state(gst::State::Playing)?;

        #[cfg(feature = "dot_graph")]
        {
            let bin_ref = self.pipeline.upcast_ref::<gst::Bin>();
            bin_ref.debug_to_dot_file(gst::DebugGraphDetails::ALL, "pipeline.dot");
        }

        for msg in self.bus.iter_timed(gst::ClockTime::NONE) {
            use gst::MessageView;

            match msg.view() {
                MessageView::Eos(..) => {
                    info!("received eos");
                    break;
                }
                MessageView::Error(err) => {
                    self.pipeline.set_state(gst::State::Null)?;
                    return Err(anyhow::anyhow!(
                        "Error received from element {}: {}",
                        err.src()
                            .map(gst::prelude::GstObjectExt::path_string)
                            .unwrap_or_else(|| "None".into()),
                        err.error()
                    ));
                }
                _ => (),
            }
        }

        let stats = self.get_stats()?;

        self.pipeline.set_state(gst::State::Null)?;
        Ok(stats)
    }

    /// Get the stats of the pipeline
    pub fn get_stats(&self) -> anyhow::Result<PipelineStats> {
        Ok(PipelineStats {})
    }

    pub fn eos_sender(&self) -> SendEos {
        SendEos {
            pipeline: gst::glib::clone::Downgrade::downgrade(&self.pipeline),
        }
    }

    pub fn raw_pipeline(&self) -> &gst::Pipeline {
        &self.pipeline
    }
    pub fn bus(&self) -> &gst::Bus {
        &self.bus
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
enum Encoder {
    AmfH264,
    X264,
    RngX264,
}

impl Encoder {
    fn instance(self) -> gst::element_factory::ElementBuilder<'static> {
        match self {
            Encoder::AmfH264 => gst::ElementFactory::make("amfh264enc")
                .property_from_str("preset", "quality")
                .property_from_str("rate-control", "vbr")
                .property("b-frames", 2)
                .property("b-reference", true)
                .property("pre-encode", true)
                .property("gop-size", 2)
                .property_from_str("pa-hqmb-mode", "auto")
                .property_from_str("pa-taq-mode", "mode2"),
            // .property("max-bitrate", 2000_u32)
            Encoder::X264 => gst::ElementFactory::make("x264enc")
                .property("quantizer", 18u32)
                .property_from_str("pass", "qual")
                .property("bitrate", 4096_u32)
                .property_from_str("psy-tune", "film"),
            Encoder::RngX264 => {
                let rng = rand::rng().random_range(14u32..=18);
                gst::ElementFactory::make("x264enc")
                    .property("quantizer", rng)
                    .property_from_str("pass", "qual")
                    .property("bitrate", 4096_u32)
                    .property_from_str("psy-tune", "film")
            }
        }
    }

    fn build(self) -> Result<gst::Element, gst::glib::BoolError> {
        self.instance().name("encoder").build()
    }
}

#[derive(Debug, Clone)]
pub enum Size {
    Range(Range<i32>),
    Exact(i32),
}

impl From<i32> for Size {
    fn from(value: i32) -> Self {
        Size::Exact(value)
    }
}

impl From<Range<i32>> for Size {
    fn from(value: Range<i32>) -> Self {
        Size::Range(value)
    }
}

trait SetHeightWidth {
    fn set_height_width(self, width: Size, height: Size) -> Self;
}

impl<T> SetHeightWidth for gst_video::VideoCapsBuilder<T> {
    fn set_height_width(self, width: Size, height: Size) -> Self {
        let width_set: gst_video::VideoCapsBuilder<T> = match width {
            Size::Exact(w) => self.width(w),
            Size::Range(r) => self.width_range(r),
        };
        match height {
            Size::Exact(h) => width_set.height(h),
            Size::Range(r) => width_set.height_range(r),
        }
    }
}
