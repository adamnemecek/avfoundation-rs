pub enum AVAudioChannelLayoutFFI {}

foreign_obj_type! {
    type CType = AVAudioChannelLayoutFFI;
    pub struct AVAudioChannelLayout;
    pub struct AVAudioChannelLayoutRef;
}

impl AVAudioChannelLayoutRef {}
