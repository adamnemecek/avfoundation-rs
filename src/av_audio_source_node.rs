use crate::{
    AVAudioFormat,
    AVAudioMixing,
    AVAudioNodeRef,
    AVAudioStereoMixing,
};

pub type AVAudioSourceNodeRenderBlock = block::Block<(), ()>;

pub enum AVAudioSourceNodeFFI {}

foreign_obj_type! {
    type CType = AVAudioSourceNodeFFI;
    pub struct AVAudioSourceNode;
    pub struct AVAudioSourceNodeRef;
    type ParentType = AVAudioNodeRef;
}

impl AVAudioSourceNode {
    pub fn with_render_block(render_block: AVAudioSourceNodeRenderBlock) -> Self {
        unsafe {
            let class = class!(AVAudioSourceNode);
            let alloc: *const AVAudioSourceNodeRef = msg_send![class, alloc];
            msg_send![alloc, initWithRenderBlock: render_block]
        }
    }

    //     /*! @method initWithFormat:renderBlock:
    //     @abstract Create a node with a render block.
    //     @param format
    //         The format of the PCM audio data that will be supplied by the block.
    //     @param block
    //         The block to supply audio data to the output.
    //     @discussion
    //         The block can be called on realtime or non-realtime threads depending on the engine’s
    //         operating mode and it is the client's responsibility to handle it in a thread-safe manner.

    //         The audio format for the output bus will be set from the connection format when connecting
    //         to another node.

    //         AVAudioSourceNode supports different audio formats for the block and output, but only
    //         Linear PCM conversions are supported (sample rate, bit depth, interleaving).
    //  */
    //- (instancetype)initWithFormat:(AVAudioFormat*)format renderBlock:(AVAudioSourceNodeRenderBlock)block NS_DESIGNATED_INITIALIZER;
    pub fn with_format_and_render_block(
        format: AVAudioFormat,
        render_block: AVAudioSourceNodeRenderBlock,
    ) -> Self {
        unsafe {
            let class = class!(AVAudioSourceNode);
            let alloc: *const AVAudioSourceNodeRef = msg_send![class, alloc];
            msg_send![alloc, initWithFormat: format renerBlock: render_block]
        }
    }
}

impl AVAudioSourceNodeRef {}

impl AVAudioStereoMixing for AVAudioSourceNodeRef {}

impl AVAudioMixing for AVAudioSourceNodeRef {}
