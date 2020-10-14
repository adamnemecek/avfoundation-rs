use crate::AudioNodeRef;
use objc::runtime::{BOOL, NO, YES};

pub enum AudioEngineManualRenderingMode {
    Offline,
    RealTime,
}
pub enum AVAudioEngine {}

foreign_obj_type! {
    type CType = AVAudioEngine;
    pub struct AudioEngine;
    pub struct AudioEngineRef;
}

impl AudioEngine {
    pub fn new() -> Self {
        unsafe {
            let class = class!(AVAudioEngine);
            msg_send![class, new]
        }
    }
}

impl AudioEngineRef {
    pub fn attach(&self, node: &AudioNodeRef) {
        unsafe { msg_send![self, attach: node] }
    }

    pub fn detach(&self, node: &AudioNodeRef) {
        unsafe { msg_send![self, detach: node] }
    }

    // pub fn connect(&self, node1: &AudioNodeRef, to node2: &AudioNodeRef, fromBus bus1: &AudioNodeBusRef, toBus bus2: &AudioNodeBusRef, format: AVAudioFormat?) {
    //    unsafe {
    //      msg_send![self, connect]
    //    }
    //}
    // pub fn connect(&self, node1: &AudioNodeRef, to node2: &AudioNodeRef, format: AVAudioFormat?) {
    //    unsafe {
    //      msg_send![self, connect]
    //    }
    //}
    // pub fn connect(&self, sourceNode: &AudioNodeRef, to destNodes: [AVAudioConnectionPoint], fromBus sourceBus: &AudioNodeBusRef, format: AVAudioFormat?) {
    //    unsafe {
    //      msg_send![self, connect]
    //    }
    //}
    // pub fn disconnect_node_input(&self, node: &AudioNodeRef, bus: &AudioNodeBusRef) {
    //    unsafe {
    //      msg_send![self, disconnectNodeInput]
    //    }
    //}
    // pub fn disconnect_node_input(&self, node: &AudioNodeRef) {
    //    unsafe {
    //      msg_send![self, disconnectNodeInput]
    //    }
    //}
    // pub fn disconnect_node_output(&self, node: &AudioNodeRef, bus: &AudioNodeBusRef) {
    //    unsafe {
    //      msg_send![self, disconnectNodeOutput]
    //    }
    //}

    pub fn disconnect_node_output(&self, node: &AudioNodeRef) {
        unsafe { msg_send![self, disconnectNodeOutput: node] }
    }

    pub fn prepare(&self) {
        unsafe { msg_send![self, prepare] }
    }

    // throws
    pub fn start(&self) {
        unsafe { msg_send![self, start] }
    }

    pub fn pause(&self) {
        unsafe { msg_send![self, pause] }
    }

    pub fn reset(&self) {
        unsafe { msg_send![self, reset] }
    }

    pub fn stop(&self) {
        unsafe { msg_send![self, stop] }
    }

    // pub fn input_connection_point(for node: &AudioNodeRef, inputBus bus: &AudioNodeBusRef) -> AVAudioConnectionPoint? {
    //    unsafe {
    //      msg_send![self, inputConnectionPoint]
    //    }
    //}
    // pub fn output_connection_points(for node: &AudioNodeRef, outputBus bus: &AudioNodeBusRef) -> [AVAudioConnectionPoint] {
    //    unsafe {
    //      msg_send![self, outputConnectionPoints]
    //    }
    //}

    // open var musicSequence: MusicSequence?

    //     @available(OSX 10.10, *)
    // open var inputNode: AVAudioInputNode { get }
    // open var mainMixerNode: AVAudioMixerNode { get }
    pub fn is_running(&self) -> bool {
        unsafe {
            match msg_send![self, isRunning] {
                YES => true,
                NO => false,
                _ => unreachable!(),
            }
        }
    }

    // open var isAutoShutdownEnabled: Bool
    pub fn is_auto_shutdown_enabled(&self) -> bool {
        unsafe {
            match msg_send![self, isAutoShutdownEnabled] {
                YES => true,
                NO => false,
                _ => unreachable!(),
            }
        }
    }

    pub fn set_auto_shutdown_enabled(&self, value: bool) {
        todo!()
    }
    // open var attachedNodes: Set<&AudioNodeRef> { get }

    // pub fn enable_manual_rendering_mode(&self, mode: AVAudioEngineManualRenderingMode, format pcmFormat: AVAudioFormat, maximumFrameCount: AVAudioFrameCount) throws {
    //    unsafe {
    //      msg_send![self, enableManualRenderingMode]
    //    }
    //}

    pub fn disable_manual_rendering_mode(&self) {
        unsafe { msg_send![self, disableManualRenderingMode] }
    }

    // pub fn render_offline(&self, numberOfFrames: AVAudioFrameCount, to buffer: AVAudioPCMBuffer) throws -> AVAudioEngineManualRenderingStatus {
    //    unsafe {
    //      msg_send![self, renderOffline]
    //    }
    //}

    // @available(OSX 10.13, *)
    // open var manualRenderingBlock: AVAudioEngineManualRenderingBlock { get }

    // @available(OSX 10.13, *)
    // open var isInManualRenderingMode: Bool { get }
    pub fn is_in_manual_rendering_mode(&self) -> bool {
        unsafe {
            match msg_send![self, isInManualRenderingMode] {
                YES => true,
                NO => false,
                _ => unreachable!(),
            }
        }
    }

    // @available(OSX 10.13, *)
    // open var manualRenderingMode: AVAudioEngineManualRenderingMode { get }

    // @available(OSX 10.13, *)
    // open var manualRenderingFormat: AVAudioFormat { get }
    // pub fn manualRenderingFormat(&self) -> bool {
    //     unsafe {
    //         let res: BOOL = msg_send![self, manualRenderingFormat];
    //         match res {
    //             YES => true,
    //             NO => false,
    //             _ => unreachable!(),
    //         }
    //     }
    // }

    // @available(OSX 10.13, *)
    // open var manualRenderingMaximumFrameCount: AVAudioFrameCount { get }

    // @available(OSX 10.13, *)
    // open var manualRenderingSampleTime: AVAudioFramePosition { get }

    // pub fn connect_midi(&self, sourceNode: &AudioNodeRef, to destinationNode: &AudioNodeRef, format: AVAudioFormat?, block tapBlock: AUMIDIOutputEventBlock? = nil) {
    //    unsafe {
    //      msg_send![self, connectMidi]
    //    }
    //}
    // pub fn connect_midi(&self, sourceNode: &AudioNodeRef, to destinationNodes: [&AudioNodeRef], format: AVAudioFormat?, block tapBlock: AUMIDIOutputEventBlock? = nil) {
    //    unsafe {
    //      msg_send![self, connectMidi]
    //    }
    //}
    // pub fn disconnect_midi(&self, sourceNode: &AudioNodeRef, from destinationNode: &AudioNodeRef) {
    //    unsafe {
    //      msg_send![self, disconnectMidi]
    //    }
    //}
    // pub fn disconnect_midi(&self, sourceNode: &AudioNodeRef, from destinationNodes: [&AudioNodeRef]) {
    //    unsafe {
    //      msg_send![self, disconnectMidi]
    //    }
    //}

    pub fn disconnect_midi_input(&self, node: &AudioNodeRef) {
        unsafe { msg_send![self, disconnectMidiInput: node] }
    }

    pub fn disconnect_midi_output(&self, node: &AudioNodeRef) {
        unsafe { msg_send![self, disconnectMidiOutput: node] }
    }
}
