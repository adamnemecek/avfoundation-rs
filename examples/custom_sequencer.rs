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

pub struct MIDIInstrument {
    pub inner: AVAudioUnitMIDIInstrument,
    block: AUScheduleMIDIEventBlock,
}

impl MIDIInstrument {
    pub fn new(component: &AVAudioUnitComponentRef) -> Self {
        let instrument = AVAudioUnitMIDIInstrument::new_with_audio_component_description(
            component.audio_component_description(),
        );
        let block = instrument
            .au_audio_unit()
            .schedule_midi_event_block()
            .unwrap()
            .clone();
        Self {
            inner: instrument,
            block,
        }
    }

    pub fn midi_msg(&self, msg: &[u8]) {
        unsafe {
            self.block.call((
                AUEventSampleTime::immediate(),
                0,
                msg.len() as _,
                msg.as_ptr(),
            ));
        }
    }
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

    // let instrument = AVAudioUnitMIDIInstrument::new_with_audio_component_description(
    //     component.audio_component_description(),
    // );

    let instrument = MIDIInstrument::new(component);

    let engine = AVAudioEngine::new();

    engine.attach_node(&instrument.inner);

    let output = engine.output_node();
    engine.connect_nodes(&instrument.inner, output, None);
    let _ = engine.start().unwrap();

    // instrument.au_audio_unit().token_by_adding_render_observer(observer)
    // midi_block.call(args)
        println!("here");
    // cbytes[0] = 0xB0 // status
    // cbytes[1] = 60 // note
    // cbytes[2] = 0 // velocity
    let msg: [u8; 3] = [0x90, 60, 100];
    // unsafe {
    //     midi_block.call((AUEventSampleTime::immediate(), 0, 3, msg.as_ptr()));
    // }
    // instrument.midi_msg(msg)

    instrument.midi_msg(&msg);

    std::thread::sleep(std::time::Duration::from_micros(10));

    // run_main_loop();
}
