pub enum AVAudioConnectionPointFFI {}

foreign_obj_type! {
    type CType = AVAudioConnectionPointFFI;
    pub struct AVAudioConnectionPoint;
    pub struct AVAudioConnectionPointRef;
    // type ParentType = AUParameterNodeRef;
}
