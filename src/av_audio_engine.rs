use crate::{
    AVAudioConnectionPoint,
    AVAudioConnectionPointRef,
    AVAudioFormatRef,
    AVAudioFrameCount,
    AVAudioInputNodeRef,
    AVAudioMixerNodeRef,
    AVAudioNodeBus,
    AVAudioNodeRef,
    AVAudioOutputNodeRef,
    AVAudioPCMBuffer,
};

pub struct MusicSequenceRef {}
use objc::runtime::{
    Object,
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
    Offline = 0,
    RealTime = 1,
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

    pub fn attach_node(&self, node: &AVAudioNodeRef) {
        unsafe { msg_send![self, attachNode: node] }
    }

    ///    @method detachNode:
    ///    @abstract
    ///        Detach a node previously attached to the engine.
    ///
    ///    If necessary, the engine will safely disconnect the node before detaching it.

    pub fn detach_node(&self, node: &AVAudioNodeRef) {
        unsafe { msg_send![self, detachNode: node] }
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

    pub fn connect(
        &self,
        from: &AVAudioNodeRef,
        to: &AVAudioNodeRef,
        from_bus: AVAudioNodeBus,
        to_bus: AVAudioNodeBus,
        format: Option<&AVAudioFormatRef>,
    ) {
        unsafe {
            msg_send![self, connect: from to: to fromBus: from_bus toBus: to_bus format: format]
        }
    }

    ///    @method connect:to:format:
    ///    @abstract
    ///        Establish a connection between two nodes
    ///
    ///    This calls connect:to:fromBus:toBus:format: using bus 0 on the source node,
    ///    and bus 0 on the destination node, except in the case of a destination which is a mixer,
    ///    in which case the destination is the mixer's nextAvailableInputBus.

    // - (void)connect:(AVAudioNode *)node1 to:(AVAudioNode *)node2 format:(AVAudioFormat * __nullable)format;
    pub fn connect_nodes(
        &self,
        node1: &AVAudioNodeRef,
        node2: &AVAudioNodeRef,
        format: Option<&AVAudioFormatRef>,
    ) {
        // let format = format.map
        use cocoa_foundation::base::nil;
        assert!(format.is_none());
        unsafe {
            let _: () = msg_send![self, connect: node1 to: node2 format: nil];
        }
    }

    ///    @method connect:toConnectionPoints:fromBus:format:
    ///    @abstract
    ///        Establish connections between a source node and multiple destination nodes.
    ///    @param sourceNode
    ///        The source node
    ///    @param destNodes
    ///        An array of AVAudioConnectionPoint objects specifying destination
    ///        nodes and busses
    ///    @param sourceBus
    ///        The output bus on source node
    ///    @param format
    ///        If non-nil, the format of the source node's output bus is set to this
    ///        format. In all cases, the format of the destination nodes' input bus is set to
    ///        match that of the source node's output bus
    ///
    ///    Use this method to establish connections from a source node to multiple destination nodes.
    ///    Connections made using this method are either one-to-one (when a single destination
    ///    connection is specified) or one-to-many (when multiple connections are specified), but
    ///    never many-to-one.
    ///
    ///    To incrementally add a new connection to a source node, use this method with an array
    ///    of AVAudioConnectionPoint objects comprising of pre-existing connections (obtained from
    ///    `outputConnectionPointsForNode:outputBus:`) and the new connection.
    ///
    ///    Note that any pre-existing connection involving the destination's input bus will be
    ///    broken. And, any pre-existing connection on source node which is not a part of the
    ///    specified destination connection array will also be broken.
    ///
    ///    Also note that when the output of a node is split into multiple paths, all the paths
    ///    must render at the same rate until they reach a common mixer.
    ///    In other words, starting from the split node until the common mixer node where all split
    ///    paths terminate, you cannot have:
    ///        - any AVAudioUnitTimeEffect
    ///        - any sample rate conversion

    // fn dummy(&self) {}
    // - (void)connect:(AVAudioNode *)sourceNode toConnectionPoints:(NSArray<AVAudioConnectionPoint *> *)destNodes fromBus:(AVAudioNodeBus)sourceBus format:(AVAudioFormat * __nullable)format API_AVAILABLE(macos(10.11), ios(9.0), watchos(2.0), tvos(9.0));
    // pub fn connect(&self, sourceNode: &AVAudioNodeRef, to destNodes: [AVAudioConnectionPoint], fromBus sourceBus: AVAudioNodeBus, format: AVAudioFormat?) {
    //    unsafe {
    //      msg_send![self, connect]
    //    }
    //}
    // pub fn disconnect_node_input(&self, node: &AVAudioNodeRef, bus: AVAudioNodeBus) {
    //    unsafe {
    //      msg_send![self, disconnectNodeInput]
    //    }
    //}
    // pub fn disconnect_node_input(&self, node: &AVAudioNodeRef) {
    //    unsafe {
    //      msg_send![self, disconnectNodeInput]
    //    }
    //}
    // pub fn disconnect_node_output(&self, node: &AVAudioNodeRef, bus: AVAudioNodeBus) {
    //    unsafe {
    //      msg_send![self, disconnectNodeOutput]
    //    }
    //}

    pub fn disconnect_node_output(&self, node: &AVAudioNodeRef) {
        unsafe { msg_send![self, disconnectNodeOutput: node] }
    }

    ///    @method prepare
    ///    @abstract
    ///        Prepare the engine for starting.
    ///
    ///    This method preallocates many of the resources the engine requires in order to start.
    ///    It can be used to be able to start more responsively.
    pub fn prepare(&self) {
        unsafe { msg_send![self, prepare] }
    }

    // throws
    pub fn start_and_return_error(&self) -> bool {
        use cocoa_foundation::base::nil;
        unsafe {
            let mut error = nil;
            match msg_send![self, startAndReturnError: &error] {
                YES => true,
                NO => false,
                _ => unreachable!(),
            }
        }
    }

    ///    @method pause
    ///    @abstract
    ///        Pause the engine.
    ///
    ///    When the engine is rendering to/from an audio device, stops the audio hardware and the flow
    ///    of audio through the engine. When operating in this mode, it is recommended that the engine
    ///    be paused or stopped (as applicable) when not in use, to minimize power consumption.
    ///
    ///    Pausing the engine does not deallocate the resources allocated by prepare. Resume the
    ///    engine by invoking start again.
    pub fn pause(&self) {
        unsafe { msg_send![self, pause] }
    }

    ///    @method reset
    ///    @abstract reset
    ///        Reset all of the nodes in the engine.
    ///
    ///    This will reset all of the nodes in the engine. This is useful, for example, for silencing
    ///    reverb and delay tails.
    ///
    ///    In manual rendering mode, the render timeline is reset to a sample time of zero.
    pub fn reset(&self) {
        unsafe { msg_send![self, reset] }
    }

    ///    @method stop
    ///    @abstract
    ///        When the engine is rendering to/from an audio device, stops the audio hardware and the
    ///        engine. When operating in this mode, it is recommended that the engine be paused or stopped
    ///         (as applicable) when not in use, to minimize power consumption.
    ///
    ///        Stopping the engine releases the resources allocated by prepare.
    pub fn stop(&self) {
        unsafe { msg_send![self, stop] }
    }

    ///    @method inputConnectionPointForNode:inputBus:
    ///    @abstract
    ///        Get connection information on a node's input bus.
    ///    @param node
    ///        The node whose input connection is being queried.
    ///    @param bus
    ///        The node's input bus on which the connection is being queried.
    ///    @return
    ///        An AVAudioConnectionPoint object with connection information on the node's
    ///        specified input bus.
    ///
    ///    Connections are always one-to-one or one-to-many, never many-to-one.
    ///
    ///    Returns nil if there is no connection on the node's specified input bus.

    pub fn input_connection_point_for_node(
        &self,
        node: &AVAudioNodeRef,
        input_bus: AVAudioNodeBus,
    ) -> Option<&AVAudioConnectionPointRef> {
        unsafe { msg_send![self, inputConnectionPointForNode: node inputBus: input_bus] }
    }

    ///    @method outputConnectionPointsForNode:outputBus:
    ///    @abstract
    ///        Get connection information on a node's output bus.
    ///    @param node
    ///        The node whose output connections are being queried.
    ///    @param bus
    ///        The node's output bus on which connections are being queried.
    ///    @return
    ///        An array of AVAudioConnectionPoint objects with connection information on the node's
    ///        specified output bus.
    ///
    ///    Connections are always one-to-one or one-to-many, never many-to-one.
    ///
    ///
    pub fn output_connection_points_for_node(
        &self,
        node: &AVAudioNodeRef,
        output_bus: AVAudioNodeBus,
    ) -> Vec<AVAudioConnectionPoint> {
        unsafe {
            let array: *const Object =
                msg_send![self, outputConnectionPointsForNode: node outputBus: output_bus];
            crate::nsarray_to_vec(array)
        }
    }

    // open var musicSequence: MusicSequence?
    pub fn music_sequence(&self) -> ! {
        todo!()
    }

    pub fn set_music_sequence(&self) -> ! {
        todo!()
    }

    /// /*! @property outputNode
    /// 	@abstract
    /// 		The engine's singleton output node.
    ///
    /// 	Audio output is performed via an output node. The engine creates a singleton on demand when
    /// 	this property is first accessed. Connect another node to the input of the output node, or
    /// 	obtain a mixer that is connected there by default, using the "mainMixerNode" property.
    ///
    /// 	When the engine is rendering to/from an audio device, the AVAudioSesssion category and/or
    /// 	availability of hardware determine whether an app can perform output. Check the output
    /// 	format of output node (i.e. hardware format) for non-zero sample rate and channel count to
    /// 	see if output is enabled.
    /// 	Trying to perform output through the output node when it is not enabled or available will
    /// 	cause the engine to throw an error (when possible) or an exception.
    ///
    /// 	In manual rendering mode, the output format of the output node will determine the
    /// 	render format of the engine. It can be changed through
    /// 	`enableManualRenderingMode:format:maximumFrameCount:error:`.
    /// */
    // todo: can this ever be nil?
    pub fn output_node(&self) -> &AVAudioOutputNodeRef {
        unsafe { msg_send![self, outputNode] }
    }

    /// /*! @property inputNode
    /// 	@abstract
    /// 		The engine's singleton input node.
    ///
    /// 	Audio input is performed via an input node. The engine creates a singleton on demand when
    /// 	this property is first accessed. To receive input, connect another node from the output of
    /// 	the input node, or create a recording tap on it.
    ///
    /// 	When the engine is rendering to/from an audio device, the AVAudioSesssion category and/or
    /// 	availability of hardware determine whether an app can perform input (e.g. input hardware is
    /// 	not available on tvos). Check for the input node's input format (i.e. hardware format) for
    /// 	non-zero sample rate and channel count to see if input is enabled.
    /// 	Trying to perform input through the input node when it is not enabled or available will
    /// 	cause the engine to throw an error (when possible) or an exception.
    ///
    /// 	In manual rendering mode, the input node can be used to synchronously supply data to
    /// 	the engine while it is rendering (see
    /// 	`AVAudioInputNode(setManualRenderingInputPCMFormat:inputBlock:)`.
    /// */
    pub fn input_node(&self) -> &AVAudioInputNodeRef {
        unsafe { msg_send![self, inputNode] }
    }

    //     @available(OSX 10.10, *)
    // open var inputNode: AVAudioInputNode { get }
    // open var mainMixerNode: AVAudioMixerNode { get }
    pub fn main_mixer_node(&self) -> Option<&AVAudioMixerNodeRef> {
        unsafe { msg_send![self, mainMixerNode] }
    }

    ///    @property running
    ///    @abstract
    ///        The engine's running state.
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
    pub fn attached_nodes(&self) -> ! {
        todo!()
    }

    // pub fn enable_manual_rendering_mode(&self, mode: AVAudioEngineManualRenderingMode, format pcmFormat: AVAudioFormat, maximumFrameCount: AVAudioFrameCount) throws {
    //    unsafe {
    //      msg_send![self, enableManualRenderingMode]
    //    }
    //}

    pub fn disable_manual_rendering_mode(&self) {
        unsafe { msg_send![self, disableManualRenderingMode] }
    }

    pub fn render_offline(
        &self,
        number_of_frames: AVAudioFrameCount,
        buffer: AVAudioPCMBuffer,
    ) -> AVAudioEngineManualRenderingStatus {
        //    unsafe {
        //      msg_send![self, renderOffline]
        //    }
        todo!()
    }

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
