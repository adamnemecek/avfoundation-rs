use crate::AudioNodeRef;
pub enum AVAudioUnit {}

foreign_obj_type! {
    type CType = AVAudioUnit;
    pub struct AudioUnit;
    pub struct AudioUnitRef;
    type ParentType = AudioNodeRef;
}

impl AudioUnitRef {
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
