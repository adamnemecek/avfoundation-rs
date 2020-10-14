@available(OSX 10.10, *)
open class AVAudioChannelLayout : NSObject, NSSecureCoding {

    
    /**	@method initWithLayoutTag:
    	@abstract Initialize from a layout tag.
    	@param layoutTag
    		The tag.
    	@discussion
    		Returns nil if the tag is either kAudioChannelLayoutTag_UseChannelDescriptions or
    		kAudioChannelLayoutTag_UseChannelBitmap.
    */
    public convenience init?(layoutTag: AudioChannelLayoutTag)

    
    /**	@method initWithLayout:
    	@abstract Initialize from an AudioChannelLayout.
    	@param layout
    		The AudioChannelLayout.
    	@discussion
    		If the provided layout's tag is kAudioChannelLayoutTag_UseChannelDescriptions, this
    		initializer attempts to convert it to a more specific tag.
    */
    public init(layout: UnsafePointer<AudioChannelLayout>)

    
    /**	@method isEqual:
    	@abstract Determine whether another AVAudioChannelLayout is exactly equal to this layout.
    	@param object
    		The AVAudioChannelLayout to compare against.
    	@discussion
    		The underlying AudioChannelLayoutTag and AudioChannelLayout are compared for equality.
    */
    open func isEqual(_ object: Any) -> Bool

    
    /**	@property layoutTag
    	@abstract The layout's tag. */
    open var layoutTag: AudioChannelLayoutTag { get }

    
    /**	@property layout
    	@abstract The underlying AudioChannelLayout. */
    open var layout: UnsafePointer<AudioChannelLayout> { get }

    
    /** @property channelCount
    	@abstract The number of channels of audio data.
    */
    open var channelCount: AVAudioChannelCount { get }
}
