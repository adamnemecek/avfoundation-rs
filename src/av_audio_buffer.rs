use crate::{AVAudioFormat};
pub enum AVAudioBufferFFI {}

foreign_obj_type! {
    type CType = AVAudioBufferFFI;
    pub struct AVAudioBuffer;
    pub struct AVAudioBufferRef;
}

impl AVAudioBufferRef {
    pub fn format(&self) -> AVAudioFormat {
        unsafe { msg_send![self, format] }
    }
    // audiobuferlist
    // mutableaudiobufferlist
}


pub enum AVAudioPCMBufferFFI {}

foreign_obj_type! {
    type CType = AVAudioPCMBufferFFI;
    pub struct AVAudioPCMBuffer;
    pub struct AVAudioPCMBufferRef;
    type ParentType = AVAudioBufferRef;
}


pub enum AVAudioCompressedBufferFFI {}

foreign_obj_type! {
    type CType = AVAudioCompressedBufferFFI;
    pub struct AVAudioCompressedBuffer;
    pub struct AVAudioCompressedBufferRef;
    type ParentType = AVAudioBufferRef;
}

