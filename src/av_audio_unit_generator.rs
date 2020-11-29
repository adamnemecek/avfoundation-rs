use crate::AVAudioNodeRef;

pub enum AVAudioUnitGeneratorFFI {}

foreign_obj_type! {
    type CType = AVAudioUnitGeneratorFFI;
    pub struct AVAudioUnitGenerator;
    pub struct AVAudioUnitGeneratorRef;
    type ParentType = AVAudioNodeRef;
}
