use avfoundation::{
    AVAudioConnectionPoint,
    AVAudioEngine,
    AVAudioNode,
    AVAudioNodeBus,
    AVAudioPlayerNode,
    AVAudioSequencer,
    AVAudioSession,
    AVAudioUnitComponentManager,
    ShouldStop,
};

// from AVAudioNode.h line 106
// <pre>
// AVAudioEngine *engine = [[AVAudioEngine alloc] init];
// AVAudioInputNode *input = [engine inputNode];
// AVAudioFormat *format = [input outputFormatForBus: 0];
// [input installTapOnBus: 0 bufferSize: 8192 format: format block: ^(AVAudioPCMBuffer *buf, AVAudioTime *when) {
// // ‘buf' contains audio captured from input node at time 'when'
// }];
fn main() {
    let engine = AVAudioEngine::new();

    let input = engine.input_node();
    if let Some(input) = input {
        let format = input.output_format_for_bus(0.into());
        input.install_tap(0.into(), 8192, Some(format), |buffer, when| {});
    }
}
