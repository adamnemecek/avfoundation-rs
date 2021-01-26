use avfoundation::{
    nsstring_as_str,
    AVAudioEngine,
    AVAudioUnit,
    AVAudioUnitComponentManager,
    AVAudioUnitMIDIInstrument,
    AVAudioUnitRef,
    NSErrorRef,
    NSViewControllerRef,
    ShouldStop,
};

use block::ConcreteBlock;
use cocoa_foundation::base::id;
use objc::runtime::Object;
#[macro_use]
extern crate objc;

// fn class_desc(o: id) -> &str {
//     let desc = unsafe {
//         let cls: id = msg_send![o, class];
//         let desc: id = msg_send![cls, description];
//         nsstring_as_str(desc.as_ref().unwrap())
//     };
// }

fn main() {
    let manager = AVAudioUnitComponentManager::shared();
    // let components = manager.components_passing_test(|unit| (true, ShouldStop::Continue));
    let components = manager.components_passing_test(|unit| {
        if unit.name().contains("DLS") {
            (true, ShouldStop::Stop)
        } else {
            (false, ShouldStop::Continue)
        }
        // (true, ShouldStop::Continue)
    });

    let desc = components.first().unwrap().audio_component_description();

    let engine = AVAudioEngine::new();

    // println!("{:?}", components.first());

    // let midi = AVAudioUnitMIDIInstrument::new_with_audio_component_description(desc);
    let block = block::ConcreteBlock::new(move |unit: &AVAudioUnitRef, err: avfoundation::NSError| {
        let desc = unsafe {
            let cls: id = msg_send![unit, class];
            let desc: id = msg_send![cls, description];
            nsstring_as_str(desc.as_ref().unwrap())
        };
        println!("callback {:?} {:?}", desc, err.localized_description());
        let ui_block = block::ConcreteBlock::new(move |id: &avfoundation::NSViewControllerRef| {
            println!("ui block");
        })
        .copy();
        unit.au_audio_unit().request_view_controller(ui_block);
    })
    .copy();
    let unit = AVAudioUnit::with_component_description(desc, Default::default(), block);

    run_main_loop();
}

fn run_main_loop() {
    use cocoa_foundation::foundation::NSRunLoop;
    // let l = NSRunLoop::currentRunLoop();
    let run_loop: id = unsafe { NSRunLoop::currentRunLoop() };
    unsafe {
        let _: () = msg_send![run_loop, run];
    };
}
