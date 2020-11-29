

pub enum AVAudioFileFFI {}

foreign_obj_type! {
    type CType = AVAudioFileFFI;
    pub struct AVAudioFile;
    pub struct AVAudioFileRef;
}

impl AVAudioFileRef {
    // pub fn url(&self) -> id {
        // todo!()
    // }
}