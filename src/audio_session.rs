#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AVAudioSessionRecordPermission {
    Undetermined,
    Denied,
    Granted,
}

pub enum AVAudioSessionFFI {}

foreign_obj_type! {
    type CType = AVAudioSessionFFI;
    pub struct AVAudioSession;
    pub struct AVAudioSessionRef;
}

impl AVAudioSession {
    pub fn shared() -> Self {
        unsafe {
            let class = class!(AVAudioSession);
            msg_send![class, sharedInstance]
        }
    }
}
