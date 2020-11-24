use crate::AVAudioNodeRef;
pub enum AVAudioUnitNative {}

foreign_obj_type! {
    type CType = AVAudioUnitNative;
    pub struct AVAudioUnit;
    pub struct AVAudioUnitRef;
    type ParentType = AVAudioNodeRef;
}

impl AVAudioUnitRef {
    // pub fn audio_component_description(&self) -> AudioComponentDescription {
    //  unsafe { msg_send![self, audioComponentDescription] }
    // }

    // pub fn audio_unit(&self) -> AudioUnit {
    //  unsafe { msg_send![self, audioUnit] }
    // }

    // pub fn au_audio_unit(&self) -> AUAudioUnit {
    //  unsafe { msg_send![self, auAudioUnit] }
    // }

    pub fn name(&self) -> &str {
        unsafe {
            let name = msg_send![self, name];
            crate::nsstring_as_str(name)
        }
    }

    pub fn manufacturer_name(&self) -> &str {
        unsafe {
            let name = msg_send![self, manufacturerName];
            crate::nsstring_as_str(name)
        }
    }

    pub fn version(&self) -> i64 {
        unsafe { msg_send![self, version] }
    }
}
