use avfoundation::{
    self,
    AVMIDIPlayer,
};

fn main() {
    let path = std::path::PathBuf::from("/Users/adamnemecek/Downloads/darude-sandstorm.mid");
    let bank = std::path::PathBuf::from("/Users/adamnemecek/Downloads/FatBoy-v0.790.sf2");
    let mut player = AVMIDIPlayer::with_url(path, bank);

    println!("player {:?}", player.duration());
    player.set_rate(1.5);
    player.play(|| println!("done"));
    let duration = std::time::Duration::from_secs(30);
    std::thread::sleep(duration);
}
