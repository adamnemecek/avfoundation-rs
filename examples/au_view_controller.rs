use avfoundation::{
    nsstring_as_str,
    run_main_loop,
    AVAudioEngine,
    AVAudioUnit,
    AVAudioUnitComponentManager,
    AVAudioUnitMIDIInstrument,
    AVAudioUnitRef,
    NSError,
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

// use std::sync::mpsc::channel;

fn main() {
    let manager = AVAudioUnitComponentManager::shared();
    // let components = manager.components_passing_test(|unit| (true, ShouldStop::Continue));
    let s = true;
    let name = if s { "Sylenth" } else { "DLS" };
    let components = manager.components_passing_test(move |unit| {
        if unit.name().contains(name) {
            (true, ShouldStop::Stop)
        } else {
            (false, ShouldStop::Continue)
        }
    });
    assert!(components.len() == 1);

    let desc = components.first().unwrap().audio_component_description();

    let engine = AVAudioEngine::new();

    // println!("{:?}", components.first());

    // let midi = AVAudioUnitMIDIInstrument::new_with_audio_component_description(desc);

    // let (tx, rx) = std::sync::mpsc::channel();
    // let (tx, rx) = std::sync::mpsc::channel();

    // let block =
    //     block::ConcreteBlock::new(move |unit: *mut AVAudioUnitRef, error: *mut NSErrorRef| {
    //         let tx1 = tx.clone();
    //         println!("here");
    //         let res = unsafe {
    //             if error.is_null() {
    //                 let a = unit.as_ref().unwrap().to_owned();
    //                 Ok(a)
    //             } else {
    //                 Err(error.as_ref().unwrap().to_owned())
    //             }
    //         };

    //         match res {
    //             Ok(unit) => {
    //                 let ui_block = block::ConcreteBlock::new(
    //                     move |id: Option<&avfoundation::NSViewControllerRef>| {
    //                         // println!("ui block");
    //                         let vc = id.map(|x| x.to_owned());
    //                         let _ = tx1.send(vc);
    //                     },
    //                 )
    //                 .copy();

    //                 unit.au_audio_unit().request_view_controller(ui_block);
    //             }
    //             Err(e) => {}
    //         }

    //         // match res {
    //         //     Ok(unit) => {
    //         //         unit.au_audio_unit().request_view_controller(
    //         //             block::ConcreteBlock::new(move |a| {
    //         //                 tx1.send(1);
    //         //             })
    //         //             .copy(),
    //         //         );
    //         //     }
    //         //     Err(_) => todo!(),
    //         // }
    //         //     let desc = unsafe {
    //         //         let cls: id = msg_send![unit, class];
    //         //         let desc: id = msg_send![cls, description];
    //         //         nsstring_as_str(desc.as_ref().unwrap())
    //         //     };
    //         //     println!("callback {:?} {:?}", desc, err.localized_description());
    //         //     let ui_block =
    //         //         block::ConcreteBlock::new(move |id: &avfoundation::NSViewControllerRef| {
    //         //             println!("ui block");
    //         //         })
    //         //         .copy();
    //         //     unit.au_audio_unit().request_view_controller(ui_block);
    //     })
    //     .copy();

    // let unit = AVAudioUnit::new_with_component_description(desc, Default::default(), block);

    // let unit = AVAudioUnit::new_with_component_description_tx(desc, Default::default(), &tx);
    let unit = AVAudioUnitMIDIInstrument::new_with_audio_component_description(desc);

    // unit.au_audio_unit().request_view_controller_tx(&tx);
    unit.au_audio_unit().request_view_controller_fn(move |vc| {
        // let z = tx.send(avfoundation::AVFoundationEvent::RequestViewController(vc));
        println!("request view controller");
    });
    // println!("here");
    // let unit =
    //     AVAudioUnit::new_with_component_description_fn(desc, Default::default(), |a| match a {
    //         Ok(unit) => {
    //             tx1.send(1);
    //         }
    //         Err(_) => {}
    //     });

    // use avfoundation::AVFoundationEvent;
    // loop {
    // std::thread::sleep(std::time::Duration::from_millis(100));
    //     for e in rx.try_recv() {
    //         println!("here");
    //         match e {
    //             AVFoundationEvent::AVAudioUnitHandler(unit) => match unit {
    //                 Ok(unit) => {

    //                     // engine.attach_node(&unit);
    //                     unit.au_audio_unit().request_view_controller_tx(&tx);
    //                 }
    //                 Err(_) => {
    //                     todo!()
    //                 }
    //             },
    //             AVFoundationEvent::RequestViewController(vc) => {
    //                 println!("vc {:?}", vc);
    //             }
    //         }
    //     }
    // }
    run_main_loop();

    // println!("received {:?}", a);
    // let f = ;
    // let unit = AVAudioUnit::new_with_component_description(
    //     desc,
    //     Default::default(),
    //     |res: Result<AVAudioUnit, NSError>| match res {
    //         Ok(unit) => {
    //             unit.au_audio_unit().request_view_controller_fn(|view| {
    //                 tx.send(view);
    //             });
    //         }
    //         Err(_) => todo!(),
    //     },
    // );
}
