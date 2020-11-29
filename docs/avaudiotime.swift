open class AVAudioTime : NSObject {

    
    /**    @method initWithAudioTimeStamp:sampleRate:
    */
    public init(audioTimeStamp ts: UnsafePointer<AudioTimeStamp>, sampleRate: Double)

    
    /** @method initWithHostTime:
    */
    public init(hostTime: UInt64)

    
    /** @method initWithSampleTime:atRate:
    */
    public init(sampleTime: AVAudioFramePosition, atRate sampleRate: Double)

    
    /** @method initWithHostTime:sampleTime:atRate:
    */
    public init(hostTime: UInt64, sampleTime: AVAudioFramePosition, atRate sampleRate: Double)

    
    /**    @method hostTimeForSeconds:
        @abstract Convert seconds to host time.
    */
    open class func hostTime(forSeconds seconds: TimeInterval) -> UInt64

    
    /**    @method secondsForHostTime:
        @abstract Convert host time to seconds.
    */
    open class func seconds(forHostTime hostTime: UInt64) -> TimeInterval

    
    /**    @method extrapolateTimeFromAnchor:
        @abstract Converts between host and sample time.
        @param anchorTime
            An AVAudioTime with a more complete AudioTimeStamp than that of the receiver (self).
        @return
            the extrapolated time
        @discussion
            If anchorTime is an AVAudioTime where both host time and sample time are valid,
            and self is another timestamp where only one of the two is valid, this method
            returns a new AVAudioTime copied from self and where any additional valid fields provided by
            the anchor are also valid.
     
            Note that the anchorTime must have both host and sample time valid, and self must have
            sample rate and at least one of host or sample time valid. Otherwise this method returns nil.
    
    <pre>
    // time0 has a valid audio sample representation, but no host time representation.
    AVAudioTime *time0 = [AVAudioTime timeWithSampleTime: 0.0 atRate: 44100.0];
    // anchor has a valid host time representation and sample time representation.
    AVAudioTime *anchor = [player playerTimeForNodeTime: player.lastRenderTime];
    // fill in valid host time representation
    AVAudioTime *fullTime0 = [time0 extrapolateTimeFromAnchor: anchor];
    </pre>
    */
    open func extrapolateTime(fromAnchor anchorTime: AVAudioTime) -> AVAudioTime?

    
    /** @property hostTimeValid
        @abstract Whether the hostTime property is valid.
    */
    open var isHostTimeValid: Bool { get }

    
    /** @property hostTime
        @abstract The host time.
    */
    open var hostTime: UInt64 { get }

    
    /** @property sampleTimeValid
        @abstract Whether the sampleTime and sampleRate properties are valid.
    */
    open var isSampleTimeValid: Bool { get }

    
    /**    @property sampleTime
        @abstract The time as a number of audio samples, as tracked by the current audio device.
    */
    open var sampleTime: AVAudioFramePosition { get }

    
    /**    @property sampleRate
        @abstract The sample rate at which sampleTime is being expressed.
    */
    open var sampleRate: Double { get }

    
    /** @property audioTimeStamp
        @abstract The time expressed as an AudioTimeStamp structure.
        @discussion
            This may be useful for compatibility with lower-level CoreAudio and AudioToolbox API's.
    */
    open var audioTimeStamp: AudioTimeStamp { get }
}
