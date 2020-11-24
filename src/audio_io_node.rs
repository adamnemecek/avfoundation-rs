use crate::AVAudioNodeRef;

pub enum AVAudioIONodeFFI {}

foreign_obj_type! {
    type CType = AVAudioIONodeFFI;
    pub struct AVAudioIONode;
    pub struct AVAudioIONodeRef;
    type ParentType = AVAudioNodeRef;
}
