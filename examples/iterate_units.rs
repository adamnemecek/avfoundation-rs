use avfoundation::{
    AVAudioUnitComponentManager,
    ShouldStop,
    AVAudioUnitMIDIInstrument,
};

fn main() {
    let manager = AVAudioUnitComponentManager::shared();
    // let components = manager.components_passing_test(|unit| (true, ShouldStop::Continue));
    let components = manager.components_passing_test(|unit| {
        if unit.name().contains("Serum") {
            (true, ShouldStop::Stop)
        } else {
            (false, ShouldStop::Continue)
        }
    });
    let serum = components.first().unwrap();
    println!("serum {:?}", serum);

    let desc = serum.audio_component_description();
    let midi = AVAudioUnitMIDIInstrument::new_with_audio_component_description(desc);
    println!("serum {:?}", midi);



    std::thread::sleep(std::time::Duration::from_secs(1));
    // for c in components {
    //     println!(
    //         "manufacturer: {:?}, name: {:?}, description: {:?}, type_name {:?}",
    //         c.manufacturer_name(),
    //         c.name(),
    //         c.audio_component_description(),
    //         c.type_name(),
    //     );
    //     println!("--");
    // }
}
