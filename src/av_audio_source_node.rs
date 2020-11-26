use crate::AVAudioNodeRef;
pub enum AVAudioSourceNodeFFI {}

foreign_obj_type! {
    type CType = AVAudioSourceNodeFFI;
    pub struct AVAudioSourceNode;
    pub struct AVAudioSourceNodeRef;
    type ParentType = AVAudioNodeRef;
}
