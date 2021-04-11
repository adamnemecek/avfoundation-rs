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
    let components = manager.components_passing_test(|unit| {
        if unit.name().contains("DLS") {
            (true, ShouldStop::Stop)
        } else {
            (false, ShouldStop::Continue)
        }
    });

    let desc = components.first().unwrap().audio_component_description();

    let engine = AVAudioEngine::new();

    // println!("{:?}", components.first());

    // let midi = AVAudioUnitMIDIInstrument::new_with_audio_component_description(desc);

    let (tx, rx) = std::sync::mpsc::channel();
    // let (tx1, rx1) = std::sync::mpsc::channel();

    let block =
        block::ConcreteBlock::new(move |unit: *mut AVAudioUnitRef, error: *mut NSErrorRef| {
            let tx1 = tx.clone();
            println!("here");
            let res = unsafe {
                if error.is_null() {
                    let a = unit.as_ref().unwrap().to_owned();
                    Ok(a)
                } else {
                    Err(error.as_ref().unwrap().to_owned())
                }
            };

            match res {
                Ok(unit) => {
                    let ui_block = block::ConcreteBlock::new(
                        move |id: Option<&avfoundation::NSViewControllerRef>| {
                            // println!("ui block");
                            let vc = id.map(|x| x.to_owned());
                            let _ = tx1.send(vc);
                        },
                    )
                    .copy();

                    unit.au_audio_unit().request_view_controller(ui_block);
                }
                Err(e) => {}
            }

            // match res {
            //     Ok(unit) => {
            //         unit.au_audio_unit().request_view_controller(
            //             block::ConcreteBlock::new(move |a| {
            //                 tx1.send(1);
            //             })
            //             .copy(),
            //         );
            //     }
            //     Err(_) => todo!(),
            // }
            //     let desc = unsafe {
            //         let cls: id = msg_send![unit, class];
            //         let desc: id = msg_send![cls, description];
            //         nsstring_as_str(desc.as_ref().unwrap())
            //     };
            //     println!("callback {:?} {:?}", desc, err.localized_description());
            //     let ui_block =
            //         block::ConcreteBlock::new(move |id: &avfoundation::NSViewControllerRef| {
            //             println!("ui block");
            //         })
            //         .copy();
            //     unit.au_audio_unit().request_view_controller(ui_block);
        })
        .copy();

    let unit = AVAudioUnit::new_with_component_description(desc, Default::default(), block);
    // let unit =
    //     AVAudioUnit::new_with_component_description_fn(desc, Default::default(), |a| match a {
    //         Ok(unit) => {
    //             tx1.send(1);
    //         }
    //         Err(_) => {}
    //     });

    let a = rx.recv().unwrap();
    println!("received {:?}", a);
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

    // run_main_loop();
}
