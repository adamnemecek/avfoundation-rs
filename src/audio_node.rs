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
// open func reset()
// open func inputFormat(forBus bus: AVAudioNodeBus) -> AVAudioFormat
// open func outputFormat(forBus bus: AVAudioNodeBus) -> AVAudioFormat
// open func name(forInputBus bus: AVAudioNodeBus) -> String?
// open func name(forOutputBus bus: AVAudioNodeBus) -> String?
// open func installTap(onBus bus: AVAudioNodeBus, bufferSize: AVAudioFrameCount, format: AVAudioFormat?, block tapBlock: @escaping AVAudioNodeTapBlock)
// open func removeTap(onBus bus: AVAudioNodeBus)
    pub fn number_of_inputs(&self) -> i64 {
        unsafe { msg_send![self, numberOfOutputs] }
    }
}
