use crate::AudioTimeStamp;

pub struct OSStatus {}
pub struct AudioBufferList {}

// pub type OSStatus = u32;

pub struct AudioBufferListRef {}
use crate::{
    AVAudioFrameCount,
    AVAudioNodeRef,
};
pub enum AVAudioSinkNodeFFI {}

// typedef OSStatus (^AVAudioSinkNodeReceiverBlock)(const AudioTimeStamp *timestamp, AVAudioFrameCount frameCount, const AudioBufferList *inputData) API_AVAILABLE(macos(10.15), ios(13.0), tvos(13.0), watchos(6.0)) ;
pub type AVAudioSinkNodeReceiverBlock =
    block::Block<(*const AudioTimeStamp, AVAudioFrameCount, AudioBufferList), OSStatus>;

foreign_obj_type! {
    type CType = AVAudioSinkNodeFFI;
    pub struct AVAudioSinkNode;
    pub struct AVAudioSinkNodeRef;
    type ParentType = AVAudioNodeRef;
}

impl AVAudioSinkNodeRef {}
