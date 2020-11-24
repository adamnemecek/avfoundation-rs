use avfoundation::{AVAudioEngine, AVAudioNode, AVAudioUnitComponentManager, ShouldStop};

fn main() {
    let engine = AVAudioEngine::new();
    let node = AVAudioNode::new();
    let z = node.number_of_inputs();

    let manager = AVAudioUnitComponentManager::shared();
    let components = manager.components_passing_test(|unit| (true, ShouldStop::Continue));
    println!("{:?}", engine.is_in_manual_rendering_mode());
}
