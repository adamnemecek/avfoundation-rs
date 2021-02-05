use avfoundation::prelude::*;

use block::ConcreteBlock;
use cocoa_foundation::base::id;
use objc::runtime::Object;
#[macro_use]
extern crate objc;

fn run_main_loop() {
    use cocoa_foundation::foundation::NSRunLoop;
    // let l = NSRunLoop::currentRunLoop();
    let run_loop: id = unsafe { NSRunLoop::currentRunLoop() };
    unsafe {
        let _: () = msg_send![run_loop, run];
    };
}

fn main() {
    // you need to get the
    let manager = AVAudioUnitComponentManager::shared();
    let components = manager.components_passing_test(|unit| {
        if unit.name().contains("Primer") {
            (true, ShouldStop::Stop)
        } else {
            (false, ShouldStop::Continue)
        }
    });
    let component = components.first().unwrap();
    // let engine = AVAudioEngine::new();
    println!("components {:?}", components);

    let instrument = AVAudioUnitMIDIInstrument::new_with_audio_component_description(
        component.audio_component_description(),
    );

    let engine = AVAudioEngine::new();

    let midi_block = instrument.au_audio_unit().schedule_midi_event_block();
    engine.attach_node(&instrument);

    let output = engine.output_node();
    engine.connect_nodes(&instrument, output, None);
    let _ = engine.start().unwrap();

    run_main_loop();
}
