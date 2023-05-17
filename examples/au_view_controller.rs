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
use cocoa_foundation::{
    base::id,
    foundation::NSRect,
};
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
    // let name = if s { "Sylenth" } else { "DLS" };
    let name = "Helm";
    let components = manager.components_passing_test(move |unit| {
        // if unit.name().contains(name) {
        //     (true, ShouldStop::Continue)
        // } else {
        //     (false, ShouldStop::Continue)
        // }
        (unit.name().contains(name), ShouldStop::Continue)
    });
    // assert!(components.len() == 1);
    // for e in components.iter() {
    //     println!("{:?}", e.name());
    // }

    println!("{:?}", components.iter().map(|x| x.name()).collect::<Vec<_>>());
    let desc = components[0].audio_component_description();

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

    let (tx, rx) = std::sync::mpsc::channel();
    let unit = AVAudioUnit::new_with_component_description_fn(desc, Default::default(), move |x| {
        tx.send(x);
    });

    let unit = rx.recv().unwrap().unwrap();
    println!("{:?}", unit);

    // let unit = AVAudioUnit::new_with_component_description(desc, Default::default(), block);

    // let unit = AVAudioUnit::new_with_component_description_tx(desc, Default::default(), &tx);
    // let unit = AVAudioUnitMIDIInstrument::new_with_audio_component_description(desc);

    use cocoa_foundation::foundation::{
        NSPoint,
        NSSize,
    };

    fn describe(rect: NSRect) -> String {
        let NSRect {
            origin: NSPoint { x, y },
            size: NSSize { width, height },
        } = rect;
        format!(
            "NSRect {{ x: {:?}, y: {:?}, w: {:?}, h: {:?}}}",
            x, y, width, height
        )
    }
    let (tx, rx) = std::sync::mpsc::channel();

    unit.au_audio_unit().request_view_controller_tx(&tx);

    let unit = rx.recv().unwrap();
    println!("{:?}", unit);

    // // unit.au_audio_unit().request_view_controller_tx(&tx);
    // unit.au_audio_unit().request_view_controller_tx(move |vc| {
    //     // let z = tx.send(avfoundation::AVFoundationEvent::RequestViewController(vc));
    //     // let vc = vc.unwrap();

    //     // let view: id = unsafe { msg_send![vc, view] };
    //     // let frame: NSRect = unsafe { msg_send![view, frame] };
    //     // let bounds: NSRect = unsafe { msg_send![view, bounds] };

    //     // println!(
    //     //     "request view controller bounds {:?} frame {:?}",
    //     //     describe(bounds),
    //     //     describe(frame)
    //     // );
    // });
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
