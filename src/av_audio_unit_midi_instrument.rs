use crate::AVAudioNodeRef;

pub enum AVAudioUnitMIDIInstrumentFFI {}

foreign_obj_type! {
    type CType = AVAudioUnitMIDIInstrumentFFI;
    pub struct AVAudioUnitMIDIInstrument;
    pub struct AVAudioUnitMIDIInstrumentRef;
    type ParentType = AVAudioNodeRef;
}

// impl MIXING for AVAudioUnitMIDIInstrumentRef {}
