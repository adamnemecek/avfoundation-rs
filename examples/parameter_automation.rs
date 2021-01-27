use avfoundation::prelude::*;

fn main() {
    let e = AVAudioEngine::new();
    let manager = AVAudioUnitComponentManager::shared();
    let components = manager.components_passing_test(|unit| {
        if unit.name().contains("Sylenth") {
            (true, ShouldStop::Stop)
        } else {
            (false, ShouldStop::Continue)
        }
    });
    let serum = components.first().unwrap();
    println!("dls {:?}", serum);

    let desc = serum.audio_component_description();
    let plugin = AVAudioUnitMIDIInstrument::new_with_audio_component_description(desc);
    let unit = plugin.au_audio_unit();
    let tree = unit.parameter_tree().unwrap();
    let params = tree.all_parameters();
    let param = params.first().unwrap().to_owned();

    for param in params {
        println!("address: {:?}", param.address());
        println!("value: {:?}", param.value());
        println!("min_value: {:?}", param.min_value());
        println!("max_value: {:?}", param.max_value());
        println!("identifier: {:?}", param.display_name());
    }
}
