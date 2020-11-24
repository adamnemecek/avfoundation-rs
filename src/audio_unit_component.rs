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
}
