use avfoundation::{
    AVAudioEngine,
    AVAudioNode,
    // ShouldStop,
    // AVAudioUnitComponentManager,
    AVAudioUnitSampler,
};

pub struct Instrument {
    engine: AVAudioEngine,
    sampler: AVAudioUnitSampler,
}

impl Instrument {
    pub fn new() -> Self {
        let engine = AVAudioEngine::new();

        let sampler = AVAudioUnitSampler::new();
        engine.attach_node(&sampler);
        let output = engine.output_node();
        engine.connect_nodes(&sampler, output, None);
        engine.start_and_return_error();
        let bank = 121;
        let url = std::path::Path::new("/Users/adamnemecek/Downloads/FatBoy-v0.790.sf2");

        // println!("before load");
        sampler.load_bank_instrument_at_url(url.to_path_buf(), 100, bank, 0);
        println!("after load");
        Self { engine, sampler }
    }

    pub fn play_note(&self, note: u8, loudness: u8, channel: u8) {
        self.sampler.start_note(note, loudness, channel)
    }

    // pub fn play_chord(&self, )

    pub fn stop_note(&self, note: u8, channel: u8) {
        self.sampler.stop_note(note, channel)
    }
}

fn main() {
    use rand::Rng;
    let instrument = Instrument::new();
    let mut rng = rand::thread_rng();

    // for i in 0..20000 {
    //     std::thread::sleep(std::time::Duration::from_millis(100));
    //     let pitch = rng.gen_range(0..127);
    //     instrument.play_note(pitch, 100, 0);
    //     let pitch = rng.gen_range(0..127);
    //     instrument.play_note(pitch, 100, 0);
    //     let pitch = rng.gen_range(0..127);
    //     instrument.play_note(pitch, 100, 0);
    //     let pitch = rng.gen_range(0..127);
    //     instrument.play_note(pitch, 100, 0);
    // }

    use chromagear::prelude::*;
    let d_major = Scale::new(PC::C, &[0, 2, 4, 5, 7, 9, 11]);

    let channel = 0;
    let loudness = 100;
    for degree in 0..8 {
        // let degree = rng.gen_range(0..6);
        println!("degree {}", degree);

        let chord = d_major.chord(degree % 7);
        println!("chord {:?}", chord.names());

        for p in chord.iter() {
            instrument.play_note(Pitch::new(p, 5).into(), loudness, channel);
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
        for p in chord.iter() {
            instrument.stop_note(Pitch::new(p, 5).into(), channel);
        }
    }

    // std::thread::sleep(std::time::Duration::from_secs(1));
}
