use crate::AVAudioNodeRef;

pub enum AVAudioIONodeFFI {}

foreign_obj_type! {
    type CType = AVAudioIONodeFFI;
    pub struct AVAudioIONode;
    pub struct AVAudioIONodeRef;
    type ParentType = AVAudioNodeRef;
}




pub enum AVAudioOutputNodeFFI {}

foreign_obj_type! {
    type CType = AVAudioOutputNodeFFI;
    pub struct AVAudioOutputNode;
    pub struct AVAudioOutputNodeRef;
    type ParentType = AVAudioIONodeRef;
}

impl AVAudioOutputNodeRef {
    
}
