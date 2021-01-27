use avfoundation::AVAudioEngine;
fn main() {
    let e = AVAudioEngine::new();
    let manager = AVAudioUnitComponentManager::shared();
    let components = manager.components_passing_test(|unit| {
        if unit.name().contains("DLS") {
            (true, ShouldStop::Stop)
        } else {
            (false, ShouldStop::Continue)
        }
    });
    let serum = components.first().unwrap();
    println!("serum {:?}", serum);

    let desc = serum.audio_component_description();
    let midi = AVAudioUnitMIDIInstrument::new_with_audio_component_description(desc);
}
