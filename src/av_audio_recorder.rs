use objc::runtime::{
    NO,
    YES,
};

pub trait AVAudioRecorderDelegate: objc::Message + Sized {
    // audioRecorderdidfinishrecording
}

pub enum AVAudioRecorderFFI {}

foreign_obj_type! {
    type CType = AVAudioRecorderFFI;
    pub struct AVAudioRecorder;
    pub struct AVAudioRecorderRef;
}

impl AVAudioRecorder {}

impl AVAudioRecorderRef {
    pub fn prepare_to_record(&self) -> bool {
        unsafe {
            match msg_send![self, prepareToRecord] {
                YES => true,
                NO => false,
                _ => unreachable!(),
            }
        }
    }

    pub fn record(&self) -> bool {
        unsafe {
            match msg_send![self, record] {
                YES => true,
                NO => false,
                _ => unreachable!(),
            }
        }
    }

    // recordattime
    // recordforduration
    // recordattime

    pub fn pause(&self) {
        unsafe { msg_send![self, pause] }
    }

    pub fn stop(&self) {
        unsafe { msg_send![self, stop] }
    }

    pub fn delete_recording(&self) -> bool {
        unsafe {
            match msg_send![self, deleteRecording] {
                YES => true,
                NO => false,
                _ => unreachable!(),
            }
        }
    }

    pub fn is_recoring(&self) -> bool {
        unsafe {
            match msg_send![self, isRecording] {
                YES => true,
                NO => false,
                _ => unreachable!(),
            }
        }
    }

    // url
    // settings
    // format
    // delegate
    // currenttime
    // devicecurrenttime
    // meteringenabled
    // peakpowerforchannel
    // averagepowerforchannel
    // channelassignments
}

#[cfg(target_os = "ios")]
impl AVAudioRecorderRef {}
