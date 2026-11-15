use std::sync::atomic::AtomicU32;

use gst::prelude::*;

use tracing::info;

use crate::discover::{FileInfo, VideoInfo, discover_file_info};

pub fn get_first_video_stream_info(
    path: impl AsRef<std::path::Path>,
) -> Result<(FileInfo, VideoInfo), anyhow::Error> {
    let info = discover_file_info(path)?;
    let video_info = info.top_level_streams[0]
        .as_container()
        .ok_or(anyhow::anyhow!("No container found in the file"))?
        .streams[0]
        .as_video()
        .ok_or(anyhow::anyhow!("No video stream found in the file"))?
        .to_owned();

    Ok((info, video_info))
}

/// Get the real frame count of a video file using GStreamer.
/// This function creates a GStreamer pipeline that reads the video file and counts the frames.
pub fn get_real_frame_count(path: impl AsRef<std::path::Path>) -> Result<u32, anyhow::Error> {
    let pipeline = gst::Pipeline::default();
    let src = gst::ElementFactory::make("filesrc")
        .name("src")
        .property("location", path.as_ref().to_str().unwrap())
        .build()?;

    let src_pad = src.static_pad("src").unwrap();
    src_pad.add_probe(
        gst::PadProbeType::EVENT_BOTH | gst::PadProbeType::EVENT_FLUSH,
        move |_, asdf| {
            if let Some(structure) = asdf.event().and_then(|e| e.structure()) {
                info!("Event received: {:?}", structure.name());
            }
            gst::PadProbeReturn::Ok
        },
    );

    let decode_bin = gst::ElementFactory::make("decodebin").build()?;

    let appsink = gst_app::AppSink::builder()
        .name("sink")
        .caps(&gst_video::VideoCapsBuilder::new().build())
        .sync(false)
        .build();

    let frame_count = std::sync::Arc::new(AtomicU32::new(0));
    let frame_count_clone = frame_count.clone();

    appsink.set_callbacks(
        gst_app::AppSinkCallbacks::builder()
            .new_sample(move |sink| {
                let _ = sink.pull_sample();
                frame_count_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

                Ok(gst::FlowSuccess::Ok)
            })
            .eos(|_| {
                info!("End of stream in appsink");
            })
            .build(),
    );

    pipeline.add_many([&src, &decode_bin, appsink.upcast_ref()])?;
    gst::Element::link_many([&src, &decode_bin])?;

    // let pipline_weak = gst::glib::clone::Downgrade::downgrade(&pipline);

    let has_run = std::sync::atomic::AtomicBool::new(false);

    decode_bin.connect_pad_added(move |_, src_pad| {
        let is_video = src_pad
            .current_caps()
            .and_then(|caps| {
                caps.structure(0)
                    .map(|s| s.name().starts_with("video/x-raw"))
            })
            .unwrap_or(false);

        if is_video
            && has_run
                .compare_exchange(
                    false,
                    true,
                    std::sync::atomic::Ordering::SeqCst,
                    std::sync::atomic::Ordering::SeqCst,
                )
                .is_ok()
        {
            let sink_pad = appsink.static_pad("sink").unwrap();
            src_pad.link(&sink_pad).unwrap();
        }
    });

    pipeline.set_state(gst::State::Playing)?;

    let bus = pipeline
        .bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                pipeline.set_state(gst::State::Null)?;
                return Err(anyhow::anyhow!(
                    "Error received from element {}: {}",
                    err.src()
                        .map(GstObjectExt::path_string)
                        .unwrap_or_else(|| "None".into()),
                    err.error()
                ));
            }
            _ => {}
        }
    }

    pipeline.set_state(gst::State::Null)?;

    Ok(frame_count.load(std::sync::atomic::Ordering::SeqCst))
}
