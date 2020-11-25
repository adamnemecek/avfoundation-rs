use crate::AVAudioEngineRef;
use objc::runtime::{
    Object,
    NO,
    YES,
};

pub enum AVAudioSequencerFFI {}

foreign_obj_type! {
    type CType = AVAudioSequencerFFI;
    pub struct AVAudioSequencer;
    pub struct AVAudioSequencerRef;
}

impl AVAudioSequencer {
    pub fn with_engine(engine: &AVAudioEngineRef) -> Self {
        unsafe {
            let class = class!(AVAudioSequencer);
            let alloc: *const Object = msg_send![class, alloc];
            msg_send![alloc, initWithAudioEngine: engine]
        }
    }
}

impl AVAudioSequencerRef {
    pub fn is_playing(&self) -> bool {
        unsafe {
            match msg_send![self, isPlaying] {
                YES => true,
                NO => false,
                _ => unreachable!(),
            }
        }
    }
}

pub enum AVMusicTrackFFI {}

foreign_obj_type! {
    type CType = AVMusicTrackFFI;
    pub struct AVMusicTrack;
    pub struct AVMusicTrackRef;
}
