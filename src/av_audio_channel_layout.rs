/// @class AVAudioChannelLayout
/// @abstract A description of the roles of a set of audio channels.
/// @discussion
///     This object is a thin wrapper for the AudioChannelLayout structure, described
///     in <CoreAudio/CoreAudioTypes.h>.
use crate::AVAudioChannelCount;

pub enum AVAudioChannelLayoutFFI {}

foreign_obj_type! {
    type CType = AVAudioChannelLayoutFFI;
    pub struct AVAudioChannelLayout;
    pub struct AVAudioChannelLayoutRef;
}

impl AVAudioChannelLayout {
    //  - (instancetype)init NS_UNAVAILABLE;

    // /*!    @method initWithLayoutTag:
    //     @abstract Initialize from a layout tag.
    //     @param layoutTag
    //         The tag.
    //     @discussion
    //         Returns nil if the tag is either kAudioChannelLayoutTag_UseChannelDescriptions or
    //         kAudioChannelLayoutTag_UseChannelBitmap.
    // */
    // - (nullable instancetype)initWithLayoutTag:(AudioChannelLayoutTag)layoutTag;

    // /*!    @method initWithLayout:
    //     @abstract Initialize from an AudioChannelLayout.
    //     @param layout
    //         The AudioChannelLayout.
    //     @discussion
    //         If the provided layout's tag is kAudioChannelLayoutTag_UseChannelDescriptions, this
    //         initializer attempts to convert it to a more specific tag.
    // */
    // - (instancetype)initWithLayout:(const AudioChannelLayout *)layout NS_DESIGNATED_INITIALIZER;

    // /*!    @method isEqual:
    //     @abstract Determine whether another AVAudioChannelLayout is exactly equal to this layout.
    //     @param object
    //         The AVAudioChannelLayout to compare against.
    //     @discussion
    //         The underlying AudioChannelLayoutTag and AudioChannelLayout are compared for equality.
    // */
    // - (BOOL)isEqual:(id)object;

    // /*!    @method layoutWithLayoutTag:
    //     @abstract Create from a layout tag.
    // */
    // + (instancetype)layoutWithLayoutTag:(AudioChannelLayoutTag)layoutTag;

    // /*!    @method layoutWithLayout:
    //     @abstract Create from an AudioChannelLayout
    // */
    // + (instancetype)layoutWithLayout:(const AudioChannelLayout *)layout;
}

impl AVAudioChannelLayoutRef {
    // /*!    @property layoutTag
    //     @abstract The layout's tag. */
    // @property (nonatomic, readonly) AudioChannelLayoutTag layoutTag;
    pub fn layout_tag(&self) -> ! {
        unsafe { msg_send![self, layoutTag] }
    }

    // /*!    @property layout
    //     @abstract The underlying AudioChannelLayout. */
    // @property (nonatomic, readonly) const AudioChannelLayout *layout;
    pub fn layout(&self) -> ! {
        unsafe { msg_send![self, layout] }
    }

    // /*! @property channelCount
    //     @abstract The number of channels of audio data.
    // */
    // @property (nonatomic, readonly) AVAudioChannelCount channelCount;
    pub fn channel_count(&self) -> AVAudioChannelCount {
        unsafe { msg_send![self, channelCount] }
    }
}
