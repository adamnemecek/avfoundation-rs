// use crate::{AudioFormat, AudioNodeBus};

pub enum AVAudioTime {}

foreign_obj_type! {
    type CType = AVAudioTime;
    pub struct AudioTime;
    pub struct AudioTimeRef;
}

impl AudioTimeRef {}
