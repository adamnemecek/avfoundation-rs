vailable(OSX 10.10, *)
open class AVAudioFormat : NSObject, NSSecureCoding {

    
    /** @method initWithStreamDescription:
        @abstract Initialize from an AudioStreamBasicDescription.
        @param asbd
            the AudioStreamBasicDescription
        @discussion
            If the format specifies more than 2 channels, this method fails (returns nil).
    */
    public init?(streamDescription asbd: UnsafePointer<AudioStreamBasicDescription>)

    
    /** @method initWithStreamDescription:channelLayout:
        @abstract Initialize from an AudioStreamBasicDescription and optional channel layout.
        @param asbd
            the AudioStreamBasicDescription
        @param layout
            the channel layout. Can be nil only if asbd specifies 1 or 2 channels.
        @discussion
            If the format specifies more than 2 channels, this method fails (returns nil) unless layout
            is non-nil.
    */
    public init?(streamDescription asbd: UnsafePointer<AudioStreamBasicDescription>, channelLayout layout: AVAudioChannelLayout?)

    
    /** @method initStandardFormatWithSampleRate:channels:
        @abstract Initialize to deinterleaved float with the specified sample rate and channel count.
        @param sampleRate
            the sample rate
        @param channels
            the channel count
        @discussion
            If the format specifies more than 2 channels, this method fails (returns nil).
    */
    public init?(standardFormatWithSampleRate sampleRate: Double, channels: AVAudioChannelCount)

    
    /** @method initStandardFormatWithSampleRate:channelLayout:
        @abstract Initialize to deinterleaved float with the specified sample rate and channel layout.
        @param sampleRate
            the sample rate
        @param layout
            the channel layout. must not be nil.
    */
    public init(standardFormatWithSampleRate sampleRate: Double, channelLayout layout: AVAudioChannelLayout)

    
    /** @method initWithCommonFormat:sampleRate:channels:interleaved:
        @abstract Initialize to float with the specified sample rate, channel count and interleavedness.
        @param format
            the common format type
        @param sampleRate
            the sample rate
        @param channels
            the channel count
        @param interleaved
            true if interleaved
        @discussion
            If the format specifies more than 2 channels, this method fails (returns nil).
    */
    public init?(commonFormat format: AVAudioCommonFormat, sampleRate: Double, channels: AVAudioChannelCount, interleaved: Bool)

    
    /** @method initWithCommonFormat:sampleRate:interleaved:channelLayout:
        @abstract Initialize to float with the specified sample rate, channel layout and interleavedness.
        @param format
            the common format type
        @param sampleRate
            the sample rate
        @param interleaved
            true if interleaved
        @param layout
            the channel layout. must not be nil.
    */
    public init(commonFormat format: AVAudioCommonFormat, sampleRate: Double, interleaved: Bool, channelLayout layout: AVAudioChannelLayout)

    
    /** @method initWithSettings:
        @abstract Initialize using a settings dictionary.
        @discussion
            See AVAudioSettings.h. Note that many settings dictionary elements pertain to encoder
            settings, not the basic format, and will be ignored.
    
             Returns nil if a format cannot be constructed with the provided settings, e.g. when:
                - AVNumberOfChannelsKey specifies more than 2 channels, but AVChannelLayoutKey hasn't 
                  been specified or the layout does not match
                - AVLinearPCMBitDepthKey for linear PCM format specifies less than 8 or greater
                  than 32 bits
                - values for the keys are not of the expected types
    */
    public init?(settings: [String : Any])

    
    /**
         @method initWithCMAudioFormatDescription:
         @abstract initialize from a CMAudioFormatDescriptionRef.
         @param formatDescription
             the CMAudioFormatDescriptionRef.
         @discussion
             If formatDescription is invalid, this method fails (returns nil).
     */
    @available(OSX 10.11, *)
    public init(cmAudioFormatDescription formatDescription: CMAudioFormatDescription)

    
    /**    @method isEqual:
        @abstract Determine whether another format is functionally equivalent.
        @param object
            the format to compare against
        @discussion
            For PCM, interleavedness is ignored for mono. Differences in the AudioStreamBasicDescription
            alignment and packedness are ignored when they are not significant (e.g. with 1 channel, 2
            bytes per frame and 16 bits per channel, neither alignment, the format is implicitly packed
            and can be interpreted as either high- or low-aligned.)
            For AVAudioChannelLayout, a layout with standard mono/stereo tag is considered to be 
            equivalent to a nil layout. Otherwise, the layouts are compared for equality.
    */
    open func isEqual(_ object: Any) -> Bool

    
    /**    @property standard
        @abstract Describes whether the format is deinterleaved native-endian float.
    */
    open var isStandard: Bool { get }

    
    /**    @property commonFormat
        @abstract An `AVAudioCommonFormat` identifying the format
    */
    open var commonFormat: AVAudioCommonFormat { get }

    
    /** @property channelCount
        @abstract The number of channels of audio data.
    */
    open var channelCount: AVAudioChannelCount { get }

    
    /** @property sampleRate
        @abstract A sampling rate in Hertz.
    */
    open var sampleRate: Double { get }

    
    /**    @property interleaved
        @abstract Describes whether the samples are interleaved.
        @discussion
            For non-PCM formats, the value is undefined.
    */
    open var isInterleaved: Bool { get }

    
    /**    @property streamDescription
        @abstract Returns the AudioStreamBasicDescription, for use with lower-level audio API's.
    */
    open var streamDescription: UnsafePointer<AudioStreamBasicDescription> { get }

    
    /**    @property channelLayout
        @abstract The underlying AVAudioChannelLayout, if any.
        @discussion
            Only formats with more than 2 channels are required to have channel layouts.
    */
    open var channelLayout: AVAudioChannelLayout? { get }

    
    /** @property magicCookie
        @abstract The underlying magic cookie, if any.
        @discussion
            A magic cookie contains metadata associated with encoders and decoders.
            Encoders produce a magic cookie, and some decoders require a magic cookie to decode properly.
    */
    @available(OSX 10.12, *)
    open var magicCookie: Data?

    
    /**    @property settings
        @abstract Returns the format represented as a dictionary with keys from AVAudioSettings.h.
    */
    open var settings: [String : Any] { get }

    
    /**
         @property formatDescription
         @abstract Converts to a CMAudioFormatDescriptionRef, for use with Core Media API's.
     */
    @available(OSX 10.11, *)
    open var formatDescription: CMAudioFormatDescription { get }
}
