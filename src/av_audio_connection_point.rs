use crate::{
    AVAudioNodeBus,
    AVAudioNodeRef,
};

pub enum AVAudioConnectionPointFFI {}

foreign_obj_type! {
    type CType = AVAudioConnectionPointFFI;
    pub struct AVAudioConnectionPoint;
    pub struct AVAudioConnectionPointRef;
}

impl AVAudioConnectionPoint {
    pub fn with_node(node: &AVAudioNodeRef, bus: AVAudioNodeBus) -> Self {
        unsafe {
            let class = class!(AVAudioConnectionPoint);
            let alloc: *const AVAudioConnectionPointFFI = msg_send![class, alloc];
            msg_send![alloc, initWithNode: node bus: bus]
        }
    }
}

impl AVAudioConnectionPointRef {
    pub fn node(&self) -> &AVAudioNodeRef {
        unsafe { msg_send![self, node] }
    }

    pub fn bus(&self) -> AVAudioNodeBus {
        unsafe { msg_send![self, bus] }
    }
}
