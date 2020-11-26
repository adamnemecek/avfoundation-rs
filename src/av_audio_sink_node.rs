use crate::AVAudioNodeRef;
pub enum AVAudioSinkNodeFFI {}

foreign_obj_type! {
    type CType = AVAudioSinkNodeFFI;
    pub struct AVAudioSinkNode;
    pub struct AVAudioSinkNodeRef;
    type ParentType = AVAudioNodeRef;
}

impl AVAudioSinkNodeRef {}
