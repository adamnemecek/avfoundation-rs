use avfoundation::{
    AVAudioEngine,
    AVAudioUnit,
    AVAudioUnitComponentManager,
    AVAudioUnitMIDIInstrument,
    AVAudioUnitRef,
    NSErrorRef,
    ShouldStop,
};

use block::ConcreteBlock;

fn main() {
    let manager = AVAudioUnitComponentManager::shared();
    // let components = manager.components_passing_test(|unit| (true, ShouldStop::Continue));
    let components = manager.components_passing_test(|unit| {
        if unit.name().contains("Serum") {
            (true, ShouldStop::Stop)
        } else {
            (false, ShouldStop::Continue)
        }
        // (true, ShouldStop::Continue)
    });

    let desc = components.first().unwrap().audio_component_description();

    let engine = AVAudioEngine::new();

    // println!("{:?}", components.first());
    // let midi = AVAudioUnitMIDIInstrument::new_with_audio_component_description(desc);
    let block =
        block::ConcreteBlock::new(move |unit: &AVAudioUnitRef, &NSErrorRef| println!("callback"))
            .copy();
    let unit = AVAudioUnit::with_component_description(desc, Default::default(), block);

    // RunLoop.main.run()
    use cocoa_foundation::foundation::NSRunLoop;
    let l = NSRunLoop::currentRunLoop();
}
