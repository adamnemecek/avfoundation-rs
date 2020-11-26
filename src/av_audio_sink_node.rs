use crate::AVAudioNodeRef;
pub enum AVAudioSinkNodeFFT { }

foreign_obj_type! {
    type CType = AVAudioSinkNodeFFT;
    pub struct AVAudioSinkNode;
    pub struct AVAudioSinkNodeRef;
    type ParentType = AVAudioNodeRef;
}