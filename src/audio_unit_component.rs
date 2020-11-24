pub struct AudioComponentDescription {
    pub component_type: u32,
    pub component_sub_type: u32,
    pub component_manufacturer: u32,
    pub component_flags: u32,
    pub component_flags_mask: u32,
}

pub enum AVAudioUnitComponentNative {}

foreign_obj_type! {
    type CType = AVAudioUnitComponentNative;
    pub struct AVAudioUnitComponent;
    pub struct AVAudioUnitComponentRef;
}

impl AVAudioUnitComponentRef {
    // fn audio_component(&self) -> AudioCOmpo

    pub fn name(&self) -> &str {
        unsafe {
            let s = msg_send![self, name];
            crate::nsstring_as_str(s)
        }
    }

    pub fn manufacturer_name(&self) -> &str {
        unsafe {
            let s = msg_send![self, manufacturerName];
            crate::nsstring_as_str(s)
        }
    }
}
