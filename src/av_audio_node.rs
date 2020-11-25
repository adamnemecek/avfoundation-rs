use crate::{
    AVAudioFormat,
    AVAudioTimeRef,
    AudioNodeBus,
};

pub enum AVAudioNodeFFI {}

foreign_obj_type! {
    type CType = AVAudioNodeFFI;
    pub struct AVAudioNode;
    pub struct AVAudioNodeRef;
}

impl AVAudioNode {
    pub fn new() -> Self {
        unsafe {
            let class = class!(AVAudioNode);
            msg_send![class, new]
        }
    }
}

impl AVAudioNodeRef {
    pub fn reset(&self) {
        unsafe { msg_send![self, reset] }
    }

    pub fn input_format_for_bus(&self, bus: AudioNodeBus) -> AVAudioFormat {
        unsafe { msg_send![self, inputFormatForBus: bus.inner] }
    }

    // pub fn output_format(&self, forBus bus: AudioNodeBus) -> AVAudioFormat {
    //    unsafe {
    //      msg_send![self, outputFormat]
    //}
    //}

    pub fn name_for_input_bus(&self, input_bus: AudioNodeBus) -> &str {
        unsafe {
            let name = msg_send![self, nameForInputBus: input_bus.inner];
            crate::nsstring_as_str(name)
        }
    }

    pub fn name_for_output_bus(&self, output_bus: AudioNodeBus) -> &str {
        unsafe {
            let name = msg_send![self, nameForOutputBus: output_bus.inner];
            crate::nsstring_as_str(name)
        }
    }

    // pub fn install_tap(&self, onBus bus: AudioNodeBus, bufferSize: AVAudioFrameCount, format: AVAudioFormat?, block tapBlock: @escaping AVAudioNodeTapBlock) {
    //    unsafe {
    //      msg_send![self, installTap]
    //}
    //}

    pub fn remove_tap(&self, bus: AudioNodeBus) {
        unsafe { msg_send![self, removeTap: bus.inner] }
    }

    pub fn number_of_inputs(&self) -> i64 {
        unsafe { msg_send![self, numberOfInputs] }
    }

    pub fn number_of_outputs(&self) -> i64 {
        unsafe { msg_send![self, numberOfOutputs] }
    }

    pub fn last_render_time(&self) -> &AVAudioTimeRef {
        unsafe { msg_send![self, lastRenderTime] }
    }

    pub fn latency(&self) -> f64 {
        unsafe { msg_send![self, latency] }
    }
}
