use avfoundation::prelude::*;
use chromagear::Timestamp;
use kern_return::KERN_SUCCESS;
use mach::{
    clock_types::NSEC_PER_SEC,
    kern_return,
};
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

// struct MachTimebase {
//     inner: mach::mach_time::mach_timebase_info,
// }

// impl MachTimebase {
//     pub fn current() -> Self {
//         let mut inner = std::mem::MaybeUninit::<mach::mach_time::mach_timebase_info>::uninit();
//         let res = unsafe { mach::mach_time::mach_timebase_info(inner.as_mut_ptr()) };
//         if res == mach::kern_return::KERN_SUCCESS {
//             Self {
//                 inner: unsafe { inner.assume_init() },
//             }
//         } else {
//             todo!()
//         }
//     }

//     #[inline]
//     pub fn numer(&self) -> u32 {
//         self.inner.numer
//     }

//     #[inline]
//     pub fn denom(&self) -> u32 {
//         self.inner.denom
//     }
// }

// // const NSEC_PER_SEC: u32 = 1000000000;

// // use mach::
// // adapted from mikmidi midi to audio example
// fn midi_time_range(timestamp: &AudioTimeStamp, frames: f64) -> (f64, f64) {
//     let nsec_per_sec = NSEC_PER_SEC as f64;
//     let sample_rate = 44100.0;
//     let start_time_interval = timestamp.m_sample_time / sample_rate;
//     let duration = frames / sample_rate;

//     let midi_start = start_time_interval * nsec_per_sec;
//     let midi_end = midi_start + duration * nsec_per_sec;
//     (midi_start, midi_end)
// }

// NSTimeInterval startTimeInterval = inTimeStamp->mSampleTime / 44100.0;
// NSTimeInterval duration = inNumberFrames / 44100.0;

// static mach_timebase_info_data_t timebaseInfo = {0,0};
// if (timebaseInfo.denom == 0) mach_timebase_info(&timebaseInfo);
// uint64_t midiStartTime = startTimeInterval * NSEC_PER_SEC * timebaseInfo.denom / timebaseInfo.numer;
// uint64_t midiEndTime = midiStartTime + duration * NSEC_PER_SEC;

// NSMutableArray *commands = [NSMutableArray array];
// for (MIKMIDICommand *command in self.allNoteMessages) {
//     if (midiStartTime > command.midiTimestamp) continue;
//     if (midiEndTime <= command.midiTimestamp) break;

//     [commands addObject:command];
// }

// if ([self.midiClock musicTimeStampForMIDITimeStamp:midiEndTime] >= self.maxTrackLength)
//     self->_doneRendering = YES;

// for (MIKMIDINoteOnCommand *command in commands) {
//     NSLog(@"channel: %i status: %x (%@)", command.channel, (unsigned int)command.statusByte, command.data);
//     error = MusicDeviceMIDIEvent(self.instrumentUnit, command.statusByte, command.dataByte1, command.dataByte2, 0);
//     if (error) return error;
// }
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct F64(f64);

impl PartialEq for F64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for F64 {}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MIDIEvent {
    pub timestamp: F64,
    pub data: [u8; 3],
}

enum MIDIEventKind {
    On,
    Off,
}

impl MIDIEvent {
    fn kind(&self) -> MIDIEventKind {
        if self.data[0] == 0x90 || self.data[2] > 0 {
            MIDIEventKind::On
        } else {
            MIDIEventKind::Off
        }
    }
}

impl std::fmt::Debug for MIDIEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind() {
            MIDIEventKind::On => {
                write!(
                    f,
                    "MIDIEvent {{ kind: On, note: {}, velocity: {}}}",
                    self.data[1], self.data[2]
                )
            }
            MIDIEventKind::Off => {
                write!(
                    f,
                    "MIDIEvent {{ kind: Off, note: {}, velocity: {}}}",
                    self.data[1], self.data[2]
                )
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ScheduledEvent {
    // in sapmle time
    sample_ts: f64,
    data: [u8; 3],
}

fn beats_to_sample(b: f64) -> f64 {
    let bpm: f64 = 120.0;
    let sample_rate = 44100.0;
    // return (int)(beat / settings.tempo * 60 * sampleRate);
    b / bpm * 60.0 * sample_rate
}

#[derive(Clone, Copy, PartialEq)]
pub struct MIDINote {
    pitch: u8,
    // in beats
    start: f64,
    end: f64,
}

impl MIDINote {
    pub fn new(pitch: u8, start: f64, len: f64) -> Self {
        assert!(pitch < 128);
        Self {
            pitch,
            start,
            end: start + len,
        }
    }

    fn on(&self) -> ScheduledEvent {
        // let data = [0x90 | channel, note, velocity];
        let data = [0x90, self.pitch, 100];
        ScheduledEvent {
            sample_ts: beats_to_sample(self.start),
            data,
        }
    }

    fn off(&self) -> ScheduledEvent {
        // let data = [0x80 | channel, note, velocity];
        let data = [0x80, self.pitch, 0];
        ScheduledEvent {
            sample_ts: beats_to_sample(self.end),
            data,
        }
    }
}

fn push(v: &mut Vec<ScheduledEvent>, e: MIDINote) {
    let on = e.on();
}

// struct TimeSliceIterator<'a> {
//     start: usize,
//     inner: &'a [ScheduledEvent]
// }

// impl<'a> TimeSliceIterator<'a> {
//     pub fn new(start: usize, inner: &'a [ScheduledEvent], range: std::ops::Range<>) -> Self {
//         todo!()
//     }
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
    AVAudioUnit::new_with_component_description_fn(desc, Default::default(), move |result| {
        let _ = tx.send(result);
    });

    use unsafe_ref::prelude::*;

    let res = rx.recv().unwrap();
    // let notes: Vec<MIDIEvent> = vec![];

    // let notes = vec![MIDINote::new(30, 1.0, 1.0), MIDINote::new(50, 2.0, 1.0), ]
    let mut notes = vec![];
    let mut ts = 0.0;
    let mut pitch = 40;
    for e in 0..60 {
        notes.push(MIDINote::new(pitch, ts, 1.0));
        pitch += 1;
        ts += 0.5;
    }

    let mut events = vec![];

    for note in notes {
        events.push(note.on());
        events.push(note.off());
    }

    events.sort_by(|a, b| a.sample_ts.partial_cmp(&b.sample_ts).unwrap());

    // let max_timestamp = notes.iter().max(|x| a.timestamp);
    let mut i = 0;
    // let slice = notes.unsafe_slice();

    let mut running = false;
    let mut running_ptr: *mut bool = &mut running;

    println!("{:?}", events);

    // let mut

    // unit.au_audio_unit().token_by_adding_render_observer_fn(
    //     move |flags, ts, frame_count, bus| {
    //         // if the offset ts is before requested ts, we have to increment the counter
    //         // scheduling events that are in the requested range, until we find
    //         // an event that is past the requested timestamp
    //         // * if the requested timestamp is past the max timestamp, we set the running_ptr
    //         // to false
    //         //
    //         if !flags.contains(AudioUnitRenderActionFlags::PRE_RENDER) {
    //             return;
    //         }

    //         if !unsafe { *running_ptr } {
    //             return;
    //         }
    //         // let (start, end) = midi_time_range(ts, frame_count as _);

    //         loop {
    //             let event = &slice[i];

    //             // if
    //             unsafe {
    //                 *running_ptr = false;
    //             }
    //             if event.timestamp.inner > end {
    //                 break;
    //             }
    //             midi_fn(AUEventSampleTime::immediate(), 0, &event.data);
    //             i += 1;
    //             // let bytes = [0x90, note, 100];
    //         }
    //     },
    // )

    let mut i = 0;
    let token = match res {
        Ok(unit) => {
            engine.attach_node(&unit);
            engine.connect_nodes(&unit, engine.output_node(), None);

            let midi_fn = unit.au_audio_unit().schedule_midi_event_fn().unwrap();
            // for scheduling
            let param_fn = unit.au_audio_unit().schedule_parameter_fn();

            let token = unit.au_audio_unit().token_by_adding_render_observer_fn(
                move |flags, ts, frame_count, bus| {
                    if !flags.contains(AudioUnitRenderActionFlags::PRE_RENDER) {
                        return;
                    }

                    // these are in sample time
                    let start = ts.m_sample_time;
                    // let end = start + frame_count;
                    let end = start + (frame_count.0 as f64);

                    println!("ts {:?}", ts);
                    // println!("start {} end {}", start, end);

                    if i < events.len() {
                        let iter = &mut events[i..].iter();

                        loop {
                            if let Some(e) = iter.next() {
                                if e.sample_ts > end {
                                    break;
                                }
                                // let delta =
                                midi_fn(AUEventSampleTime::immediate(), 0, &e.data);
                                i += 1;
                            } else {
                                break;
                            }
                        }
                    } else {
                        ()
                    };
                    // if the offset ts is before requested ts, we have to increment the counter
                    // scheduling events that are in the requested range, until we find
                    // an event that is past the requested timestamp
                    // * if the requested timestamp is past the max timestamp, we set the running_ptr
                    // to false
                    //
                    // if !flags.contains(AudioUnitRenderActionFlags::PRE_RENDER) {
                    //     return;
                    // }

                    // if !unsafe { *running_ptr } {
                    //     return;
                    // }
                    // // let (start, end) = midi_time_range(ts, frame_count as _);

                    // loop {
                    //     let event = &slice[i];

                    //     // if
                    //     unsafe {
                    //         *running_ptr = false;
                    //     }
                    //     if event.timestamp.inner > end {
                    //         break;
                    //     }
                    //     midi_fn(AUEventSampleTime::immediate(), 0, &event.data);
                    //     i += 1;
                    //     // let bytes = [0x90, note, 100];
                    // }
                },
            );

            let _ = engine.start().unwrap();
        }
        Err(err) => {
            panic!("{:?}", err)
        }
    };

    run_main_loop();
}
