use avfoundation::{
    AVAudioEngine,
    AVAudioNode,
    // ShouldStop,
    // AVAudioUnitComponentManager,
    AVAudioUnitSampler,
};

pub struct Instrument {
    engine: AVAudioEngine,
    pub sampler: AVAudioUnitSampler,
}

impl Instrument {
    pub fn new() -> Self {
        let engine = AVAudioEngine::new();

        let sampler = AVAudioUnitSampler::new();
        println!("kind {:?}", sampler.kind());
        engine.attach_node(&sampler);
        let output = engine.output_node();
        engine.connect_nodes(&sampler, output, None);
        engine.start();
        let bank = 121;
        let url = std::path::Path::new("/Users/adamnemecek/Downloads/FatBoy-v0.790.sf2");
        // println!("avfoundation {:?}", avfoundation::cls_name(&sampler));
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

    pub fn start_note1(&self, note: u8, velocity: u8) {
        unsafe {
            avfoundation::MusicDeviceMIDIEvent(
                self.sampler.audio_unit(),
                0x90,
                note as _,
                velocity as _,
                0,
            );
        }
    }

    pub fn stop_note1(&self, note: u8, channel: u8) {
        unsafe {
            avfoundation::MusicDeviceMIDIEvent(self.sampler.audio_unit(), 0x90, note as _, 0, 0);
        }
    }

    pub fn playback(&self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        use chromagear::prelude::*;
        let d_major = Scale::new(PC::C, &[0, 2, 4, 5, 7, 9, 11]);

        let channel = 0;
        let loudness = 100;
        for degree in 0..8 {
            // let degree = rng.gen_range(0..6);
            println!("degree {}", degree);

            let chord = d_major.function(degree % 7);
            // println!("chord {:?}", chord.names());

            for p in chord.iter() {
                // instrument.play_note(Pitch::new(p, 5).into(), loudness, channel);
                self.start_note1(Pitch::new(p, 5).into(), loudness);
            }
            std::thread::sleep(std::time::Duration::from_millis(1000));
            for p in chord.iter() {
                // instrument.stop_note(Pitch::new(p, 5).into(), channel);
                self.stop_note1(Pitch::new(p, 5).into(), 0);
            }
        }
    }
}

fn main() {
    let instrument = Instrument::new();
    // let instrument2 = Instrument::new();
    instrument.playback();
    // instrument2.playback();

    // let token = instrument
    //     .sampler
    //     .au_audio_unit()
    //     .token_by_adding_render_observer_fn(move |a, b, c, d| {
    //         todo!("in callback");
    //     });

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

    // std::thread::sleep(std::time::Duration::from_secs(1));
}
