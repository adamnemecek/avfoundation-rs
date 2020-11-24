pub enum AVAudioUnitComponentManagerNative {}

foreign_obj_type! {
    type CType = AVAudioUnitComponentManagerNative;
    pub struct AVAudioUnitComponentManager;
    pub struct AVAudioUnitComponentManagerRef;
}

impl AVAudioUnitComponentManager {
    pub fn shared() -> Self {
        unsafe {
            let class = class!(AVAudioUnitComponentManager);

            msg_send![class, shared]
        }
    }
}
