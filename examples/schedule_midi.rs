use avfoundation::prelude::*;
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

    let res = rx.recv().unwrap();
    match res {
        Ok(unit) => {
            engine.attach_node(&unit);
            engine.connect_nodes(&unit, engine.output_node(), None);
            let z = engine.start().unwrap();
            let midi_fn = unit.au_audio_unit().schedule_midi_fn();
            unit.au_audio_unit().token_by_adding_render_observer_2(
                move |flags, ts, frame_count, bus| {
                    if flags.contains(AudioUnitRenderActionFlags::PreRender) {
                        let bytes = [0x90, 100, 100];
                        midi_fn(AUEventSampleTime::immediate(), 0, &bytes);
                    }
                },
            );
        }
        Err(err) => {
            panic!("{:?}", err)
        }
    }

    run_main_loop();
}
