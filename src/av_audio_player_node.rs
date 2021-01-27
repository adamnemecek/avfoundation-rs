use cocoa_foundation::{
    base::nil,
    foundation::NSUInteger,
};
use objc::runtime::{
    NO,
    YES,
};

use crate::{
    AVAudioNodeCompletionHandler,
    AVAudioNodeRef,
    AVAudioPCMBufferRef,
    AVAudioTimeRef,
};

bitflags! {
    /// @enum AVAudioPlayerNodeBufferOptions
    ///     @abstract    Options controlling buffer scheduling.
    ///
    ///     @constant    AVAudioPlayerNodeBufferLoops
    ///                     The buffer loops indefinitely.
    ///     @constant    AVAudioPlayerNodeBufferInterrupts
    ///                     The buffer interrupts any buffer already playing.
    ///     @constant    AVAudioPlayerNodeBufferInterruptsAtLoop
    ///                     The buffer interrupts any buffer already playing, at its loop point.
    pub struct AVAudioPlayerNodeBufferOptions: NSUInteger {
        const LOOPS                 = 1 << 0;
        const INTERRUPTS            = 1 << 1;
        const INTERRUPTS_AT_LOOP    = 1 << 2;
    }
}
/// @enum AVAudioPlayerNodeCompletionCallbackType
///     @abstract    Specifies when the completion handler must be invoked.
///
///     @constant    AVAudioPlayerNodeCompletionDataConsumed
///                     The buffer or file data has been consumed by the player.
///       @constant    AVAudioPlayerNodeCompletionDataRendered
///                     The buffer or file data has been rendered (i.e. output) by the player. This
///                     does not account for any signal processing latencies downstream of the player
///                     in the engine (see `AVAudioNode(outputPresentationLatency)`).
///     @constant    AVAudioPlayerNodeCompletionDataPlayedBack
///                     Applicable only when the engine is rendering to/from an audio device.
///                     The buffer or file has finished playing. This accounts for both (small) signal
///                     processing latencies downstream of the player in the engine, as well as
///                     (possibly significant) latency in the audio playback device.
///
///      API_AVAILABLE(macos(10.13), ios(11.0), watchos(4.0), tvos(11.0));

pub enum AVAudioPlayerNodeCompletionCallbackType {
    DataConsumed = 0,
    DataRendered = 1,
    DataPlayedBack = 2,
}

/// @typedef AVAudioPlayerNodeCompletionHandler
/// @abstract Buffer or file completion callback handler.
/// @param callbackType
///     Indicates the type of buffer or file completion when the callback is invoked.
/// @discussion
///     AVAudioPlayerNode issues this callback to inform the client about the specific type of
///     buffer or file completion. See `AVAudioPlayerNodeCompletionCallbackType` for more details.
///
///     Note that the `AVAudioNodeCompletionHandler` callback from some of the player's scheduling
///     methods (e.g. `scheduleBuffer:completionHandler:`) is equivalent to the
///     AVAudioPlayerNodeCompletionHandler callback for `AVAudioPlayerNodeCompletionDataConsumed`.
///
///     In general the callbacks arrive on a non-main thread and it is the client's responsibility
///     to handle them in a thread-safe manner.
///
///     Setting or getting properties on an AVAudioPlayerNode while the AVAudioEngine is running requires
///     some synchronisation between the calling threads internally. If you want to call player node API within this
///     completion handler block, calls should be synchronised to the same thread/queue.

pub type AVAudioPlayerNodeCompletionHandler =
    block::RcBlock<AVAudioPlayerNodeCompletionCallbackType, ()>;
pub enum AVAudioPlayerNodeFFI {}

foreign_obj_type! {
    type CType = AVAudioPlayerNodeFFI;
    pub struct AVAudioPlayerNode;
    pub struct AVAudioPlayerNodeRef;
    type ParentType = AVAudioNodeRef;
}

impl AVAudioPlayerNode {
    pub fn new() -> Self {
        unsafe {
            let class = class!(AVAudioPlayerNode);
            msg_send![class, new]
        }
    }
}

impl AVAudioPlayerNodeRef {
    /// @method scheduleBuffer:completionHandler:
    /// @abstract Schedule playing samples from an AVAudioBuffer.
    /// @param buffer
    ///     the buffer to play
    /// @param completionHandler
    ///     called after the buffer has been consumed by the player or the player is stopped. may be nil.
    /// @discussion
    ///     Schedules the buffer to be played following any previously scheduled commands.
    ///
    ///     It is possible for the completionHandler to be called before rendering begins
    ///     or before the buffer is played completely.
    pub fn schedule_buffer(
        &self,
        buffer: &AVAudioPCMBufferRef,
        completion_handler: Option<AVAudioNodeCompletionHandler>,
    ) {
        if let Some(handler) = completion_handler {
            unsafe { msg_send![self, scheduleBuffer: buffer completionHandler: handler] }
        } else {
            unsafe { msg_send![self, scheduleBuffer: buffer completionHandler: nil] }
        }
    }
    /// @method scheduleBuffer:completionCallbackType:completionHandler:
    ///    @abstract Schedule playing samples from an AVAudioBuffer.
    ///    @param buffer
    ///        the buffer to play
    ///    @param callbackType
    ///        option to specify when the completion handler must be called
    ///    @param completionHandler
    ///        called after the buffer has been consumed by the player or has finished playing back or
    ///        the player is stopped. may be nil.
    ///    @discussion
    ///        Schedules the buffer to be played following any previously scheduled commands.
    ///
    // - (void)scheduleBuffer:(AVAudioPCMBuffer *)buffer completionCallbackType:(AVAudioPlayerNodeCompletionCallbackType)callbackType completionHandler:(AVAudioPlayerNodeCompletionHandler __nullable)completionHandler API_AVAILABLE(macos(10.13), ios(11.0), watchos(4.0), tvos(11.0));

    /// @method scheduleBuffer:atTime:options:completionHandler:
    ///    @abstract Schedule playing samples from an AVAudioBuffer.
    ///    @param buffer
    ///        the buffer to play
    ///    @param when
    ///        the time at which to play the buffer. see the discussion of timestamps, above.
    ///    @param options
    ///        options for looping, interrupting other buffers, etc.
    ///    @param completionHandler
    ///        called after the buffer has been consumed by the player or the player is stopped. may be nil.
    ///    @discussion
    ///        It is possible for the completionHandler to be called before rendering begins
    ///        or before the buffer is played completely.
    ///
    // - (void)scheduleBuffer:(AVAudioPCMBuffer *)buffer atTime:(AVAudioTime * __nullable)when options:(AVAudioPlayerNodeBufferOptions)options completionHandler:(AVAudioNodeCompletionHandler __nullable)completionHandler;

    /// @method scheduleBuffer:atTime:options:completionCallbackType:completionHandler:
    ///    @abstract Schedule playing samples from an AVAudioBuffer.
    ///    @param buffer
    ///        the buffer to play
    ///    @param when
    ///        the time at which to play the buffer. see the discussion of timestamps, above.
    ///    @param options
    ///        options for looping, interrupting other buffers, etc.
    ///    @param callbackType
    ///        option to specify when the completion handler must be called
    ///    @param completionHandler
    ///        called after the buffer has been consumed by the player or has finished playing back or
    ///        the player is stopped. may be nil.
    ///
    // - (void)scheduleBuffer:(AVAudioPCMBuffer *)buffer atTime:(AVAudioTime * __nullable)when options:(AVAudioPlayerNodeBufferOptions)options
    //  completionCallbackType:(AVAudioPlayerNodeCompletionCallbackType)callbackType completionHandler:(AVAudioPlayerNodeCompletionHandler __nullable)completionHandler API_AVAILABLE(macos(10.13), ios(11.0), watchos(4.0), tvos(11.0));

    /// @method scheduleFile:atTime:completionHandler:
    ///    @abstract Schedule playing of an entire audio file.
    ///    @param file
    ///        the file to play
    ///    @param when
    ///        the time at which to play the file. see the discussion of timestamps, above.
    ///    @param completionHandler
    ///        called after the file has been consumed by the player or the player is stopped. may be nil.
    ///    @discussion
    ///        It is possible for the completionHandler to be called before rendering begins
    ///        or before the file is played completely.
    ///
    // - (void)scheduleFile:(AVAudioFile *)file atTime:(AVAudioTime * __nullable)when completionHandler:(AVAudioNodeCompletionHandler __nullable)completionHandler;

    /// @method scheduleFile:atTime:completionCallbackType:completionHandler:
    ///    @abstract Schedule playing of an entire audio file.
    ///    @param file
    ///        the file to play
    ///    @param when
    ///        the time at which to play the file. see the discussion of timestamps, above.
    ///    @param callbackType
    ///        option to specify when the completion handler must be called
    ///    @param completionHandler
    ///        called after the file has been consumed by the player or has finished playing back or
    ///        the player is stopped. may be nil.
    ///
    // - (void)scheduleFile:(AVAudioFile *)file atTime:(AVAudioTime * __nullable)when completionCallbackType:(AVAudioPlayerNodeCompletionCallbackType)callbackType completionHandler:(AVAudioPlayerNodeCompletionHandler __nullable)completionHandler API_AVAILABLE(macos(10.13), ios(11.0), watchos(4.0), tvos(11.0));

    /// @method scheduleSegment:startingFrame:frameCount:atTime:completionHandler:
    ///    @abstract Schedule playing a segment of an audio file.
    ///    @param file
    ///        the file to play
    ///    @param startFrame
    ///        the starting frame position in the stream
    ///    @param numberFrames
    ///        the number of frames to play
    ///    @param when
    ///        the time at which to play the region. see the discussion of timestamps, above.
    ///    @param completionHandler
    ///        called after the segment has been consumed by the player or the player is stopped. may be nil.
    ///    @discussion
    ///        It is possible for the completionHandler to be called before rendering begins
    ///        or before the segment is played completely.
    ///
    // - (void)scheduleSegment:(AVAudioFile *)file startingFrame:(AVAudioFramePosition)startFrame frameCount:(AVAudioFrameCount)numberFrames atTime:(AVAudioTime * __nullable)when completionHandler:(AVAudioNodeCompletionHandler __nullable)completionHandler;

    /// @method scheduleSegment:startingFrame:frameCount:atTime:completionCallbackType:completionHandler:
    ///    @abstract Schedule playing a segment of an audio file.
    ///    @param file
    ///        the file to play
    ///    @param startFrame
    ///        the starting frame position in the stream
    ///    @param numberFrames
    ///        the number of frames to play
    ///    @param when
    ///        the time at which to play the region. see the discussion of timestamps, above.
    ///    @param callbackType
    ///        option to specify when the completion handler must be called
    ///    @param completionHandler
    ///        called after the segment has been consumed by the player or has finished playing back or
    ///        the player is stopped. may be nil.
    ///
    // - (void)scheduleSegment:(AVAudioFile *)file startingFrame:(AVAudioFramePosition)startFrame frameCount:(AVAudioFrameCount)numberFrames atTime:(AVAudioTime * __nullable)when
    // completionCallbackType:(AVAudioPlayerNodeCompletionCallbackType)callbackType completionHandler:(AVAudioPlayerNodeCompletionHandler __nullable)completionHandler API_AVAILABLE(macos(10.13), ios(11.0), watchos(4.0), tvos(11.0));

    ///    @method stop
    ///    @abstract Clear all of the node's previously scheduled events and stop playback.
    ///    @discussion
    ///        All of the node's previously scheduled events are cleared, including any that are in the
    ///        middle of playing. The node's sample time (and therefore the times to which new events are
    ///        to be scheduled) is reset to 0, and will not proceed until the node is started again (via
    ///        play or playAtTime).
    ///
    ///        Note that pausing or stopping all the players connected to an engine does not pause or stop
    ///        the engine or the underlying hardware. The engine must be explicitly paused or stopped for
    ///        the hardware to stop.
    ///
    pub fn stop(&self) {
        unsafe { msg_send![self, stop] }
    }

    /// @method prepareWithFrameCount:
    ///    @abstract Prepares previously scheduled file regions or buffers for playback.
    ///    @param frameCount
    ///        The number of sample frames of data to be prepared before returning.
    ///
    // - (void)prepareWithFrameCount:(AVAudioFrameCount)frameCount;

    ///    @method play
    ///    @abstract Start or resume playback immediately.
    ///    @discussion
    ///        equivalent to playAtTime:nil
    ///
    pub fn play(&self) {
        unsafe { msg_send![self, play] }
    }

    ///    @method playAtTime:
    ///    @abstract Start or resume playback at a specific time.
    ///    @param when
    ///        the node time at which to start or resume playback. nil signifies "now".
    ///    @discussion
    ///        This node is initially paused. Requests to play buffers or file segments are enqueued, and
    ///        any necessary decoding begins immediately. Playback does not begin, however, until the player
    ///        has started playing, via this method.
    ///
    ///         Note that providing an AVAudioTime which is past (before lastRenderTime) will cause the
    ///         player to begin playback immediately.
    ///
    ///        E.g. To start a player X seconds in future:
    // <pre>
    // start engine and player
    // NSError *nsErr = nil;
    // [_engine startAndReturnError:&nsErr];
    // if (!nsErr) {
    //     const float kStartDelayTime = 0.5; // sec
    //     AVAudioFormat *outputFormat = [_player outputFormatForBus:0];
    //     AVAudioFramePosition startSampleTime = _player.lastRenderTime.sampleTime + kStartDelayTime * outputFormat.sampleRate;
    //     AVAudioTime *startTime = [AVAudioTime timeWithSampleTime:startSampleTime atRate:outputFormat.sampleRate];
    //     [_player playAtTime:startTime];
    // }
    // </pre>
    ///
    // - (void)playAtTime:(AVAudioTime * __nullable)when;
    pub fn play_at_time(&self, when: Option<&AVAudioTimeRef>) {
        unsafe { msg_send![self, playAtTime: when] }
    }

    /// @method pause
    ///    @abstract Pause playback.
    ///    @discussion
    ///        The player's sample time does not advance while the node is paused.
    ///
    ///        Note that pausing or stopping all the players connected to an engine does not pause or stop
    ///        the engine or the underlying hardware. The engine must be explicitly paused or stopped for
    ///        the hardware to stop.
    ///
    pub fn pause(&self) {
        unsafe { msg_send![self, pause] }
    }

    ///    @method nodeTimeForPlayerTime:
    ///    @abstract
    ///        Convert from player time to node time.
    ///    @param playerTime
    ///        a time relative to the player's start time
    ///    @return
    ///        a node time
    ///    @discussion
    ///        This method and its inverse `playerTimeForNodeTime:` are discussed in the
    ///        introduction to this class.
    ///
    ///        If the player is not playing when this method is called, nil is returned.
    ///
    // - (AVAudioTime * __nullable)nodeTimeForPlayerTime:(AVAudioTime *)playerTime;

    ///    @method playerTimeForNodeTime:
    ///    @abstract
    ///        Convert from node time to player time.
    ///    @param nodeTime
    ///        a node time
    ///    @return
    ///        a time relative to the player's start time
    ///    @discussion
    ///        This method and its inverse `nodeTimeForPlayerTime:` are discussed in the
    ///        introduction to this class.
    ///
    ///        If the player is not playing when this method is called, nil is returned.
    ///
    // - (AVAudioTime * __nullable)playerTimeForNodeTime:(AVAudioTime *)nodeTime;

    ///    @property playing
    ///    @abstract Indicates whether or not the player is playing.
    pub fn is_playing(&self) -> bool {
        unsafe {
            match msg_send![self, isPlaying] {
                YES => true,
                NO => false,
                _ => unreachable!(),
            }
        }
    }
}
