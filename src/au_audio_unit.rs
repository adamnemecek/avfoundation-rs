use crate::AudioComponentDescription;
pub enum AUAudioUnitFFI {}

foreign_obj_type! {
    type CType = AUAudioUnitFFI;
    pub struct AUAudioUnit;
    pub struct AUAudioUnitRef;
}

impl AUAudioUnitRef {
    pub fn component_description(&self) -> AudioComponentDescription {
        unsafe { msg_send![self, componentDescription] }
    }
}
