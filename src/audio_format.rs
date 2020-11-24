use crate::AudioChannelLayoutRef;
use objc::runtime::{NO, YES};
pub enum AudioCommonFormat {
    OtherFormat,
    PcmFormatFloat32,
    PcmFormatFloat64,
    PcmFormatInt16,
    PcmFormatInt32,
}

pub enum AVAudioFormatFFI {}

foreign_obj_type! {
    type CType = AVAudioFormatFFI;
    pub struct AVAudioFormat;
    pub struct AVAudioFormatRef;
}

impl AVAudioFormatRef {
    pub fn is_standard(&self) -> bool {
        unsafe {
            match msg_send![self, isStandard] {
                YES => true,
                NO => false,
                _ => unreachable!(),
            }
        }
    }

    pub fn common_format(&self) -> AudioCommonFormat {
        unsafe { msg_send![self, commonFormat] }
    }

    pub fn channel_count(&self) -> u32 {
        unsafe { msg_send![self, channelCount] }
    }

    pub fn sample_rate(&self) -> f64 {
        unsafe { msg_send![self, sampleRate] }
    }

    pub fn is_interleaved(&self) -> bool {
        unsafe {
            match msg_send![self, isInterleaved] {
                YES => true,
                NO => false,
                _ => unreachable!(),
            }
        }
    }

    // pub fn stream_description: UnsafePointer<AudioStreamBasicDescription> { get } {
    //     unsafe {
    //         msg_send![self, streamDescription];
    //     }
    // }

    pub fn channel_layout(&self) -> Option<&AudioChannelLayoutRef> {
        unsafe { msg_send![self, channelLayout] }
    }

    // pub fn magic_cookie: Data? {
    //     unsafe {
    //         msg_send![self, magicCookie];
    //     }
    // }

    // pub fn settings: [String : Any] { get } {
    //     unsafe {
    //         msg_send![self, settings];
    //     }
    // }

    // pub fn format_description: CMAudioFormatDescription { get } {
    //     unsafe {
    //         msg_send![self, formatDescription];
    //     }
    // }
}
