use crate::{
    AVAudioNodeRef,
    AVAudioTimeRef,
};
use cocoa_foundation::foundation::NSUInteger;
use objc::runtime::{
    NO,
    YES,
};

/// @enum AVAudioPlayerNodeBufferOptions
///     @abstract    Options controlling buffer scheduling.
///
///     @constant    AVAudioPlayerNodeBufferLoops
///                     The buffer loops indefinitely.
///     @constant    AVAudioPlayerNodeBufferInterrupts
///                     The buffer interrupts any buffer already playing.
///     @constant    AVAudioPlayerNodeBufferInterruptsAtLoop
///                     The buffer interrupts any buffer already playing, at its loop point.
bitflags! {
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
    block::Block<(AVAudioPlayerNodeCompletionCallbackType), ()>;
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
