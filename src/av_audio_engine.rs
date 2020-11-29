use crate::AVAudioNodeRef;
use objc::runtime::{
    NO,
    YES,
};

///    @enum AVAudioEngineManualRenderingError
///    @abstract
///        Error codes that could be returned from AVAudioEngine manual rendering mode methods,
///        e.g. `enableManualRenderingMode:format:maximumFrameCount:error:` and
///        `renderOffline:toBuffer:error:`.
///        Note that this is not a comprehensive list, and the underlying audio units could
///        return other error codes (e.g. see kAudioUnitErr_* in AudioToolbox/AUComponent.h) from these
///        methods as applicable.
///
///        AVAudioEngineManualRenderingErrorInvalidMode
///            The operation cannot be performed because the engine is either not in manual
///            rendering mode or the right variant of it.
///
///        AVAudioEngineManualRenderingErrorInitialized
///            The operation cannot be performed because the engine is initialized (i.e. not stopped).
///
///         AVAudioEngineManualRenderingErrorNotRunning
///            The operation cannot be performed because the engine is not running (i.e. not started).
// osstatus
// API_AVAILABLE(macos(10.13), ios(11.0), watchos(4.0), tvos(11.0));
pub enum AVAudioEngineManualRenderingError {
    InvalidMode = -80800,
    Initialized = -80801,
    NotRunning = -80802,
}

///    @enum AVAudioEngineManualRenderingStatus
///    @abstract
///        Status codes returned from the render call to the engine operating in manual rendering mode.
///
///        AVAudioEngineManualRenderingStatusError
///            An error occurred when rendering and no data was returned. See the returned error code
///            for the description of the error.
///
///        AVAudioEngineManualRenderingStatusSuccess
///            All of the requested data was returned successfully.
///
///        AVAudioEngineManualRenderingStatusInsufficientDataFromInputNode
///            Applicable only to the input node, when it provides input data for rendering
///            (see `AVAudioInputNode(setManualRenderingInputPCMFormat:inputBlock:)`).
///            Indicates that not enough input data was returned by the input node to satisfy the
///            render request at the current time. The output buffer may contain data rendered by other
///            active sources in the engine's processing graph.
///
///         AVAudioEngineManualRenderingStatusCannotDoInCurrentContext
///            The operation could not be performed now, but the client could retry later if needed.
///            This is usually to guard a realtime render operation (e.g. rendering through
///            `manualRenderingBlock`) when a reconfiguration of the engine's internal state
///            is in progress.

// osstatus
// API_AVAILABLE(macos(10.13), ios(11.0), watchos(4.0), tvos(11.0));
pub enum AVAudioEngineManualRenderingStatus {
    Error = -1,
    Success = 0,
    InsufficientDataFromInputNode = 1,
    CannotDoInCurrentContext = 2,
}

///    @enum AVAudioEngineManualRenderingMode
///    @abstract
///        By default, the engine is connected to an audio device and automatically renders in realtime.
///        It can also be configured to operate in manual rendering mode, i.e. not connected to an
///        audio device and rendering in response to requests from the client.
///
///        AVAudioEngineManualRenderingModeOffline
///            The engine operates in an offline mode without any realtime constraints.
///
///        AVAudioEngineManualRenderingModeRealtime
///            The engine operates under realtime constraints, i.e. it will not make any blocking call
///            (e.g. calling libdispatch, blocking on a mutex, allocating memory etc.) while rendering.
///            Note that only the block based render mechanism can be used in this mode
///            (see `AVAudioEngine(manualRenderingBlock)`.

#[repr(i64)]
pub enum AudioEngineManualRenderingMode {
    Offline,
    RealTime,
}

///    @typedef AVAudioEngineManualRenderingBlock
///    @abstract
///        Block to render the engine when operating in manual rendering mode
///    @param numberOfFrames
///        The number of PCM sample frames to be rendered
///    @param outBuffer
///        The PCM buffer to which the engine must render the audio.
///        The buffer pointers (outBuffer->mBuffers[x].mData) may be null on entry, in which case
///        the block will render into a memory it owns and modify the mData pointers to point to that
///        memory. The block is responsible for preserving the validity of that memory until it is next
///        called to render, or `AVAudioEngine(stop)` is called.
///    @param outError
///        On exit, if an error occurs during rendering, a description of the error (see
///        `AVAudioEngineManualRenderingError` for the possible errors)
///    @return
///        One of the status codes from `AVAudioEngineManualRenderingStatus`. Irrespective of the
///        returned status code, on exit, the output buffer's mDataByteSize
///        (outBuffer->mBuffers[x].mDataByteSize) will indicate the number of PCM data bytes rendered by
///        the engine.
///
///    Use this if you want to render the engine from a realtime context when it is operating in
///    the manual rendering mode. See `AVAudioEngine(manualRenderingBlock)` for details.
///
///    Note that when using AVAudioEngine manual rendering with
///    `AVAudioEngineManualRenderingModeRealtime`, calling into the engine or related classes
///    from a non-realtime thread (e.g. for setting or getting node properties), can cause
///    `AVAudioEngineManualRenderingBlock` on the IO thread to return
///    `AVAudioEngineManualRenderingStatusCannotDoInCurrentContext`.
///
///    This is because interacting with some of these properties requires synchronization between
///    the realtime and calling threads.
///
///    In such a case, the client could implement their own synchronization between their realtime
///    and non-realtime threads and retry calling `AVAudioEngineManualRenderingBlock`.

// API_AVAILABLE(macos(10.13), ios(11.0), watchos(4.0), tvos(11.0));
pub type AVAudioEngineManualRenderingBlock = block::Block<(), ()>;

///    @class AVAudioEngine
///    
///    An AVAudioEngine contains a group of connected AVAudioNodes ("nodes"), each of which performs
///    an audio signal generation, processing, or input/output task.
///    
///    Nodes are created separately and attached to the engine.
///    
///    The engine supports dynamic connection, disconnection and removal of nodes while running,
///    with only minor limitations:
///    - all dynamic reconnections must occur upstream of a mixer
///    - while removals of effects will normally result in the automatic connection of the adjacent
///        nodes, removal of a node which has differing input vs. output channel counts, or which
///        is a mixer, is likely to result in a broken graph.
///    
///    By default, the engine is connected to an audio device and automatically renders in realtime.
///    It can also be configured to operate in manual rendering mode, i.e. not connected to an
///    audio device and rendering in response to requests from the client, normally at or
///    faster than realtime rate.
pub enum AVAudioEngineFFI {}

foreign_obj_type! {
    type CType = AVAudioEngineFFI;
    pub struct AVAudioEngine;
    pub struct AVAudioEngineRef;
}

impl AVAudioEngine {
    ///    @method init
    ///    @abstract
    ///        Initialize a new engine.
    ///
    ///    On creation, the engine is by default connected to an audio device and automatically renders
    ///    in realtime. It can be configured to operate in manual rendering mode through
    ///    `enableManualRenderingMode:format:maximumFrameCount:error:`.

    pub fn new() -> Self {
        unsafe {
            let class = class!(AVAudioEngine);
            msg_send![class, new]
        }
    }
}

impl AVAudioEngineRef {
    ///    @method attachNode:
    ///    @abstract
    ///        Take ownership of a new node.
    ///    @param node
    ///        The node to be attached to the engine.
    ///
    ///    To support the instantiation of arbitrary AVAudioNode subclasses, instances are created
    ///    externally to the engine, but are not usable until they are attached to the engine via
    ///    this method. Thus the idiom, without ARC, is:
    ///
    ///    ```
    ///    // when building engine:
    ///    AVAudioNode *_player;    // member of controller class (for example)
    ///    ...
    ///    _player = [[AVAudioPlayerNode alloc] init];
    ///    [engine attachNode: _player];
    ///    ...
    ///    // when destroying engine (without ARC)
    ///    [_player release];
    ///    ```

    pub fn attach(&self, node: &AVAudioNodeRef) {
        unsafe { msg_send![self, attach: node] }
    }

    ///    @method detachNode:
    ///    @abstract
    ///        Detach a node previously attached to the engine.
    ///
    ///    If necessary, the engine will safely disconnect the node before detaching it.

    pub fn detach(&self, node: &AVAudioNodeRef) {
        unsafe { msg_send![self, detach: node] }
    }
    ///    @method connect:to:fromBus:toBus:format:
    ///    @abstract
    ///        Establish a connection between two nodes.
    ///    @param node1
    ///        The source node
    ///    @param node2
    ///        The destination node
    ///    @param bus1
    ///        The output bus on the source node
    ///    @param bus2
    ///        The input bus on the destination node
    ///    @param format
    ///        If non-nil, the format of the source node's output bus is set to this
    ///        format. In all cases, the format of the destination node's input bus is set to
    ///        match that of the source node's output bus.
    ///
    ///    Nodes have input and output buses (AVAudioNodeBus). Use this method to establish
    ///    one-to-one connections betweeen nodes. Connections made using this method are always
    ///    one-to-one, never one-to-many or many-to-one.
    ///
    ///    Note that any pre-existing connection(s) involving the source's output bus or the
    ///    destination's input bus will be broken.

    // - (void)connect:(AVAudioNode *)node1 to:(AVAudioNode *)node2 fromBus:(AVAudioNodeBus)bus1 toBus:(AVAudioNodeBus)bus2 format:(AVAudioFormat * __nullable)format;

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
