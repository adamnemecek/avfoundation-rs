use avfoundation::{
    AVAudioEngine,
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
        println!("start");
        let engine = AVAudioEngine::new();
        println!("2");
        let sampler = AVAudioUnitSampler::new();
        println!("start");
        engine.attach_node(&sampler);
        println!("after attach");
        engine.connect_nodes(&sampler, engine.output_node(), None);
        println!("after connect");
        engine.start_and_return_error();
        println!("after start");
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
    let instrument = Instrument::new();

    instrument.play_note(100, 50);
}
