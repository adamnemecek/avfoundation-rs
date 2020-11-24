use avfoundation::{AVAudioEngine, AVAudioUnitComponentManager, AVAudioNode};

fn main() {
    let engine = AVAudioEngine::new();
    let node = AVAudioNode::new();
    let z = node.number_of_inputs();

    let manager = AVAudioUnitComponentManager::shared();
    println!("{:?}", engine.is_in_manual_rendering_mode());
}
