use crate::{
    AVAudioConnectionPointRef,
    AVAudioNodeBus,
    AVAudioNodeRef,
};
// use objc::{
//     runtime::Object,
//     Message,
// };

pub trait AVAudioStereoMixing: objc::Message + Sized {
    fn pan(&self) -> f32 {
        unsafe { msg_send![self, pan] }
    }
}

pub trait AVAudioMixing: AVAudioStereoMixing {
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

// impl<T: objc::Message> AVAudioMixing for T {
//     fn destination_for_mixer(
//         &self,
//         mixer: &AVAudioNodeRef,
//         bus: AVAudioNodeBus,
//     ) -> Option<&AVAudioMixingDestinationRef> {
//         unsafe {
//             let obj: *const AVAudioMixingDestinationRef =
//                 msg_send![self, destinationForMixer: mixer bus: bus];
//             obj.as_ref()
//         }
//     }
// }

pub enum AVAudioMixingDestinationFFI {}

foreign_obj_type! {
    type CType = AVAudioMixingDestinationFFI;
    pub struct AVAudioMixingDestination;
    pub struct AVAudioMixingDestinationRef;
    // type ParentType = AUParameterNodeRef;
}

impl AVAudioStereoMixing for AVAudioMixingDestinationRef {}

impl AVAudioMixing for AVAudioMixingDestinationRef {}

impl AVAudioMixingDestinationRef {
    pub fn connection_point(&self) -> &AVAudioConnectionPointRef {
        unsafe { msg_send![self, connectionPoint] }
    }
}
