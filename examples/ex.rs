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

fn main1() {
    let engine = AVAudioEngine::new();
    let node = AVAudioNode::new();
    let z = node.number_of_inputs();
    println!("{:?}", z);
    // let au = node.au_audio_unit();
    // let desc = au.component_description();
    // println!("thing {:?}", desc);
    // let player = AVAudioPlayerNode::new();
    // let au = player.au_audio_unit();
    let sequencer = AVAudioSequencer::with_engine(&engine);
    // println!("{:?}", sequencer.is_playing());
    let tracks = sequencer.tracks();

    let point = AVAudioConnectionPoint::with_node(&node, 1.into());
    println!("point {:?}", point.bus());
    // println!("tracks: {:?}", tracks);
    // println!("tracks: {:?}", sequencer.tempo_track());
    // let manager = AVAudioUnitComponentManager::shared();
    // let components = manager.components_passing_test(|unit| (true, ShouldStop::Continue));
    // for c in components {
    //     println!(
    //         "manufacturer: {:?}, name: {:?}, description: {:?}, type_name {:?}",
    //         c.manufacturer_name(),
    //         c.name(),
    //         c.audio_component_description(),
    //         c.type_name(),
    //     );
    // }
    // println!("{:?}", engine.is_in_manual_rendering_mode());
}

fn main2() {
    let session = AVAudioSession::shared();
    println!("{:?}", session.record_permission());
}

fn main() {
    main1()
}
