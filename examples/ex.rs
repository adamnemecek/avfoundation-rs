
use avfoundation::AudioNode;


fn main() {
    let node = AudioNode::new();
    let z = node.number_of_inputs();
    println!("{:?}", z);
}