@available(OSX 10.10, *)
open class AVAudioNode : NSObject {


    /** @method reset
        @abstract Clear a unit's previous processing state.
    */
    open func reset()


    /** @method inputFormatForBus:
        @abstract Obtain an input bus's format.
    */
    open func inputFormat(forBus bus: AVAudioNodeBus) -> AVAudioFormat


    /** @method outputFormatForBus:
        @abstract Obtain an output bus's format.
    */
    open func outputFormat(forBus bus: AVAudioNodeBus) -> AVAudioFormat


    /**    @method nameForInputBus:
        @abstract Return the name of an input bus.
    */
    open func name(forInputBus bus: AVAudioNodeBus) -> String?


    /**    @method nameForOutputBus:
        @abstract Return the name of an output bus.
    */
    open func name(forOutputBus bus: AVAudioNodeBus) -> String?


    /** @method installTapOnBus:bufferSize:format:block:
        @abstract Create a "tap" to record/monitor/observe the output of the node.
        @param bus
            the node output bus to which to attach the tap
        @param bufferSize
            the requested size of the incoming buffers in sample frames. Supported range is [100, 400] ms.
        @param format
            If non-nil, attempts to apply this as the format of the specified output bus. This should
            only be done when attaching to an output bus which is not connected to another node; an
            error will result otherwise.
            The tap and connection formats (if non-nil) on the specified bus should be identical.
            Otherwise, the latter operation will override any previously set format.
        @param tapBlock
            a block to be called with audio buffers

        @discussion
            Only one tap may be installed on any bus. Taps may be safely installed and removed while
            the engine is running.

            Note that if you have a tap installed on AVAudioOutputNode, there could be a mismatch
            between the tap buffer format and AVAudioOutputNode's output format, depending on the
            underlying physical device. Hence, instead of tapping the AVAudioOutputNode, it is
            advised to tap the node connected to it.

            E.g. to capture audio from input node:
    <pre>
    AVAudioEngine *engine = [[AVAudioEngine alloc] init];
    AVAudioInputNode *input = [engine inputNode];
    AVAudioFormat *format = [input outputFormatForBus: 0];
    [input installTapOnBus: 0 bufferSize: 8192 format: format block: ^(AVAudioPCMBuffer *buf, AVAudioTime *when) {
    // ‘buf' contains audio captured from input node at time 'when'
    }];
    ....
    // start engine
    </pre>
    */
    open func installTap(onBus bus: AVAudioNodeBus, bufferSize: AVAudioFrameCount, format: AVAudioFormat?, block tapBlock: @escaping AVAudioNodeTapBlock)


    /**    @method removeTapOnBus:
        @abstract Destroy a tap.
        @param bus
            the node output bus whose tap is to be destroyed
    */
    open func removeTap(onBus bus: AVAudioNodeBus)


    /**    @property engine
        @abstract The engine to which the node is attached (or nil).
    */
    open var engine: AVAudioEngine? { get }


    /** @property numberOfInputs
        @abstract The node's number of input busses.
    */
    open var numberOfInputs: Int { get }


    /** @property numberOfOutputs
        @abstract The node's number of output busses.
    */
    open var numberOfOutputs: Int { get }


    /** @property lastRenderTime
        @abstract Obtain the time for which the node most recently rendered.
        @discussion
            Will return nil if the engine is not running or if the node is not connected to an input or
            output node.
    */
    open var lastRenderTime: AVAudioTime? { get }


    /** @property AUAudioUnit
        @abstract An AUAudioUnit wrapping or underlying the implementation's AudioUnit.
        @discussion
            This provides an AUAudioUnit which either wraps or underlies the implementation's
            AudioUnit, depending on how that audio unit is packaged. Applications can interact with this
            AUAudioUnit to control custom properties, select presets, change parameters, etc.

            No operations that may conflict with state maintained by the engine should be performed
            directly on the audio unit. These include changing initialization state, stream formats,
            channel layouts or connections to other audio units.
    */
    @available(OSX 10.13, *)
    open var auAudioUnit: AUAudioUnit { get }

    // AVAUDIONODE_HAVE_AUAUDIOUNIT

    /**    @property latency
        @abstract The processing latency of the node, in seconds.
        @discussion
            This property reflects the delay between when an impulse in the audio stream arrives at the
            input vs. output of the node. This should reflect the delay due to signal processing
            (e.g. filters, FFT's, etc.), not delay or reverberation which is being applied as an effect.
            A value of zero indicates either no latency or an unknown latency.
    */
    @available(OSX 10.13, *)
    open var latency: TimeInterval { get }


    /**    @property outputPresentationLatency
        @abstract The maximum render pipeline latency downstream of the node, in seconds.
        @discussion
            This describes the maximum time it will take for the audio at the output of a node to be
            presented.
            For instance, the output presentation latency of the output node in the engine is:
                - zero in manual rendering mode
                - the presentation latency of the device itself when rendering to an audio device
                  (see `AVAudioIONode(presentationLatency)`)
            The output presentation latency of a node connected directly to the output node is the
            output node's presentation latency plus the output node's processing latency (see `latency`).

            For a node which is exclusively in the input node chain (i.e. not connected to engine's
            output node), this property reflects the latency for the output of this node to be
            presented at the output of the terminating node in the input chain.

            A value of zero indicates either an unknown or no latency.

            Note that this latency value can change as the engine is reconfigured (started/stopped,
            connections made/altered downstream of this node etc.). So it is recommended not to cache
            this value and fetch it whenever it's needed.
    */
    @available(OSX 10.13, *)
    open var outputPresentationLatency: TimeInterval { get }
}
