use std::{collections::HashMap, path::PathBuf};

use anyhow::Error;

use gst::glib;
use gst_pbutils::{
    DiscovererContainerInfo, DiscovererInfo, DiscovererResult, DiscovererStreamInfo,
    DiscovererVideoInfo, prelude::*,
};

type Caps = HashMap<String, HashMap<String, String>>;
type TagList = HashMap<String, Vec<String>>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum MediaType {
    Audio,
    Video(VideoInfo),
    Subtitle,
    Container(ContainerInfo),
    Basic(BasicInfo),
}

impl MediaType {
    pub fn is_video(&self) -> bool {
        matches!(self, MediaType::Video(_))
    }

    pub fn is_audio(&self) -> bool {
        matches!(self, MediaType::Audio)
    }

    pub fn is_container(&self) -> bool {
        matches!(self, MediaType::Container(_))
    }

    pub fn as_video(&self) -> Option<&VideoInfo> {
        if let MediaType::Video(video_info) = self {
            Some(video_info)
        } else {
            None
        }
    }

    pub fn as_container(&self) -> Option<&ContainerInfo> {
        if let MediaType::Container(container_info) = self {
            Some(container_info)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct BasicInfo {
    pub name: String,
    pub caps: Caps,
    pub tags: TagList,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct ContainerInfo {
    pub name: String,
    pub caps: Caps,
    pub tags: TagList,
    pub streams: Vec<MediaType>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct FileInfo {
    pub path: PathBuf,
    pub seekable: bool,
    pub duration: std::time::Duration,
    pub tags: TagList,
    pub top_level_streams: Vec<MediaType>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct VideoInfo {
    pub name: String,
    pub caps: Caps,
    pub tags: TagList,
    pub width: u32,
    pub height: u32,
    pub bitrate: u32,
    pub depth: u32,
    pub max_bitrate: u32,
    pub is_image: bool,
    pub is_interlaced: bool,
    pub framerate: f64,
}

fn send_value_as_str(v: &glib::SendValue) -> Option<String> {
    if let Ok(s) = v.get::<&str>() {
        Some(s.to_string())
    } else if let Ok(serialized) = v.serialize() {
        Some(serialized.into())
    } else {
        None
    }
}

fn gst_video_info_native(info: &DiscovererVideoInfo) -> VideoInfo {
    VideoInfo {
        width: info.width(),
        height: info.height(),
        bitrate: info.bitrate(),
        depth: info.depth(),
        max_bitrate: info.max_bitrate(),
        is_image: info.is_image(),
        is_interlaced: info.is_interlaced(),
        framerate: {
            let (num, denom) = info.framerate().0.into_raw();
            num as f64 / denom as f64
        },
        ..Default::default()
    }
}

fn stream_info(info: &DiscovererStreamInfo) -> BasicInfo {
    let mut stream_info = BasicInfo {
        name: info.stream_type_nick().to_string(),
        ..Default::default()
    };

    if let Some(caps) = info.caps() {
        for structure in caps.iter() {
            let name = structure.name().to_string();
            let mut caps_name = HashMap::new();

            for (key, value) in structure.iter() {
                if let Some(s) = send_value_as_str(value) {
                    caps_name.insert(key.to_string(), s.clone());
                }
            }
            stream_info.caps.insert(name, caps_name);
        }
    }

    if let Some(tags) = info.tags() {
        for (tag, values) in tags.iter_generic() {
            let values = values.into_iter().filter_map(send_value_as_str).collect();
            stream_info.tags.insert(tag.to_string(), values);
        }
    };

    stream_info
}

fn handle_media_type(info: &DiscovererStreamInfo) -> MediaType {
    let basic_info = stream_info(info);
    if let Some(dis_container_info) = info.downcast_ref::<DiscovererContainerInfo>() {
        let mut container_info = ContainerInfo {
            name: basic_info.name,
            caps: basic_info.caps,
            tags: basic_info.tags,
            ..Default::default()
        };
        for stream in dis_container_info.streams() {
            let media_info = handle_media_type(&stream);
            container_info.streams.push(media_info);
        }
        return MediaType::Container(container_info);
    } else if let Some(video_info) = info.downcast_ref::<DiscovererVideoInfo>() {
        let mut video_info = gst_video_info_native(video_info);
        video_info.name = basic_info.name;
        video_info.caps = basic_info.caps;
        video_info.tags = basic_info.tags;
        return MediaType::Video(video_info);
    };
    MediaType::Basic(basic_info)
}

fn on_discovered(
    discoverer_info: &DiscovererInfo,
    error: Option<&glib::Error>,
) -> anyhow::Result<FileInfo> {
    let uri = discoverer_info.uri();
    match discoverer_info.result() {
        DiscovererResult::Ok => {}
        DiscovererResult::UriInvalid => return Err(anyhow::anyhow!("Invalid URI {uri}")),
        DiscovererResult::Error => {
            if let Some(msg) = error {
                return Err(anyhow::anyhow!(msg.message().to_string()));
            } else {
                return Err(anyhow::anyhow!("Unknown error"));
            }
        }
        DiscovererResult::Timeout => return Err(anyhow::anyhow!("Timeout")),
        DiscovererResult::Busy => return Err(anyhow::anyhow!("Busy")),
        DiscovererResult::MissingPlugins => {
            if let Some(s) = discoverer_info.misc() {
                return Err(anyhow::anyhow!("Missing plugins: {s}"));
            } else {
                return Err(anyhow::anyhow!("Missing plugins"));
            }
        }
        _ => return Err(anyhow::anyhow!("Unknown error")),
    }

    let mut file_info = FileInfo {
        path: PathBuf::from(uri),
        seekable: discoverer_info.is_seekable(),
        duration: discoverer_info
            .duration()
            .map(std::time::Duration::from)
            .unwrap_or_default(),
        ..Default::default()
    };

    if let Some(tags) = discoverer_info.tags() {
        for (tag, values) in tags.iter_generic() {
            let values = values.into_iter().filter_map(send_value_as_str).collect();
            file_info.tags.insert(tag.to_string(), values);
        }
    }

    if let Some(stream_info) = discoverer_info.stream_info() {
        let output = handle_media_type(&stream_info);
        file_info.top_level_streams.push(output);
        while let Some(info) = stream_info.next() {
            let output = handle_media_type(&info);
            file_info.top_level_streams.push(output);
        }
    };

    Ok(file_info)
}

fn run_discoverer(uri: &str) -> Result<FileInfo, Error> {
    let timeout = 5 * gst::ClockTime::SECOND;
    let discoverer = gst_pbutils::Discoverer::new(timeout)?;
    let info = discoverer.discover_uri(uri)?;

    let output = on_discovered(&info, None)?;

    Ok(output)
}

pub fn discover_file_info(path: impl AsRef<std::path::Path>) -> Result<FileInfo, Error> {
    let mut path = path.as_ref().to_path_buf();
    if path.exists() {
        if !path.is_absolute() {
            path = path.canonicalize()?;
        }

        #[cfg(target_os = "windows")]
        if !path.starts_with("/") {
            let path_str = path.strip_prefix(r"\\?\").unwrap_or(&path);
            path = PathBuf::from(format!("/{}", path_str.display()));
        }

        path = PathBuf::from(format!("file://{}", path.display()));
    }

    let info = run_discoverer(path.to_str().unwrap())?;

    Ok(info)
}
