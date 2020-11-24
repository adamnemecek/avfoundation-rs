use crate::AVAudioIONodeRef;

pub enum AVAudioOutputNodeNative {}

foreign_obj_type! {
    type CType = AVAudioOutputNodeNative;
    pub struct AVAudioOutputNode;
    pub struct AVAudioOutputNodeRef;
    type ParentType = AVAudioIONodeRef;
}

impl AVAudioOutputNodeRef {}
