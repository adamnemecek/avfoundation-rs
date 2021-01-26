use crate::{
    AUAudioUnitRef,
    AVAudioEngineRef,
    AVAudioFormat,
    AVAudioFormatRef,
    AVAudioFrameCount,
    AVAudioNodeBus,
    AVAudioPCMBufferRef,
    AVAudioTimeRef,
    NSTimeInterval,
    NSUInteger,
};

//    @typedef AVAudioNodeTapBlock
//    @abstract A block that receives copies of the output of an AVAudioNode.
//    @param buffer
//        a buffer of audio captured from the output of an AVAudioNode
//    @param when
//        the time at which the buffer was captured
//    @discussion
//        CAUTION: This callback may be invoked on a thread other than the main thread.

pub type AVAudioNodeTapBlock<'a> =
    block::RcBlock<(&'a AVAudioPCMBufferRef, &'a AVAudioTimeRef), ()>;
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
    /// /*! @method reset
    /// 	@abstract Clear a unit's previous processing state.
    /// */
    pub fn reset(&self) {
        unsafe { msg_send![self, reset] }
    }

    pub fn input_format_for_bus(&self, bus: AVAudioNodeBus) -> &AVAudioFormatRef {
        unsafe { msg_send![self, inputFormatForBus: bus.inner] }
    }

    pub fn output_format_for_bus(&self, bus: AVAudioNodeBus) -> &AVAudioFormatRef {
        unsafe { msg_send![self, outputFormatForBus: bus.inner] }
    }

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

    // - (void)installTapOnBus:(AVAudioNodeBus)bus bufferSize:(AVAudioFrameCount)bufferSize format:(AVAudioFormat * __nullable)format block:(AVAudioNodeTapBlock)tapBlock;
    pub fn install_tap<F>(
        &self,
        bus: AVAudioNodeBus,
        buffer_size: AVAudioFrameCount,
        format: Option<&AVAudioFormatRef>,
        // block: AVAudioNodeTapBlock,
        f: F,
    ) where
        F: 'static + Fn(&AVAudioPCMBufferRef, &AVAudioTimeRef) -> (),
    {
        let block = block::ConcreteBlock::new(
            move |buffer: &AVAudioPCMBufferRef, time_ref: &AVAudioTimeRef| {
                f(buffer, time_ref);
            },
        )
        .copy();

        unsafe {
            msg_send![self, installTapOnBus: bus.inner bufferSize: buffer_size format: format block: block]
        }
    }

    pub fn remove_tap(&self, bus: AVAudioNodeBus) {
        unsafe { msg_send![self, removeTap: bus.inner] }
    }

    pub fn engine(&self) -> &AVAudioEngineRef {
        unsafe { msg_send![self, engine] }
    }

    pub fn number_of_inputs(&self) -> NSUInteger {
        unsafe { msg_send![self, numberOfInputs] }
    }

    pub fn number_of_outputs(&self) -> NSUInteger {
        unsafe { msg_send![self, numberOfOutputs] }
    }

    /// /*! @property lastRenderTime
    /// 	@abstract Obtain the time for which the node most recently rendered.
    /// 	@discussion
    /// 		Will return nil if the engine is not running or if the node is not connected to an input or
    /// 		output node.
    /// */
    pub fn last_render_time(&self) -> &AVAudioTimeRef {
        unsafe { msg_send![self, lastRenderTime] }
    }

    /// /*! @property AUAudioUnit
    /// 	@abstract An AUAudioUnit wrapping or underlying the implementation's AudioUnit.
    /// 	@discussion
    /// 		This provides an AUAudioUnit which either wraps or underlies the implementation's
    /// 		AudioUnit, depending on how that audio unit is packaged. Applications can interact with this
    /// 		AUAudioUnit to control custom properties, select presets, change parameters, etc.
    ///
    /// 		No operations that may conflict with state maintained by the engine should be performed
    /// 		directly on the audio unit. These include changing initialization state, stream formats,
    /// 		channel layouts or connections to other audio units.
    /// */
    pub fn au_audio_unit(&self) -> &AUAudioUnitRef {
        unsafe { msg_send![self, AUAudioUnit] }
    }

    /// /*!	@property latency
    /// 	@abstract The processing latency of the node, in seconds.
    /// 	@discussion
    /// 		This property reflects the delay between when an impulse in the audio stream arrives at the
    /// 		input vs. output of the node. This should reflect the delay due to signal processing
    /// 		(e.g. filters, FFT's, etc.), not delay or reverberation which is being applied as an effect.
    /// 		A value of zero indicates either no latency or an unknown latency.
    /// */
    pub fn latency(&self) -> f64 {
        unsafe { msg_send![self, latency] }
    }

    /// /*!	@property outputPresentationLatency
    /// 	@abstract The maximum render pipeline latency downstream of the node, in seconds.
    /// 	@discussion
    /// 		This describes the maximum time it will take for the audio at the output of a node to be
    /// 		presented.
    /// 		For instance, the output presentation latency of the output node in the engine is:
    /// 			- zero in manual rendering mode
    /// 			- the presentation latency of the device itself when rendering to an audio device
    /// 			  (see `AVAudioIONode(presentationLatency)`)
    /// 		The output presentation latency of a node connected directly to the output node is the
    /// 		output node's presentation latency plus the output node's processing latency (see `latency`).
    ///
    /// 		For a node which is exclusively in the input node chain (i.e. not connected to engine's
    /// 		output node), this property reflects the latency for the output of this node to be
    /// 		presented at the output of the terminating node in the input chain.
    ///
    /// 		A value of zero indicates either an unknown or no latency.
    ///
    /// 		Note that this latency value can change as the engine is reconfigured (started/stopped,
    /// 		connections made/altered downstream of this node etc.). So it is recommended not to cache
    /// 		this value and fetch it whenever it's needed.
    /// */
    pub fn output_presentation_latency(&self) -> NSTimeInterval {
        unsafe { msg_send![self, outputPresentationLatency] }
    }
}
