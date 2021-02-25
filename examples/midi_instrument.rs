use avfoundation::{
    AUEventSampleTimeImmediate,
    AVAudioEngine,
    AVAudioNode,
    AVAudioUnit,
    AVAudioUnitSampler,
};

// fn main1() {

//     let manager = AVAudioUnitComponentManager::shared();
//     let components = manager.components_passing_test(|unit| {
//         if unit.name().contains("DLS") {
//             (true, ShouldStop::Stop)
//         } else {
//             (false, ShouldStop::Continue)
//         }
//     });

//     let component = components.first().unwrap();
// }

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
        let _ = engine.start().unwrap();
        let bank = 121;
        let url = std::path::Path::new("/Users/adamnemecek/Downloads/FatBoy-v0.790.sf2");

        // println!("before load");
        sampler
            .load_bank_instrument_at_url(url.to_path_buf(), 100, bank, 0)
            .unwrap();
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

    // pub fn start(&self) {
    //     self.engine.start_and_return_error();
    // }
}

pub fn sleep(s: u64) {
    std::thread::sleep(std::time::Duration::from_secs(s))
}

fn main() {
    use rand::Rng;
    // println!("{:?}", avfoundation::AUEventSampleTimeImmediate);
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
    let audiounit = instrument.sampler.au_audio_unit();

    // println!("{:?}", audiounit.device_input_latency());
    println!("here");
    let schedule_midi = audiounit.schedule_midi_event_fn().unwrap(); //.unwrap();

    // elf.noteBlock(AUEventSampleTimeImmediate, 0, 3, cbytes)
    // cbytes[0] = 0xB0
    //         cbytes[1] = 123
    //         cbytes[2] = 0
    let bytes: [u8; 3] = [0x90, 123, 50];
    println!("here1");
    // unsafe {
    //     block.call((AUEventSampleTimeImmediate, 0, 3, bytes.as_ptr()));
    // }
    schedule_midi(AUEventSampleTimeImmediate, 0, &bytes);
    // println!("here2");
    sleep(1);
    // println!("{:?}", instrument.sampler.au_audio_unit().latency());

    use chromagear::prelude::*;
    let d_major = Scale::new(PC::C, &[0, 2, 4, 5, 7, 9, 11]);

    let channel = 0;
    let loudness = 100;
    // for degree in 0..8 {
    //     // let degree = rng.gen_range(0..6);
    //     println!("degree {}", degree);

    //     let chord = d_major.function(degree % 7);
    //     // println!("chord {:?}", chord.names());

    //     for p in chord.iter() {
    //         instrument.play_note(Pitch::new(p, 5).into(), loudness, channel);
    //     }
    //     std::thread::sleep(std::time::Duration::from_millis(1000));
    //     for p in chord.iter() {
    //         instrument.stop_note(Pitch::new(p, 5).into(), channel);
    //     }
    // }

    // std::thread::sleep(std::time::Duration::from_secs(1));
}
