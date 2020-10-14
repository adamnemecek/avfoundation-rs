use crate::{AudioFormat, AudioNodeBus};

pub enum AVAudioNode {}

foreign_obj_type! {
    type CType = AVAudioNode;
    pub struct AudioNode;
    pub struct AudioNodeRef;
}

impl AudioNode {
    pub fn new() -> Self {
        unsafe {
            let class = class!(AVAudioNode);
            msg_send![class, new]
        }
    }
}

impl AudioNodeRef {
    pub fn reset(&self) {
        unsafe { msg_send![self, reset] }
    }

    pub fn input_format(&self, bus: AudioNodeBus) -> AudioFormat {
        unsafe { msg_send![self, inputFormat: bus.inner] }
    }

    // pub fn output_format(&self, forBus bus: AudioNodeBus) -> AVAudioFormat {
    //    unsafe {
    //      msg_send![self, outputFormat]
    //}
    //}

    // pub fn name(&self, forInputBus bus: AudioNodeBus) -> String? {
    //    unsafe {
    //      msg_send![self, name]
    //}
    //}

    // pub fn name(&self, forOutputBus bus: AudioNodeBus) -> String? {
    //    unsafe {
    //      msg_send![self, name]
    //}
    //}

    // pub fn install_tap(&self, onBus bus: AudioNodeBus, bufferSize: AVAudioFrameCount, format: AVAudioFormat?, block tapBlock: @escaping AVAudioNodeTapBlock) {
    //    unsafe {
    //      msg_send![self, installTap]
    //}
    //}

    pub fn remove_tap(&self, bus: AudioNodeBus) {
        unsafe { msg_send![self, removeTap: bus.inner] }
    }

    pub fn number_of_inputs(&self) -> i64 {
        unsafe { msg_send![self, numberOfOutputs] }
    }
}
