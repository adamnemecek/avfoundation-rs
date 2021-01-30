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
    let component_desc = components.first().unwrap();
    println!("dls {:?}", component_desc);

    let desc = component_desc.audio_component_description();
    let plugin = AVAudioUnitMIDIInstrument::new_with_audio_component_description(desc);
    let au_unit = plugin.au_audio_unit();
    let tree = au_unit.parameter_tree().unwrap();
    let params = tree.all_parameters();
    let param = params.first().unwrap().to_owned();

    let block = au_unit.midi_output_event_block();

    // for param in params {
    //     println!("address: {:?}", param.address());
    //     println!("value: {:?}", param.value());
    //     println!("min_value: {:?}", param.min_value());
    //     println!("max_value: {:?}", param.max_value());
    //     println!("identifier: {:?}", param.display_name());
    // }
}
