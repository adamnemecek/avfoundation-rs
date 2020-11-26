use crate::{
    AVAudioNodeBus,
    AVAudioNodeRef,
};
use objc::{
    runtime::Object,
    Message,
};
pub trait AVAudioMixing {
    fn destination_for_mixer(
        &self,
        mixer: &AVAudioNodeRef,
        bus: AVAudioNodeBus,
    ) -> Option<&AVAudioMixingDestinationRef>;
}

impl<T: objc::Message> AVAudioMixing for T {
    fn destination_for_mixer(
        &self,
        mixer: &AVAudioNodeRef,
        bus: AVAudioNodeBus,
    ) -> Option<&AVAudioMixingDestinationRef> {
        unsafe {
            let obj: *const AVAudioMixingDestinationRef =
                msg_send![self, destinationForMixer: mixer bus: bus];
            obj.as_ref()
        }
    }
}

pub enum AVAudioMixingDestinationFFI {}

foreign_obj_type! {
    type CType = AVAudioMixingDestinationFFI;
    pub struct AVAudioMixingDestination;
    pub struct AVAudioMixingDestinationRef;
    // type ParentType = AUParameterNodeRef;
}
