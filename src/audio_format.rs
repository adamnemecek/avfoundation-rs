use objc::runtime::{NO, YES};
pub enum AudioCommonFormat {
    OtherFormat,
    PcmFormatFloat32,
    PcmFormatFloat64,
    PcmFormatInt16,
    PcmFormatInt32,
}

pub enum AVAudioFormat {}

foreign_obj_type! {
    type CType = AVAudioFormat;
    pub struct AudioFormat;
    pub struct AudioFormatRef;
}


impl AudioFormatRef {
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
        unsafe {
            msg_send![self, commonFormat]
        }
    }

    pub fn channel_count(&self) -> u32 {
        unsafe {
            msg_send![self, channelCount]
        }
    }

    // pub fn sample_rate: Double { get } {
    //     unsafe {
    //         msg_send![self, sampleRate];
    //     }
    // }

    // pub fn is_interleaved: Bool { get } {
    //     unsafe {
    //         msg_send![self, isInterleaved];
    //     }
    // }

    // pub fn stream_description: UnsafePointer<AudioStreamBasicDescription> { get } {
    //     unsafe {
    //         msg_send![self, streamDescription];
    //     }
    // }

    // pub fn channel_layout: AVAudioChannelLayout? { get } {
    //     unsafe {
    //         msg_send![self, channelLayout];
    //     }
    // }

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