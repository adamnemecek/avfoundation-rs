use avfoundation::{
    AVAudioEngine,
    AVAudioNode,
    AVAudioSession,
    AVAudioUnitComponentManager,
    ShouldStop,
};

fn main1() {
    let engine = AVAudioEngine::new();
    let node = AVAudioNode::new();
    let z = node.number_of_inputs();

    let manager = AVAudioUnitComponentManager::shared();
    let components = manager.components_passing_test(|unit| (true, ShouldStop::Continue));
    for c in components {
        println!(
            "manufacturer: {:?}, name: {:?}, description: {:?}, type_name {:?}",
            c.manufacturer_name(),
            c.name(),
            c.audio_component_description(),
            c.type_name(),
        );
    }
    println!("{:?}", engine.is_in_manual_rendering_mode());
}

fn main() {
    let session = AVAudioSession::shared();
    println!("{:?}", session.record_permission());
}
