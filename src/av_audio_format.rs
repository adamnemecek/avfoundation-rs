use crate::AVAudioChannelLayoutRef;
use objc::runtime::{
    NO,
    YES,
};
pub enum AudioCommonFormat {
    OtherFormat,
    PcmFormatFloat32,
    PcmFormatFloat64,
    PcmFormatInt16,
    PcmFormatInt32,
}

pub enum AVAudioFormatFFI {}

foreign_obj_type! {
    type CType = AVAudioFormatFFI;
    pub struct AVAudioFormat;
    pub struct AVAudioFormatRef;
}

impl AVAudioFormat {
    //     /*! @method initWithStreamDescription:
    // 	@abstract Initialize from an AudioStreamBasicDescription.
    // 	@param asbd
    // 		the AudioStreamBasicDescription
    // 	@discussion
    // 		If the format specifies more than 2 channels, this method fails (returns nil).
    // */
    // - (nullable instancetype)initWithStreamDescription:(const AudioStreamBasicDescription *)asbd;
    // pub fn new_with_stream_description(asbd: &AudioStreamBasicDescriptionRef) -> Self {
    //     todo!()
    // }
    //
    // /*! @method initWithStreamDescription:channelLayout:
    // 	@abstract Initialize from an AudioStreamBasicDescription and optional channel layout.
    // 	@param asbd
    // 		the AudioStreamBasicDescription
    // 	@param layout
    // 		the channel layout. Can be nil only if asbd specifies 1 or 2 channels.
    // 	@discussion
    // 		If the format specifies more than 2 channels, this method fails (returns nil) unless layout
    // 		is non-nil.
    // */
    // - (nullable instancetype)initWithStreamDescription:(const AudioStreamBasicDescription *)asbd channelLayout:(AVAudioChannelLayout * __nullable)layout;
    //
    // /*! @method initStandardFormatWithSampleRate:channels:
    // 	@abstract Initialize to deinterleaved float with the specified sample rate and channel count.
    // 	@param sampleRate
    // 		the sample rate
    // 	@param channels
    // 		the channel count
    // 	@discussion
    // 		If the format specifies more than 2 channels, this method fails (returns nil).
    // */
    // - (nullable instancetype)initStandardFormatWithSampleRate:(double)sampleRate channels:(AVAudioChannelCount)channels;
    //
    // /*! @method initStandardFormatWithSampleRate:channelLayout:
    // 	@abstract Initialize to deinterleaved float with the specified sample rate and channel layout.
    // 	@param sampleRate
    // 		the sample rate
    // 	@param layout
    // 		the channel layout. must not be nil.
    // */
    // - (instancetype)initStandardFormatWithSampleRate:(double)sampleRate channelLayout:(AVAudioChannelLayout *)layout;
    //
    // /*! @method initWithCommonFormat:sampleRate:channels:interleaved:
    // 	@abstract Initialize to float with the specified sample rate, channel count and interleavedness.
    // 	@param format
    // 		the common format type
    // 	@param sampleRate
    // 		the sample rate
    // 	@param channels
    // 		the channel count
    // 	@param interleaved
    // 		true if interleaved
    // 	@discussion
    // 		If the format specifies more than 2 channels, this method fails (returns nil).
    // */
    // - (nullable instancetype)initWithCommonFormat:(AVAudioCommonFormat)format sampleRate:(double)sampleRate channels:(AVAudioChannelCount)channels interleaved:(BOOL)interleaved;
    //
    // /*! @method initWithCommonFormat:sampleRate:interleaved:channelLayout:
    // 	@abstract Initialize to float with the specified sample rate, channel layout and interleavedness.
    // 	@param format
    // 		the common format type
    // 	@param sampleRate
    // 		the sample rate
    // 	@param interleaved
    // 		true if interleaved
    // 	@param layout
    // 		the channel layout. must not be nil.
    // */
    // - (instancetype)initWithCommonFormat:(AVAudioCommonFormat)format sampleRate:(double)sampleRate interleaved:(BOOL)interleaved channelLayout:(AVAudioChannelLayout *)layout;
    //
    // /*! @method initWithSettings:
    // 	@abstract Initialize using a settings dictionary.
    // 	@discussion
    // 		See AVAudioSettings.h. Note that many settings dictionary elements pertain to encoder
    // 		settings, not the basic format, and will be ignored.
    //
    //  		Returns nil if a format cannot be constructed with the provided settings, e.g. when:
    // 			- AVNumberOfChannelsKey specifies more than 2 channels, but AVChannelLayoutKey hasn't
    // 			  been specified or the layout does not match
    // 			- AVLinearPCMBitDepthKey for linear PCM format specifies less than 8 or greater
    // 			  than 32 bits
    // 			- values for the keys are not of the expected types
    // */
    // - (nullable instancetype)initWithSettings:(NSDictionary<NSString *, id> *)settings;
    //
    // #if AVAUDIOFORMAT_HAVE_CMFORMATDESCRIPTION
    // /*!
    //  	@method initWithCMAudioFormatDescription:
    //  	@abstract initialize from a CMAudioFormatDescriptionRef.
    //  	@param formatDescription
    //  		the CMAudioFormatDescriptionRef.
    //  	@discussion
    //  		If formatDescription is invalid, this method fails (returns nil).
    //  */
    // - (instancetype)initWithCMAudioFormatDescription:(CMAudioFormatDescriptionRef)formatDescription API_AVAILABLE(macos(10.11), ios(9.0), watchos(2.0), tvos(9.0));
}

impl AVAudioFormatRef {
    pub fn is_standard(&self) -> bool {
        unsafe { msg_send![self, isStandard] }
    }

    pub fn common_format(&self) -> AudioCommonFormat {
        unsafe { msg_send![self, commonFormat] }
    }

    pub fn channel_count(&self) -> u32 {
        unsafe { msg_send![self, channelCount] }
    }

    pub fn sample_rate(&self) -> f64 {
        unsafe { msg_send![self, sampleRate] }
    }

    pub fn is_interleaved(&self) -> bool {
        unsafe { msg_send![self, isInterleaved] }
    }

    // pub fn stream_description: UnsafePointer<AudioStreamBasicDescription> { get } {
    //     unsafe {
    //         msg_send![self, streamDescription];
    //     }
    // }

    pub fn channel_layout(&self) -> Option<&AVAudioChannelLayoutRef> {
        unsafe { msg_send![self, channelLayout] }
    }

    // pub fn magic_cookie: Data? {
    //     unsafe {
    //         msg_send![self, magicCookie];
    //     }
    // }

    // pub fn settings: [String : Any] { get } {
    //     unsafe {
    //         msg_send![self, settings];
    //     }
    // }

    // pub fn format_description: CMAudioFormatDescription { get } {
    //     unsafe {
    //         msg_send![self, formatDescription];
    //     }
    // }
}
