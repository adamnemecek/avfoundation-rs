use crate::AVAudioNodeRef;
use objc::runtime::{
    NO,
    YES,
};

pub enum AudioEngineManualRenderingMode {
    Offline,
    RealTime,
}

pub enum AVAudioEngineFFI {}

foreign_obj_type! {
    type CType = AVAudioEngineFFI;
    pub struct AVAudioEngine;
    pub struct AVAudioEngineRef;
}

impl AVAudioEngine {
    pub fn new() -> Self {
        unsafe {
            let class = class!(AVAudioEngine);
            msg_send![class, new]
        }
    }
}

impl AVAudioEngineRef {
    pub fn attach(&self, node: &AVAudioNodeRef) {
        unsafe { msg_send![self, attach: node] }
    }

    pub fn detach(&self, node: &AVAudioNodeRef) {
        unsafe { msg_send![self, detach: node] }
    }

    // pub fn connect(&self, node1: &AVAudioNodeRef, to node2: &AVAudioNodeRef, fromBus bus1: &AudioNodeBusRef, toBus bus2: &AudioNodeBusRef, format: AVAudioFormat?) {
    //    unsafe {
    //      msg_send![self, connect]
    //    }
    //}
    // pub fn connect(&self, node1: &AVAudioNodeRef, to node2: &AVAudioNodeRef, format: AVAudioFormat?) {
    //    unsafe {
    //      msg_send![self, connect]
    //    }
    //}
    // pub fn connect(&self, sourceNode: &AVAudioNodeRef, to destNodes: [AVAudioConnectionPoint], fromBus sourceBus: &AudioNodeBusRef, format: AVAudioFormat?) {
    //    unsafe {
    //      msg_send![self, connect]
    //    }
    //}
    // pub fn disconnect_node_input(&self, node: &AVAudioNodeRef, bus: &AudioNodeBusRef) {
    //    unsafe {
    //      msg_send![self, disconnectNodeInput]
    //    }
    //}
    // pub fn disconnect_node_input(&self, node: &AVAudioNodeRef) {
    //    unsafe {
    //      msg_send![self, disconnectNodeInput]
    //    }
    //}
    // pub fn disconnect_node_output(&self, node: &AVAudioNodeRef, bus: &AudioNodeBusRef) {
    //    unsafe {
    //      msg_send![self, disconnectNodeOutput]
    //    }
    //}

    pub fn disconnect_node_output(&self, node: &AVAudioNodeRef) {
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

    // pub fn input_connection_point(for node: &AVAudioNodeRef, inputBus bus: &AudioNodeBusRef) -> AVAudioConnectionPoint? {
    //    unsafe {
    //      msg_send![self, inputConnectionPoint]
    //    }
    //}
    // pub fn output_connection_points(for node: &AVAudioNodeRef, outputBus bus: &AudioNodeBusRef) -> [AVAudioConnectionPoint] {
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
    // open var attachedNodes: Set<&AVAudioNodeRef> { get }

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

    // pub fn connect_midi(&self, sourceNode: &AVAudioNodeRef, to destinationNode: &AVAudioNodeRef, format: AVAudioFormat?, block tapBlock: AUMIDIOutputEventBlock? = nil) {
    //    unsafe {
    //      msg_send![self, connectMidi]
    //    }
    //}
    // pub fn connect_midi(&self, sourceNode: &AVAudioNodeRef, to destinationNodes: [&AVAudioNodeRef], format: AVAudioFormat?, block tapBlock: AUMIDIOutputEventBlock? = nil) {
    //    unsafe {
    //      msg_send![self, connectMidi]
    //    }
    //}
    // pub fn disconnect_midi(&self, sourceNode: &AVAudioNodeRef, from destinationNode: &AVAudioNodeRef) {
    //    unsafe {
    //      msg_send![self, disconnectMidi]
    //    }
    //}
    // pub fn disconnect_midi(&self, sourceNode: &AVAudioNodeRef, from destinationNodes: [&AVAudioNodeRef]) {
    //    unsafe {
    //      msg_send![self, disconnectMidi]
    //    }
    //}

    pub fn disconnect_midi_input(&self, node: &AVAudioNodeRef) {
        unsafe { msg_send![self, disconnectMidiInput: node] }
    }

    pub fn disconnect_midi_output(&self, node: &AVAudioNodeRef) {
        unsafe { msg_send![self, disconnectMidiOutput: node] }
    }
}
