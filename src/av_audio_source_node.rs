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
