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
    pub fn number_of_inputs(&self) -> i64 {
        unsafe { msg_send![self, numberOfOutputs] }
    }
}
