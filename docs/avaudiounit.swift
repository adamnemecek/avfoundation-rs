open class AVAudioUnit : AVAudioNode {

    
    /**	@method	instantiateWithComponentDescription:options:completionHandler:
    	@abstract Asynchronously create an instance of an audio unit component, wrapped in an AVAudioUnit.
    	@param audioComponentDescription
    		The component to instantiate.
    	@param options
    		Instantiation options.
    	@param completionHandler
    		Called in an arbitrary thread/queue context when instantiation is complete. The client
    		should retain the provided AVAudioUnit.
    	@discussion
    		Components whose flags include kAudioComponentFlag_RequiresAsyncInstantiation must be 
    		instantiated asynchronously, via this method if they are to be used with AVAudioEngine.
    		See the discussion of this flag in AudioToolbox/AudioComponent.h.
    		
    		The returned AVAudioUnit instance normally will be of a subclass (AVAudioUnitEffect,
    		AVAudioUnitGenerator, AVAudioUnitMIDIInstrument, or AVAudioUnitTimeEffect), selected
    		according to the component's type.
    */
    @available(OSX 10.11, *)
    open class func instantiate(with audioComponentDescription: AudioComponentDescription, options: AudioComponentInstantiationOptions = [], completionHandler: @escaping (AVAudioUnit?, Error?) -> Void)

    
    /** @method loadAudioUnitPresetAtURL:error:
        @abstract Load an audio unit preset.
        @param url
            NSURL of the .aupreset file.
    	@param outError
            A pointer to a NSError object
        @discussion
            If the .aupreset file cannot be successfully loaded, an error is returned.
    */
    open func loadPreset(at url: URL) throws

    
    /** @property audioComponentDescription
        @abstract AudioComponentDescription of the underlying audio unit.
    */
    open var audioComponentDescription: AudioComponentDescription { get }

    
    /** @property audioUnit
        @abstract Reference to the underlying audio unit.
        @discussion
            A reference to the underlying audio unit is provided so that parameters that are not
            exposed by AVAudioUnit subclasses can be modified using the AudioUnit C API.
     
            No operations that may conflict with state maintained by the engine should be performed
            directly on the audio unit. These include changing initialization state, stream formats,
            channel layouts or connections to other audio units.
    */
    open var audioUnit: AudioUnit { get }

    
    /** @property AUAudioUnit
        @abstract An AUAudioUnit wrapping or underlying the implementation's AudioUnit.
        @discussion
            This provides an AUAudioUnit which either wraps or underlies the implementation's
            AudioUnit, depending on how that audio unit is packaged. Applications can interact with this
            AUAudioUnit to control custom properties, select presets, change parameters, etc.
     
            As with the audioUnit property, no operations that may conflict with state maintained by the
            engine should be performed directly on the audio unit. These include changing initialization
            state, stream formats, channel layouts or connections to other audio units.
    */
    @available(OSX 10.11, *)
    open var auAudioUnit: AUAudioUnit { get }

    // __OBJC2__
    
    /** @property name
        @abstract Name of the audio unit.
    */
    open var name: String { get }

    
    /** @property manufacturerName
        @abstract Manufacturer name of the audio unit.
    */
    open var manufacturerName: String { get }

    
    /** @property version
        @abstract Version number of the audio unit.
    */
    open var version: Int { get }
}
