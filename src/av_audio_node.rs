use crate::{
    AUAudioUnit,
    AVAudioFormat,
    AVAudioFrameCount,
    AVAudioNodeBus,
    AVAudioPCMBufferRef,
    AVAudioTimeRef,
};

//	@typedef AVAudioNodeTapBlock
//	@abstract A block that receives copies of the output of an AVAudioNode.
//	@param buffer
//		a buffer of audio captured from the output of an AVAudioNode
//	@param when
//		the time at which the buffer was captured
//	@discussion
//		CAUTION: This callback may be invoked on a thread other than the main thread.

pub type AVAudioNodeTapBlock<'a> = block::Block<(&'a AVAudioPCMBufferRef, &'a AVAudioTimeRef), ()>;
pub type AVAudioNodeTapFn<'a> =
    std::rc::Rc<dyn Fn(&'a AVAudioPCMBufferRef, &'a AVAudioTimeRef) -> ()>;

pub enum AVAudioNodeFFI {}

// @class AVAudioNode
// @abstract Base class for an audio generation, processing, or I/O block.
// @discussion
//     `AVAudioEngine` objects contain instances of various AVAudioNode subclasses. This
//     base class provides certain common functionality.
//
//     Nodes have input and output busses, which can be thought of as connection points.
//     For example, an effect typically has one input bus and one output bus. A mixer
//     typically has multiple input busses and one output bus.
//
//     Busses have formats, expressed in terms of sample rate and channel count. When making
//     connections between nodes, often the format must match exactly. There are exceptions
//     (e.g. `AVAudioMixerNode` and `AVAudioOutputNode`).
//
//     Nodes do not currently provide useful functionality until attached to an engine.
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

    pub fn input_format_for_bus(&self, bus: AVAudioNodeBus) -> AVAudioFormat {
        unsafe { msg_send![self, inputFormatForBus: bus.inner] }
    }

    // pub fn output_format(&self, forBus bus: AudioNodeBus) -> AVAudioFormat {
    //    unsafe {
    //      msg_send![self, outputFormat]
    //}
    //}

    pub fn name_for_input_bus(&self, input_bus: AVAudioNodeBus) -> &str {
        unsafe {
            let name = msg_send![self, nameForInputBus: input_bus.inner];
            crate::nsstring_as_str(name)
        }
    }

    pub fn name_for_output_bus(&self, output_bus: AVAudioNodeBus) -> &str {
        unsafe {
            let name = msg_send![self, nameForOutputBus: output_bus.inner];
            crate::nsstring_as_str(name)
        }
    }

    pub fn install_tap<F>(
        &self,
        bus: AVAudioNodeBus,
        buffer_size: AVAudioFrameCount,
        format: Option<AVAudioFormat>,
        // block: AVAudioNodeTapBlock,
        f: F,
    ) where
        F: FnMut(&AVAudioPCMBufferRef, &AVAudioTimeRef) -> (),
    {
        // let block = block::Block
        todo!()
        //    unsafe {
        //      msg_send![self, installTap]
        //}
    }

    pub fn remove_tap(&self, bus: AVAudioNodeBus) {
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

    pub fn au_audio_unit(&self) -> &AUAudioUnit {
        unsafe { msg_send![self, AUAudioUnit] }
    }

    pub fn latency(&self) -> f64 {
        unsafe { msg_send![self, latency] }
    }
}
