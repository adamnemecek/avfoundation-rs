use avfoundation::{AudioEngine, AudioNode};

fn main() {
    let engine = AudioEngine::new();
    let node = AudioNode::new();
    let z = node.number_of_inputs();
    println!("{:?}", engine.is_in_manual_rendering_mode());
}
