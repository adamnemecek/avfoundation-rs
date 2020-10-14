use crate::AudioIONodeRef;

pub enum AVAudioOutputNode {}

foreign_obj_type! {
    type CType = AVAudioOutputNode;
    pub struct AudioOutputNode;
    pub struct AudioOutputNodeRef;
    type ParentType = AudioIONodeRef;
}

impl AudioOutputNodeRef {}
