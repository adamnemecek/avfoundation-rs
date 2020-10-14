
use crate::{AudioNodeRef};

pub enum AVAudioIONode {}

foreign_obj_type! {
    type CType = AVAudioIONode;
    pub struct AudioIONode;
    pub struct AudioIONodeRef;
    type ParentType = AudioNodeRef;
}
