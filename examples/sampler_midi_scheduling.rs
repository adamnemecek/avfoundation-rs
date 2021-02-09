// func sampler_midi_scheduling() {
//     let engine = AVAudioEngine()
//     let sampler = AVAudioUnitSampler()

//     engine.attach(sampler)
//     let output = engine.outputNode
//     engine.connect(sampler, to: output, format: nil)
//     try! engine.start()
//     let path = "/Users/adamnemecek/Downloads/FatBoy-v0.790.sf2"
//     let url = URL(fileURLWithPath: path)
//     try! sampler.loadSoundBankInstrument(at: url, program: 100, bankMSB: 121, bankLSB: 0)

//     sampler.auAudioUnit.token(byAddingRenderObserver: {
//         (flags, timestamp, frameCount, outputBusNumber) in
//         let pitch: UInt8 = (0...127).randomElement()! as UInt8
//         sampler.startNote(pitch, withVelocity: 100, onChannel: 0)
//     })

//     RunLoop.main.run()

// }
use avfoundation::{
    run_main_loop,
    AUAudioFrameCount,
    // AudioTimeStamp,
    AVAudioEngine,
    AVAudioUnitSampler,
    AudioUnitRenderActionFlags,
};

use avfoundation::prelude::AudioTimeStamp;
// use block::

// use rand::

fn main() {
    let engine = AVAudioEngine::new();
    let sampler = AVAudioUnitSampler::new();

    engine.attach_node(&sampler);
    let output = engine.output_node();
    engine.connect_nodes(&sampler, &output, None);
    let _ = engine.start().unwrap();
    let url = std::path::PathBuf::from("/Users/adamnemecek/Downloads/FatBoy-v0.790.sf2");
    let bank = 121;
    sampler
        .load_bank_instrument_at_url(url.to_path_buf(), 100, bank, 0)
        .unwrap();

    let s2 = sampler.clone();

    struct Event {
        timestamp: AudioTimeStamp,
    }

    let (tx, rx) = std::sync::mpsc::channel();

    let block = block::ConcreteBlock::new(
        move |flags: AudioUnitRenderActionFlags,
              timestamp: *const AudioTimeStamp,
              frame_count: AUAudioFrameCount,
              bus: i64| {
            let ts = unsafe { *timestamp };
            let _ = tx.send(Event { timestamp: ts });
            s2.start_note(100, 100, 0);
        },
    )
    .copy();
    sampler
        .au_audio_unit()
        .token_by_adding_render_observer(block);

    // let previous =
    for e in rx.recv() {}
}
