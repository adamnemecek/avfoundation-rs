use avfoundation::prelude::*;
use kern_return::KERN_SUCCESS;
use mach::kern_return;
// public func midi_scheduler_main() {

//     let engine = AVAudioEngine()
//     let component = AVAudioUnitComponentManager.shared().findComponent(name: "DLS")!
//     let desc = component.audioComponentDescription

//     AVAudioUnit.instantiate(with: desc, options: []) {(unit, err) in
//         let unit = unit!
//         engine.attach(unit)
//         engine.connect(unit, to: engine.outputNode, format: nil)
//         try! engine.start()
//         let bytes: [UInt8] = [0x90, 100, 100]
//         let block = unit.auAudioUnit.scheduleMIDIEventBlock!

//         unit.auAudioUnit.token(byAddingRenderObserver: {
//             (flags, timestamp, frameCount, outputBusNumber) in

//             if flags.contains(.unitRenderAction_PreRender) {
//                 bytes.withUnsafeBufferPointer{
//                     block(AUEventSampleTimeImmediate, 0, 3, $0.baseAddress!)
//                 }
//             }
//         })
//     }

//     RunLoop.main.run()

// }

struct MachTimebase {
    inner: mach::mach_time::mach_timebase_info,
}

impl MachTimebase {
    pub fn current() -> Self {
        let mut inner = std::mem::MaybeUninit::<mach::mach_time::mach_timebase_info>::uninit();
        let res = unsafe { mach::mach_time::mach_timebase_info(inner.as_mut_ptr()) };
        if res == mach::kern_return::KERN_SUCCESS {
            Self {
                inner: unsafe { inner.assume_init() },
            }
        } else {
            todo!()
        }
    }
}

fn main() {
    let engine = AVAudioEngine::new();
    let component = AVAudioUnitComponentManager::shared().components_passing_test(|x| {
        if x.name().contains("DLS") {
            (true, ShouldStop::Stop)
        } else {
            (false, ShouldStop::Continue)
        }
    });
    let desc = component.first().unwrap().audio_component_description();
    let (tx, rx) = std::sync::mpsc::channel();
    AVAudioUnit::new_with_component_description(desc, Default::default(), move |result| {
        let _ = tx.send(result);
    });

    use unsafe_slice::prelude::*;

    let res = rx.recv().unwrap();
    let notes: Vec<u8> = vec![50, 60, 70, 80, 90];
    let mut i = 0;
    let slice = notes.unsafe_slice();
    match res {
        Ok(unit) => {
            engine.attach_node(&unit);
            engine.connect_nodes(&unit, engine.output_node(), None);
            let _ = engine.start().unwrap();
            let midi_fn = unit.au_audio_unit().schedule_midi_fn().unwrap();
            let _token = unit.au_audio_unit().token_by_adding_render_observer_fn(
                move |flags, ts, frame_count, bus| {
                    let idx = i % slice.len();
                    if flags.contains(AudioUnitRenderActionFlags::PreRender) {
                        let note = slice[idx];
                        let bytes = [0x90, note, 100];
                        midi_fn(AUEventSampleTime::immediate(), 0, &bytes);
                    }
                    i += 1;
                },
            );
        }
        Err(err) => {
            panic!("{:?}", err)
        }
    }

    run_main_loop();
}
