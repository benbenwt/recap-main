mod proto {
    tonic::include_proto!("mod");
}

pub mod video_inference {
    pub use super::proto::elefant::training_data::shared::*;
    pub use super::proto::elefant::training_data::video_inference::*;
}

pub use prost;
pub use tonic;

impl From<glam::IVec2> for video_inference::Vec2Int {
    fn from(point: glam::IVec2) -> Self {
        video_inference::Vec2Int {
            x: point.x,
            y: point.y,
        }
    }
}

impl From<video_inference::Vec2Int> for glam::IVec2 {
    fn from(point: video_inference::Vec2Int) -> Self {
        glam::IVec2::new(point.x, point.y)
    }
}

impl From<glam::DVec2> for video_inference::Vec2Float {
    fn from(point: glam::DVec2) -> Self {
        video_inference::Vec2Float {
            x: point.x,
            y: point.y,
        }
    }
}

impl From<video_inference::Vec2Float> for glam::DVec2 {
    fn from(point: video_inference::Vec2Float) -> Self {
        glam::DVec2::new(point.x, point.y)
    }
}
