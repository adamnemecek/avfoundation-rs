use avfoundation::{
    self,
    AVMIDIPlayer,
};

fn main() {
    let path = std::path::PathBuf::from("/Users/adamnemecek/Code/avfoundation-rs");
    let player = AVMIDIPlayer::with_url(path.clone(), path);
}
