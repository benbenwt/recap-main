use std::collections::HashMap;

use iced::widget;
use video_annotation_proto::video_annotation::VideoAnnotationMetadata;
use window_handling::{MonitorInfo as _, WindowInfo};

use crate::{Message, SavedState, input_manager::keyboard::keyboard_layout};

// Version now configured in Cargo.toml [package.metadata.versions] section
pub const RECAP_VERSION: &str = env!("RECAP_VERSION");
pub const GIT_COMMIT: &str = env!("GIT_COMMIT");

pub(crate) fn meta_data_from_state(
    saved_state: &SavedState,
) -> Result<VideoAnnotationMetadata, anyhow::Error> {
    // Build custom_metadata map from all entries (filter out empty keys)
    let custom_metadata: HashMap<String, String> = saved_state
        .metadata_entries
        .iter()
        .filter(|(k, _)| !k.trim().is_empty())
        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        .collect();

    let meta_data = VideoAnnotationMetadata {
        custom_metadata,
        ..Default::default()
    };
    Ok(meta_data)
}

pub fn capture_device_specs_from_target(
    target: &dyn WindowInfo,
) -> Result<video_annotation_proto::video_annotation::CaptureDeviceSpecs, anyhow::Error> {
    let (width, height) = target.size()?;
    let monitor = target.current_monitor()?;
    let monitor_id = monitor.id() as i64;
    let monitor_size = monitor.size()?;
    let capture_device_specs = video_annotation_proto::video_annotation::CaptureDeviceSpecs {
        recap_version: RECAP_VERSION.parse().unwrap(),
        git_commit: GIT_COMMIT.to_string(),
        os: std::env::consts::OS.to_string(),
        keyboard_layout: keyboard_layout()?.to_string(),
        window_specs: Some(video_annotation_proto::video_annotation::WindowSpecs {
            width,
            height,
            scale: target.scale_factor(),
            dpi: target.dpi()? as i32,
            title: target.title().unwrap_or_default(),
            screen_id: monitor_id,
        }),
        screen_specs: vec![video_annotation_proto::video_annotation::ScreenSpecs {
            width: monitor_size.0,
            height: monitor_size.1,
            scale: monitor.scale_factor(),
            dpi: monitor.dpi()? as i32,
            id: monitor_id,
        }],
    };
    Ok(capture_device_specs)
}

pub(crate) fn set_meta_data(state: &crate::App) -> iced::Element<'_, Message> {
    let mut rows: Vec<iced::Element<'_, Message>> = vec![];

    // Render each key-value pair
    for (key, value) in state.saved_state.metadata_entries.iter() {
        let current_key = key.clone();
        let key_for_value = key.clone();
        let key_for_remove = key.clone();

        let key_input = widget::text_input("key", key)
            .on_input(move |new_key| Message::RenameMetadataKey {
                old_key: current_key.clone(),
                new_key,
            })
            .width(100.0);

        let value_input = widget::text_input("value", value)
            .on_input(move |v| Message::SetMetadataValue {
                key: key_for_value.clone(),
                value: v,
            })
            .width(150.0);

        let remove_button = widget::button(widget::text("-"))
            .on_press(Message::RemoveMetadataEntry(key_for_remove))
            .padding(5);

        rows.push(
            widget::row![key_input, widget::text(":"), value_input, remove_button,]
                .spacing(5)
                .into(),
        );
    }

    // Add button for new entry
    rows.push(
        widget::button(widget::text("+ Add Field"))
            .on_press(Message::AddMetadataEntry)
            .padding(5)
            .style(iced::widget::button::secondary)
            .into(),
    );

    // Save button
    rows.push(
        widget::button("Save settings")
            .on_press(Message::SaveSettings)
            .padding(10)
            .style(iced::widget::button::primary)
            .into(),
    );

    widget::column(rows).spacing(8).into()
}
