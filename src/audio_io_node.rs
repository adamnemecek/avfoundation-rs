use crate::AVAudioNodeRef;

pub enum AVAudioIONodeNative {}

foreign_obj_type! {
    type CType = AVAudioIONodeNative;
    pub struct AVAudioIONode;
    pub struct AVAudioIONodeRef;
    type ParentType = AVAudioNodeRef;
}
