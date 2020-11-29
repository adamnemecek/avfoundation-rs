/** @method init
    @abstract
        Initialize a new engine.
    @discussion
        On creation, the engine is by default connected to an audio device and automatically renders
        in realtime. It can be configured to operate in manual rendering mode through
        `enableManualRenderingMode:format:maximumFrameCount:error:`.
*/
public init()

/**    @method attachNode:
    @abstract
        Take ownership of a new node.
    @param node
        The node to be attached to the engine.
    @discussion
        To support the instantiation of arbitrary AVAudioNode subclasses, instances are created
        externally to the engine, but are not usable until they are attached to the engine via
        this method. Thus the idiom, without ARC, is:

<pre>
// when building engine:
AVAudioNode *_player;    // member of controller class (for example)
...
_player = [[AVAudioPlayerNode alloc] init];
[engine attachNode: _player];
...
// when destroying engine (without ARC)
[_player release];
</pre>
*/
open func attach(_ node: AVAudioNode)

/**    @method detachNode:
    @abstract
        Detach a node previously attached to the engine.
    @discussion
        If necessary, the engine will safely disconnect the node before detaching it.
*/
open func detach(_ node: AVAudioNode)

/** @method connect:to:fromBus:toBus:format:
    @abstract
        Establish a connection between two nodes.
    @param node1
        The source node
    @param node2
        The destination node
    @param bus1
        The output bus on the source node
    @param bus2
        The input bus on the destination node
    @param format
        If non-nil, the format of the source node's output bus is set to this
        format. In all cases, the format of the destination node's input bus is set to
        match that of the source node's output bus.
    @discussion
        Nodes have input and output buses (AVAudioNodeBus). Use this method to establish
        one-to-one connections betweeen nodes. Connections made using this method are always
        one-to-one, never one-to-many or many-to-one.

        Note that any pre-existing connection(s) involving the source's output bus or the
        destination's input bus will be broken.
*/
open func connect(_ node1: AVAudioNode, to node2: AVAudioNode, fromBus bus1: AVAudioNodeBus, toBus bus2: AVAudioNodeBus, format: AVAudioFormat?)

/**    @method connect:to:format:
    @abstract
        Establish a connection between two nodes
    @discussion
        This calls connect:to:fromBus:toBus:format: using bus 0 on the source node,
        and bus 0 on the destination node, except in the case of a destination which is a mixer,
        in which case the destination is the mixer's nextAvailableInputBus.
*/
open func connect(_ node1: AVAudioNode, to node2: AVAudioNode, format: AVAudioFormat?)

/** @method connect:toConnectionPoints:fromBus:format:
    @abstract
        Establish connections between a source node and multiple destination nodes.
    @param sourceNode
        The source node
    @param destNodes
        An array of AVAudioConnectionPoint objects specifying destination
        nodes and busses
    @param sourceBus
        The output bus on source node
    @param format
        If non-nil, the format of the source node's output bus is set to this
        format. In all cases, the format of the destination nodes' input bus is set to
        match that of the source node's output bus
    @discussion
        Use this method to establish connections from a source node to multiple destination nodes.
        Connections made using this method are either one-to-one (when a single destination
        connection is specified) or one-to-many (when multiple connections are specified), but
        never many-to-one.

        To incrementally add a new connection to a source node, use this method with an array
        of AVAudioConnectionPoint objects comprising of pre-existing connections (obtained from
        `outputConnectionPointsForNode:outputBus:`) and the new connection.

        Note that any pre-existing connection involving the destination's input bus will be
        broken. And, any pre-existing connection on source node which is not a part of the
        specified destination connection array will also be broken.

        Also note that when the output of a node is split into multiple paths, all the paths
        must render at the same rate until they reach a common mixer.
        In other words, starting from the split node until the common mixer node where all split
        paths terminate, you cannot have:
            - any AVAudioUnitTimeEffect
            - any sample rate conversion
*/
@available(OSX 10.11, *)
open func connect(_ sourceNode: AVAudioNode, to destNodes: [AVAudioConnectionPoint], fromBus sourceBus: AVAudioNodeBus, format: AVAudioFormat?)

/** @method disconnectNodeInput:bus:
    @abstract
        Remove a connection between two nodes.
    @param node
        The node whose input is to be disconnected
    @param bus
        The destination's input bus to disconnect
*/
open func disconnectNodeInput(_ node: AVAudioNode, bus: AVAudioNodeBus)

/**    @method disconnectNodeInput:
    @abstract
        Remove a connection between two nodes.
    @param node
        The node whose inputs are to be disconnected
    @discussion
        Connections are broken on each of the node's input busses.
*/
open func disconnectNodeInput(_ node: AVAudioNode)

/** @method disconnectNodeOutput:bus:
    @abstract
        Remove a connection between two nodes.
    @param node
        The node whose output is to be disconnected
    @param bus
        The source's output bus to disconnect
*/
open func disconnectNodeOutput(_ node: AVAudioNode, bus: AVAudioNodeBus)

/**    @method disconnectNodeOutput:
    @abstract
        Remove a connection between two nodes.
    @param node
        The node whose outputs are to be disconnected
    @discussion
        Connections are broken on each of the node's output busses.
*/
open func disconnectNodeOutput(_ node: AVAudioNode)

/**    @method prepare
    @abstract
        Prepare the engine for starting.
    @discussion
        This method preallocates many of the resources the engine requires in order to start.
        It can be used to be able to start more responsively.
*/
open func prepare()

/** @method startAndReturnError:
    @abstract
        Start the engine.
    @return
        YES for success
    @discussion
        Calls prepare if it has not already been called since stop.

        When the engine is rendering to/from an audio device, starts the audio hardware via the
        AVAudioInputNode and/or AVAudioOutputNode instances in the engine. Audio begins to flow
        through the engine.
        Reasons for potential failure to start in this mode include:
        1. There is problem in the structure of the graph. Input can't be routed to output or to a
            recording tap through converter type nodes.
        2. An AVAudioSession error.
        3. The driver failed to start the hardware.

        In manual rendering mode, prepares the engine to render when requested by the client.
*/
open func start() throws

/**    @method pause
    @abstract
        Pause the engine.
    @discussion
        When the engine is rendering to/from an audio device, stops the audio hardware and the flow
        of audio through the engine. When operating in this mode, it is recommended that the engine
        be paused or stopped (as applicable) when not in use, to minimize power consumption.

        Pausing the engine does not deallocate the resources allocated by prepare. Resume the
        engine by invoking start again.
*/
open func pause()

/**    @method reset
    @abstract reset
        Reset all of the nodes in the engine.
    @discussion
        This will reset all of the nodes in the engine. This is useful, for example, for silencing
        reverb and delay tails.

        In manual rendering mode, the render timeline is reset to a sample time of zero.
*/
open func reset()

/** @method stop
    @abstract
        When the engine is rendering to/from an audio device, stops the audio hardware and the
        engine. When operating in this mode, it is recommended that the engine be paused or stopped
         (as applicable) when not in use, to minimize power consumption.

        Stopping the engine releases the resources allocated by prepare.
*/
open func stop()

/** @method inputConnectionPointForNode:inputBus:
    @abstract
        Get connection information on a node's input bus.
    @param node
        The node whose input connection is being queried.
    @param bus
        The node's input bus on which the connection is being queried.
    @return
        An AVAudioConnectionPoint object with connection information on the node's
        specified input bus.
    @discussion
        Connections are always one-to-one or one-to-many, never many-to-one.

        Returns nil if there is no connection on the node's specified input bus.
*/
@available(OSX 10.11, *)
open func inputConnectionPoint(for node: AVAudioNode, inputBus bus: AVAudioNodeBus) -> AVAudioConnectionPoint?

/** @method outputConnectionPointsForNode:outputBus:
    @abstract
        Get connection information on a node's output bus.
    @param node
        The node whose output connections are being queried.
    @param bus
        The node's output bus on which connections are being queried.
    @return
        An array of AVAudioConnectionPoint objects with connection information on the node's
        specified output bus.
    @discussion
        Connections are always one-to-one or one-to-many, never many-to-one.

        Returns an empty array if there are no connections on the node's specified output bus.
*/
@available(OSX 10.11, *)
open func outputConnectionPoints(for node: AVAudioNode, outputBus bus: AVAudioNodeBus) -> [AVAudioConnectionPoint]

/** @property musicSequence
    @abstract
        The MusicSequence previously attached to the engine (if any).
 */
open var musicSequence: MusicSequence?

/** @property outputNode
    @abstract
        The engine's singleton output node.
    @discussion
        Audio output is performed via an output node. The engine creates a singleton on demand when
        this property is first accessed. Connect another node to the input of the output node, or
        obtain a mixer that is connected there by default, using the "mainMixerNode" property.

        When the engine is rendering to/from an audio device, the AVAudioSesssion category and/or
        availability of hardware determine whether an app can perform output. Check the output
        format of output node (i.e. hardware format) for non-zero sample rate and channel count to
        see if output is enabled.
        Trying to perform output through the output node when it is not enabled or available will
        cause the engine to throw an error (when possible) or an exception.

        In manual rendering mode, the output format of the output node will determine the
        render format of the engine. It can be changed through
        `enableManualRenderingMode:format:maximumFrameCount:error:`.
*/
open var outputNode: AVAudioOutputNode { get }

/** @property inputNode
    @abstract
        The engine's singleton input node.
    @discussion
        Audio input is performed via an input node. The engine creates a singleton on demand when
        this property is first accessed. To receive input, connect another node from the output of
        the input node, or create a recording tap on it.

        When the engine is rendering to/from an audio device, the AVAudioSesssion category and/or
        availability of hardware determine whether an app can perform input (e.g. input hardware is
        not available on tvos). Check for the input node's input format (i.e. hardware format) for
        non-zero sample rate and channel count to see if input is enabled.
        Trying to perform input through the input node when it is not enabled or available will
        cause the engine to throw an error (when possible) or an exception.

        In manual rendering mode, the input node can be used to synchronously supply data to
        the engine while it is rendering (see
        `AVAudioInputNode(setManualRenderingInputPCMFormat:inputBlock:)`.
*/
@available(OSX 10.10, *)
open var inputNode: AVAudioInputNode { get }

/** @property mainMixerNode
    @abstract
        The engine's optional singleton main mixer node.
    @discussion
        The engine will construct a singleton main mixer and connect it to the outputNode on demand,
        when this property is first accessed. You can then connect additional nodes to the mixer.

        By default, the mixer's output format (sample rate and channel count) will track the format
        of the output node. You may however make the connection explicitly with a different format.
*/
open var mainMixerNode: AVAudioMixerNode { get }

/** @property running
    @abstract
        The engine's running state.
*/
open var isRunning: Bool { get }

/** @property autoShutdownEnabled
    @abstract
        When auto shutdown is enabled, the engine can start and stop the audio hardware dynamically,
        to conserve power. This is the enforced behavior on watchOS and can be optionally enabled on
        other platforms.
    @discussion
        To conserve power, it is advised that the client pause/stop the engine when not in use.
        But when auto shutdown is enabled, the engine will stop the audio hardware if it was running
        idle for a certain duration, and restart it later when required.
        Note that, because this operation is dynamic, it may affect the start times of the source
        nodes (e.g. `AVAudioPlayerNode`), if the engine has to resume from its shutdown state.

        On watchOS, auto shutdown is always enabled. On other platforms, it is disabled by
        default, but the client can enable it if needed.

        This property is applicable only when the engine is rendering to/from an audio device. If
        the value is changed when the engine is in manual rendering mode, it will take effect
        whenever the engine is switched to render to/from the audio device.
*/
@available(OSX 10.13, *)
open var isAutoShutdownEnabled: Bool

/** @property attachedNodes
    @abstract
        Set of all nodes attached to the engine.
 */
@available(OSX 10.15, *)
open var attachedNodes: Set<AVAudioNode> { get }

/**    @method enableManualRenderingMode:format:maximumFrameCount:error:
    @abstract
        Set the engine to operate in manual rendering mode with the specified render format and
        maximum frame count.
    @param pcmFormat
        The format of the output PCM audio data from the engine
    @param maximumFrameCount
        The maximum number of PCM sample frames the engine will be asked to produce in any single
        render call
     @param outError
        On exit, if the engine cannot switch to the manual rendering mode, a description of the
        error (see `AVAudioEngineManualRenderingError` for the possible errors)
    @return
        YES for success
    @discussion
        Use this method to configure the engine to render in response to requests from the client.

        The engine must be in a stopped state before calling this method.
        The render format must be a PCM format and match the format of the buffer to which
        the engine is asked to render (see `renderOffline:toBuffer:error:`).

        The input data in manual rendering mode can be supplied through the source nodes, e.g.
        `AVAudioPlayerNode`, `AVAudioInputNode` etc.

         When switching to manual rendering mode, the engine:
        1. Switches the input and output nodes to manual rendering mode. Their input and output
           formats may change.
        2. Removes any taps previously installed on the input and output nodes.
        3. Maintains all the engine connections as is.

        Reasons for potential failure when switching to manual rendering mode include:
        - Engine is not in a stopped state.
*/
@available(OSX 10.13, *)
open func enableManualRenderingMode(_ mode: AVAudioEngineManualRenderingMode, format pcmFormat: AVAudioFormat, maximumFrameCount: AVAudioFrameCount) throws

/**    @method disableManualRenderingMode
    @abstract
        Set the engine to render to/from an audio device.
    @discussion
         When disabling the manual rendering mode, the engine:
        1. Stops and resets itself (see `stop` and `reset`).
        2. Switches the output/input nodes to render to/from an audio device. Their input and
           output formats may change.
        3. Removes any taps previously installed on the input and output nodes.
        4. Maintains all the engine connections as is.

        Calling this method when the engine is already rendering to/from an audio device has no
        effect.
*/
@available(OSX 10.13, *)
open func disableManualRenderingMode()

/**    @method renderOffline:toBuffer:error:
    @abstract
        Render call to the engine operating in the offline manual rendering mode
    @param numberOfFrames
        The number of PCM sample frames to be rendered
    @param buffer
        The PCM buffer to which the engine must render the audio
    @param outError
        On exit, if an error occurs during rendering, a description of the error (see
        `AVAudioEngineManualRenderingError` for the possible errors)
    @return
        One of the status codes from `AVAudioEngineManualRenderingStatus`. Irrespective of the
        returned status code, on exit, the output buffer's frameLength will indicate the number of
        PCM samples rendered by the engine
    @discussion
        The engine must be in the offline manual rendering mode
        (`AVAudioEngineManualRenderingModeOffline`) and started before calling this method.

        The format of the buffer must match the render format set through
        `enableManualRenderingMode:format:maximumFrameCount:error:`. The buffer capacity must be
        greater than or equal to the number of samples asked to render.
        On exit, the buffer's frameLength will indicate the number of PCM samples rendered by the
        engine.

         The engine's timeline in manual rendering mode starts at a sample time of zero, and is in
        terms of the render format's sample rate. Resetting the engine (see `reset`) will reset the
        timeline back to zero.

         When rendering in `AVAudioEngineManualRenderingModeRealtime`, this ObjC render method
        must not be used, an error is returned otherwise. Use the block based render call
        (`manualRenderingBlock`) in that mode instead.
*/
@available(OSX 10.13, *)
open func renderOffline(_ numberOfFrames: AVAudioFrameCount, to buffer: AVAudioPCMBuffer) throws -> AVAudioEngineManualRenderingStatus

/**    @property manualRenderingBlock
    @abstract
        Block to render the engine operating in manual rendering mode
    @discussion
        This block based render call must be used to render the engine when operating in
        `AVAudioEngineManualRenderingModeRealtime`. In this mode, the engine operates under
        realtime constraints and will not make any blocking call (e.g. calling libdispatch, blocking
        on a mutex, allocating memory etc.) while rendering.

        Before invoking the rendering functionality, client must fetch this block and cache the
        result. The block can then be called from a realtime context, without any possibility of
        blocking.

        When rendering in `AVAudioEngineManualRenderingModeOffline`, either this block based render
        call or    `renderOffline:toBuffer:error:` ObjC method can be used.
        All the rules outlined in `renderOffline:toBuffer:error:` are applicable here as well.
*/
@available(OSX 10.13, *)
open var manualRenderingBlock: AVAudioEngineManualRenderingBlock { get }

/** @property isInManualRenderingMode
    @abstract
        Whether or not the engine is operating in manual rendering mode, i.e. not connected
        to an audio device and rendering in response to the requests from the client
*/
@available(OSX 10.13, *)
open var isInManualRenderingMode: Bool { get }

/** @property manualRenderingMode
    @abstract
        The manual rendering mode configured on the engine
    @discussion
        This property is meaningful only when the engine is operating in manual rendering mode,
        i.e. when `isInManualRenderingMode` returns true.
*/
@available(OSX 10.13, *)
open var manualRenderingMode: AVAudioEngineManualRenderingMode { get }

/** @property manualRenderingFormat
    @abstract
        The render format of the engine in manual rendering mode.
    @discussion
        Querying this property when the engine is not in manual rendering mode will return an
        invalid format, with zero sample rate and channel count.
*/
@available(OSX 10.13, *)
open var manualRenderingFormat: AVAudioFormat { get }

/** @property manualRenderingMaximumFrameCount
    @abstract
        The maximum number of PCM sample frames the engine can produce in any single render call in
        the manual rendering mode.
    @discussion
        Querying this property when the engine is not in manual rendering mode will return zero.
*/
@available(OSX 10.13, *)
open var manualRenderingMaximumFrameCount: AVAudioFrameCount { get }

/** @property manualRenderingSampleTime
    @abstract
        Indicates where the engine is on its render timeline in manual rendering mode.
    @discussion
        The timeline in manual rendering mode starts at a sample time of zero, and is in terms
        of the render format's sample rate. Resetting the engine (see `reset`) will reset the
        timeline back to zero.
*/
@available(OSX 10.13, *)
open var manualRenderingSampleTime: AVAudioFramePosition { get }

/** @method connectMIDI:to:format:block:
    @abstract
        Establish a MIDI only connection between two nodes.
    @param sourceNode
        The source node.
    @param destinationNode
        The destination node.
    @param format
        If non-nil, the format of the source node's output bus is set to this format.
        In all cases, the format of the source nodes' output bus has to match with the
        destination nodes' output bus format.
        Although the output bus of the source is not in use, the format needs to be set
        in order to be able to use the sample rate for MIDI event timing calculations.
    @param tapBlock
        If non-nil, this block is called from the source node's `AUMIDIOutputEventBlock`
        on the realtime thread. The host can tap the MIDI data of the source node through
        this block. May be nil.
    @discussion
        Use this method to establish a MIDI only connection between a source node and a
        destination node that has MIDI input capability.

        The source node can only be a AVAudioUnit node of type `kAudioUnitType_MIDIProcessor`.
        The destination node types can be `kAudioUnitType_MusicDevice`,
        `kAudioUnitType_MusicEffect` or `kAudioUnitType_MIDIProcessor`.

        Note that any pre-existing MIDI connection involving the destination will be broken.

        Any client installed block on the source node's audio unit `AUMIDIOutputEventBlock`
        will be overwritten when making the MIDI connection.
 */
@available(OSX 10.14, *)
open func connectMIDI(_ sourceNode: AVAudioNode, to destinationNode: AVAudioNode, format: AVAudioFormat?, block tapBlock: AUMIDIOutputEventBlock? = nil)

/** @method connectMIDI:toNodes:format:block:
    @abstract
        Establish a MIDI only connection between a source node and multiple destination nodes.
    @param sourceNode
        The source node.
    @param destinationNodes
        An array of AVAudioNodes specifying destination nodes.
    @param format
        If non-nil, the format of the source node's output bus is set to this format.
        In all cases, the format of the source nodes' output bus has to match with the
        destination nodes' output bus format.
        Although the output bus of the source is not in use, the format needs to be set
        in order to be able to use the sample rate for MIDI event timing calculations.
    @param tapBlock
        If non-nil, this block is called from the source node's `AUMIDIOutputEventBlock`
        on the realtime thread. The host can tap the MIDI data of the source node through
        this block. May be nil.
    @discussion
        Use this method to establish a MIDI only connection between a source node and
        multiple destination nodes.

        The source node can only be a AVAudioUnit node of type `kAudioUnitType_MIDIProcessor`.
        The destination node types can be `kAudioUnitType_MusicDevice`,
        `kAudioUnitType_MusicEffect` or `kAudioUnitType_MIDIProcessor`.

        MIDI connections made using this method are either one-to-one (when a single
        destination connection is specified) or one-to-many (when multiple connections are
        specified), but never many-to-one.

        Note that any pre-existing connection involving the destination will be broken.

        Any client installed block on the source node's audio unit `AUMIDIOutputEventBlock`
        will be overwritten when making the MIDI connection.
 */
@available(OSX 10.14, *)
open func connectMIDI(_ sourceNode: AVAudioNode, to destinationNodes: [AVAudioNode], format: AVAudioFormat?, block tapBlock: AUMIDIOutputEventBlock? = nil)

/** @method disconnectMIDI:from:
    @abstract
        Remove a MIDI connection between two nodes.
    @param sourceNode
        The node whose MIDI output is to be disconnected.
    @param destinationNode
        The node whose MIDI input is to be disconnected.
    @discussion
        If a tap block is installed on the source node, it will be removed when the last
        connection from the source node is removed.
 */
@available(OSX 10.14, *)
open func disconnectMIDI(_ sourceNode: AVAudioNode, from destinationNode: AVAudioNode)

/** @method disconnectMIDI:fromNodes:
    @abstract
        Remove a MIDI connection between one source node and multiple destination nodes.
    @param sourceNode
        The node whose MIDI output is to be disconnected.
    @param destinationNodes
        An array of AVAudioNodes specifying nodes whose MIDI input is to be disconnected.
    @discussion
        If a tap block is installed on the source node, it will be removed when the last
        connection from the source node is removed.
 */
@available(OSX 10.14, *)
open func disconnectMIDI(_ sourceNode: AVAudioNode, from destinationNodes: [AVAudioNode])

/** @method disconnectMIDIInput:
    @abstract
        Disconnects all input MIDI connections of this node.
    @param node
        The node whose MIDI input is to be disconnected.
*/
@available(OSX 10.14, *)
open func disconnectMIDIInput(_ node: AVAudioNode)

/** @method disconnectMIDIOutput:
    @abstract
        Disconnects all output MIDI connections of this node.
    @param node
        The node whose MIDI outputs are to be disconnected.
*/
@available(OSX 10.14, *)
open func disconnectMIDIOutput(_ node: AVAudioNode)
