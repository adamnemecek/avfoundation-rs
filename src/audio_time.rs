// use crate::{AudioFormat, AudioNodeBus};
use objc::runtime::{NO, YES};
pub enum AVAudioTime {}

foreign_obj_type! {
    type CType = AVAudioTime;
    pub struct AudioTime;
    pub struct AudioTimeRef;
}

impl AudioTimeRef {
    pub fn is_host_time_valid(&self) -> bool {
        unsafe {
            match msg_send![self, isHostTimeValid] {
                YES => true,
                NO => false,
                _ => unreachable!(),
            }
        }
    }

    pub fn host_time(&self) -> u64 {
        unsafe { msg_send![self, hostTime] }
    }

    pub fn is_sample_time_valid(&self) -> bool {
        unsafe {
            match msg_send![self, isSampleTimeValid] {
                YES => true,
                NO => false,
                _ => unreachable!(),
            }
        }
    }

    // pub fn sampleTime(&self) -> AVAudioFramePosition {
    //     unsafe { msg_send![self, sampleTime] }
    //  }

    pub fn sample_rate(&self) -> f64 {
        unsafe { msg_send![self, sampleRate] }
    }

    // pub fn audioTimeStamp(&self) -> AudioTimeStamp {
    //     unsafe { msg_send![self, audioTimeStamp] }
    //  }

}
