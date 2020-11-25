use crate::AVAudioNode;

pub enum AVAudioPlayerNodeFFI {}

foreign_obj_type! {
    type CType = AVAudioPlayerNodeFFI;
    pub struct AVAudioPlayerNode;
    pub struct AVAudioPlayerNodeRef;
    type ParentType = AVAudioNode;
}

impl AVAudioPlayerNode {
    pub fn new() -> Self {
        unsafe {
            let class = class!(AVAudioPlayerNode);
            msg_send![class, new]
        }
    }
}

impl AVAudioPlayerNodeRef {}
