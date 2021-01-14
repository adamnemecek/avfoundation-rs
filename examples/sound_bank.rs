use avfoundation::{
    AVAudioEngine,
    // ShouldStop,
    // AVAudioUnitComponentManager,
    AVAudioUnitSampler,
    AVAudioNode,
};

pub struct Instrument {
    engine: AVAudioEngine,
    sampler: AVAudioUnitSampler,
}

impl Instrument {
    pub fn new() -> Self {
        // println!("start");
        let engine = AVAudioEngine::new();

        // println!("2");
        let sampler = AVAudioUnitSampler::new();
        // println!("start");
        // let node = AVAudioNode::new();
        // let output = engine.output_node();
        // println!("3");
        engine.attach_node(&sampler);
        // println!("after attach");
        let output = engine.output_node();
        // println!("after output");
        engine.connect_nodes(&sampler, output, None);
        // println!("after connect");
        engine.start_and_return_error();
        // println!("after start");
        let bank = 121;
        let url = std::path::Path::new("/Users/adamnemecek/Downloads/FatBoy-v0.790.sf2");

        // println!("before load");
        sampler.load_bank_instrument_at_url(url.to_path_buf(), 0, bank, 0);
        println!("after load");
        Self { engine, sampler }
    }

    pub fn play_note(&self, pitch: u8, loudness: u8) {
        self.sampler.start_note(pitch, loudness, 0)
    }
}

fn main() {
    use rand::Rng;
    let instrument = Instrument::new();
    let mut rng = rand::thread_rng();

    for i in 0..100 {
        std::thread::sleep(std::time::Duration::from_millis(20));
        let pitch = rng.gen_range(0..127);
        instrument.play_note(pitch, 100);
    }
    std::thread::sleep(std::time::Duration::from_secs(1));
}
