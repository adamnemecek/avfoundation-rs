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
    pub fn current_position_in_seconds(&self) -> f64 {
        unsafe { msg_send![self, currentPositionInSeconds] }
    }

    pub fn rate(&self) -> f32 {
        unsafe { msg_send![self, rate] }
    }

    pub fn is_playing(&self) -> bool {
        unsafe {
            match msg_send![self, isPlaying] {
                YES => true,
                NO => false,
                _ => unreachable!(),
            }
        }
    }

    pub fn prepare_to_play(&self) {
        unsafe {
            let _: () = msg_send![self, prepareToPlay];
        }
    }

    pub fn stop(&self) {
        unsafe {
            let _: () = msg_send![self, stop];
        }
    }

    pub fn tempo_track(&self) -> Option<&AVMusicTrackRef> {
        unsafe {
            let ret: *const AVMusicTrackRef = msg_send![self, tempoTrack];
            ret.as_ref()
        }
    }

    pub fn tracks(&self) -> Vec<AVMusicTrack> {
        unsafe {
            let array: *const Object = msg_send![self, tracks];
            crate::nsarray_to_vec(array)
        }
    }
}

pub enum AVMusicTrackFFI {}

foreign_obj_type! {
    type CType = AVMusicTrackFFI;
    pub struct AVMusicTrack;
    pub struct AVMusicTrackRef;
}
