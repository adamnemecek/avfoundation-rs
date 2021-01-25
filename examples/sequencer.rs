// #![deny(unused_must_use)]

use avfoundation::{
    AVAudioEngine,
    AVAudioSequencer,
    AVMusicSequenceLoadOptions,
};


fn main() {
    // five();
    let path = std::path::PathBuf::from("/Users/adamnemecek/Downloads/darude-sandstorm.mid");
    // let bank = std::path::PathBuf::from("/Users/adamnemecek/Downloads/FatBoy-v0.790.sf2");
    let engine = AVAudioEngine::new();
    let sequencer = AVAudioSequencer::with_engine(&engine);

    match sequencer.load_from_url(path, Default::default()) {
        Ok(res) => {
            println!("ok")
        }
        Err(e) => println!("err {:?}", e.domain()),
    };
    let tempo = sequencer.tempo_track().unwrap();
    println!("tempo {:?}", tempo.is_soloed());

    let tracks = sequencer.tracks();
    for t in tracks {
        println!("is_soloed {:?}", t.is_soloed());
    }
    println!(
        "current_position_in_seconds {:?}",
        sequencer.current_position_in_seconds()
    );

    // println!("player {:?}", player.duration());
    // player.set_rate(1.5);
    // player.play(|| println!("done"));
    // let duration = std::time::Duration::from_secs(200);
    // player.set_current_position(30.0);
}
