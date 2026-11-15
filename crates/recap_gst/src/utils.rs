use gst::{glib::WeakRef, prelude::ElementExtManual};

#[derive(Debug)]
pub struct SendEos {
    pub pipeline: WeakRef<gst::Pipeline>,
}

impl SendEos {
    pub fn end(&self) -> anyhow::Result<()> {
        let pipeline = self
            .pipeline
            .upgrade()
            .ok_or(anyhow::anyhow!("Pipeline is not available"))?;

        pipeline.send_event(gst::event::Eos::new());

        Ok(())
    }
}
