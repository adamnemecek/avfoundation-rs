use crate::{
    AVAudioNodeBus,
    AVAudioNodeRef,
};
pub enum AVAudioMixerNodeFFI {}

foreign_obj_type! {
    type CType = AVAudioMixerNodeFFI;
    pub struct AVAudioMixerNode;
    pub struct AVAudioMixerNodeRef;
    type ParentType = AVAudioNodeRef;
}

impl AVAudioMixerNode {
    // - (instancetype)init NS_DESIGNATED_INITIALIZER;
    pub fn new() -> Self {
        unsafe {
            let class = class!(AVAudioMixerNode);
            msg_send![class, new]
        }
    }
}

impl AVAudioMixerNodeRef {
    /*! @property outputVolume
        @abstract The mixer's output volume.
        @discussion
            This accesses the mixer's output volume (0.0-1.0, inclusive).
    */
    // @property (nonatomic) float outputVolume;
    pub fn output_volume(&self) -> f32 {
        unsafe { msg_send![self, outputVolume] }
    }

    // /*! @property nextAvailableInputBus
    //     @abstract Find an unused input bus.
    //     @discussion
    //         This will find and return the first input bus to which no other node is connected.
    // */
    // @property (nonatomic, readonly) AVAudioNodeBus nextAvailableInputBus;
    pub fn next_available_input_bus(&self) -> AVAudioNodeBus {
        unsafe { msg_send![self, nextAvailableInputBus] }
    }
}
