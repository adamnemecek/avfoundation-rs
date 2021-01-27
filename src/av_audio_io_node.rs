use crate::{
    AVAudioFormatRef,
    AVAudioMixing,
    AVAudioNodeRef,
    AVAudioStereoMixing,
};

pub type NSTimeInterval = f64;

use objc::runtime::{
    NO,
    YES,
};

pub enum AVAudioIONodeFFI {}

foreign_obj_type! {
    type CType = AVAudioIONodeFFI;
    pub struct AVAudioIONode;
    pub struct AVAudioIONodeRef;
    type ParentType = AVAudioNodeRef;
}

impl AVAudioIONodeRef {
    /// @property presentationLatency
    /// @abstract
    ///     The presentation or hardware latency, applicable when the engine is rendering to/from an
    ///     audio device.
    /// @discussion
    ///     This corresponds to kAudioDevicePropertyLatency and kAudioStreamPropertyLatency.
    ///     See <CoreAudio/AudioHardwareBase.h>.
    pub fn presentation_latency(&self) -> NSTimeInterval {
        unsafe { msg_send![self, presentationLatency] }
    }

    // @property audioUnit
    // @abstract
    //     The node's underlying AudioUnit, if any.
    // @discussion
    //     This is only necessary for certain advanced usages.
    // pub fn audio_unit(&self) -> AudioUnit {
    //     todo!()
    // }
}

/// todo:
pub type AVAudioIONodeInputBlock = block::RcBlock<(), ()>;
/// @class AVAudioInputNode
///     @abstract
///         A node that performs audio input in the engine.
///     @discussion
///          When the engine is rendering to/from an audio device, this node connects to the system's
///         audio input.
///         When the engine is operating in manual rendering mode, this node can be used to supply
///         the input data to the engine.
///
///         This node has one element.
///         The format of the input scope reflects:
///             - the audio hardware sample rate and channel count, when connected to the hardware
///             - the format of the PCM audio data that the node will supply to the engine, in the
///               manual rendering mode (see `setManualRenderingInputPCMFormat:inputBlock:`)
///
///         When rendering from an audio device, the input node does not support format conversion.
///         Hence the format of the output scope must be same as that of the input, as well as the
///         formats for all the nodes connected in the input node chain.
///
///         In the manual rendering mode, the format of the output scope is initially the same as that
///         of the input, but you may set it to a different format, in which case the node will convert.

pub enum AVAudioInputNodeFFI {}

foreign_obj_type! {
    type CType = AVAudioInputNode;
    pub struct AVAudioInputNode;
    pub struct AVAudioInputNodeRef;
    type ParentType = AVAudioNodeRef;
}

impl AVAudioInputNodeRef {
    /// @method setManualRenderingInputPCMFormat:inputBlock:
    /// @abstract
    ///     Supply the data through the input node to the engine operating in the manual rendering mode.
    /// @param format
    ///     The format of the PCM audio data the block will supply to the engine
    /// @param block
    ///     The block the engine will call on the input node to get the audio to send to the output,
    ///     when operating in the manual rendering mode. See `AVAudioIONodeInputBlock` for more details
    /// @return
    ///     YES for success
    /// @discussion
    ///     This block must be set if the input node is being used when the engine is operating in
    ///     manual rendering mode.
    ///     Switching the engine to render to/from an audio device invalidates any previously set block,
    ///     and makes this method ineffective.
    // - (BOOL)setManualRenderingInputPCMFormat:(AVAudioFormat *)format inputBlock:(AVAudioIONodeInputBlock)block API_AVAILABLE(macos(10.13), ios(11.0), watchos(4.0), tvos(11.0));
    pub fn set_manual_rendering_pcm_format(
        &self,
        format: &AVAudioFormatRef,
        input_block: AVAudioIONodeInputBlock,
    ) -> bool {
        unsafe {
            match msg_send![self, setManualRenderingInputPCMFormat: format inputBlock: input_block]
            {
                YES => true,
                NO => false,
            }
        }
    }

    // /*! @property voiceProcessingBypassed
    //     @abstract
    //        Bypass all processing done by the voice processing unit.
    //     @discussion
    //        Querying this property when voice processing is disabled will return false.
    //  */
    // @property (nonatomic, getter=isVoiceProcessingBypassed) BOOL voiceProcessingBypassed API_AVAILABLE(macos(10.15), ios(13.0), watchos(6.0), tvos(13.0));
    pub fn is_voice_processing_bypassed(&self) -> bool {
        unsafe {
            match msg_send![self, isVoiceProcessingBypassed] {
                YES => true,
                NO => false,
            }
        }
    }

    pub fn set_voice_processing_bypassed(&self, value: bool) {
        unsafe { msg_send![self, setVoiceProcessingBypassed: value] }
    }

    // /*! @property voiceProcessingAGCEnabled
    //     @abstract
    //         Enable automatic gain control on the processed microphone/uplink
    //         signal. Enabled by default.
    //     @discussion
    //         Querying this property when voice processing is disabled will return false.
    //  */
    // @property (nonatomic, getter=isVoiceProcessingAGCEnabled) BOOL voiceProcessingAGCEnabled API_AVAILABLE(macos(10.15), ios(13.0), watchos(6.0), tvos(13.0));

    // /*! @property voiceProcessingInputMuted
    //     @abstract
    //         Mutes the input of the voice processing unit.
    //     @discussion
    //         Querying this property when voice processing is disabled will return false.
    //     */
    // @property (nonatomic, getter=isVoiceProcessingInputMuted) BOOL voiceProcessingInputMuted API_AVAILABLE(macos(10.15), ios(13.0), watchos(6.0), tvos(13.0));
}

impl AVAudioStereoMixing for AVAudioInputNodeRef {}
impl AVAudioMixing for AVAudioInputNodeRef {}

/// @abstract
/// A node that performs audio output in the engine.
/// @discussion
/// When the engine is rendering to/from an audio device, this node connects to the system's
/// audio output.
/// When the engine is operating in manual rendering mode, this node performs output in
/// response to client's requests.
///
/// This node has one element.
/// The format of the output scope reflects:
///     - the audio hardware sample rate and channel count, when connected to the hardware
///     - the engine's manual rendering mode output format (see
///       `AVAudioEngine(manualRenderingFormat)`), in the manual rendering mode

/// The format of the input scope is initially the same as that of the
/// output, but you may set it to a different format, in which case the node will convert.
pub enum AVAudioOutputNodeFFI {}

foreign_obj_type! {
    type CType = AVAudioOutputNodeFFI;
    pub struct AVAudioOutputNode;
    pub struct AVAudioOutputNodeRef;
    type ParentType = AVAudioIONodeRef;
}

impl AVAudioOutputNodeRef {}
