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
