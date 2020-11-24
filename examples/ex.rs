use avfoundation::{AVAudioEngine, AVAudioNode};

fn main() {
    let engine = AVAudioEngine::new();
    let node = AVAudioNode::new();
    let z = node.number_of_inputs();
    println!("{:?}", engine.is_in_manual_rendering_mode());
}
