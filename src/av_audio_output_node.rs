use crate::AVAudioIONodeRef;

pub enum AVAudioOutputNodeFFI {}

foreign_obj_type! {
    type CType = AVAudioOutputNodeFFI;
    pub struct AVAudioOutputNode;
    pub struct AVAudioOutputNodeRef;
    type ParentType = AVAudioIONodeRef;
}

impl AVAudioOutputNodeRef {}
