use crate::{
    AUAudioUnitRef,
    AVAudioNodeRef,
    AudioComponentDescription,
    NSError,
    NSErrorRef,
};

// /*! @class AVAudioUnit
//     @abstract An AVAudioNode implemented by an audio unit.
//     @discussion
//         An AVAudioUnit is an AVAudioNode implemented by an audio unit. Depending on the type of
//         the audio unit, audio is processed either in real-time or non real-time.
// */
pub enum AVAudioUnitFFI {}

foreign_obj_type! {
    type CType = AVAudioUnitFFI;
    pub struct AVAudioUnit;
    pub struct AVAudioUnitRef;
    type ParentType = AVAudioNodeRef;
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AudioComponentInstantiationOptions {
    inner: u32,
}

pub type AVAudioUnitInitCompletionHandler<'a> = block::RcBlock<(&'a AVAudioUnitRef, NSError), ()>;

impl AVAudioUnit {
    // /*!	@method	instantiateWithComponentDescription:options:completionHandler:
    // 	@abstract Asynchronously create an instance of an audio unit component, wrapped in an AVAudioUnit.
    // 	@param audioComponentDescription
    // 		The component to instantiate.
    // 	@param options
    // 		Instantiation options.
    // 	@param completionHandler
    // 		Called in an arbitrary thread/queue context when instantiation is complete. The client
    // 		should retain the provided AVAudioUnit.
    // 	@discussion
    // 		Components whose flags include kAudioComponentFlag_RequiresAsyncInstantiation must be
    // 		instantiated asynchronously, via this method if they are to be used with AVAudioEngine.
    // 		See the discussion of this flag in AudioToolbox/AudioComponent.h.

    // 		The returned AVAudioUnit instance normally will be of a subclass (AVAudioUnitEffect,
    // 		AVAudioUnitGenerator, AVAudioUnitMIDIInstrument, or AVAudioUnitTimeEffect), selected
    // 		according to the component's type.
    // */
    // + (void)instantiateWithComponentDescription:(AudioComponentDescription)audioComponentDescription options:(AudioComponentInstantiationOptions)options completionHandler:(void (^)(__kindof AVAudioUnit * __nullable audioUnit, NSError * __nullable error))completionHandler API_AVAILABLE(macos(10.11), ios(9.0), tvos(9.0));
    pub fn with_component_description(
        desc: AudioComponentDescription,
        options: AudioComponentInstantiationOptions,
        completion_handler: AVAudioUnitInitCompletionHandler,
    ) -> Self {
        unsafe {
            let self_: Self = msg_send![class!(AVAudioUnit), instantiateWithComponentDescription: desc
                                                                                         options: options
                                                                               completionHandler: completion_handler];
            self_
        }
    }
}

impl AVAudioUnitRef {
    // /*! @method loadAudioUnitPresetAtURL:error:
    //     @abstract Load an audio unit preset.
    //     @param url
    //         NSURL of the .aupreset file.
    // 	@param outError
    //         A pointer to a NSError object
    //     @discussion
    //         If the .aupreset file cannot be successfully loaded, an error is returned.
    // */
    // - (BOOL)loadAudioUnitPresetAtURL:(NSURL *)url error:(NSError **)outError;

    // /*! @property audioComponentDescription
    //     @abstract AudioComponentDescription of the underlying audio unit.
    // */
    // @property (nonatomic, readonly) AudioComponentDescription audioComponentDescription;
    // pub fn audio_component_description(&self) -> AudioComponentDescription {
    //  unsafe { msg_send![self, audioComponentDescription] }
    // }

    // /*! @property audioUnit
    //     @abstract Reference to the underlying audio unit.
    //     @discussion
    //         A reference to the underlying audio unit is provided so that parameters that are not
    //         exposed by AVAudioUnit subclasses can be modified using the AudioUnit C API.

    //         No operations that may conflict with state maintained by the engine should be performed
    //         directly on the audio unit. These include changing initialization state, stream formats,
    //         channel layouts or connections to other audio units.
    // */
    // @property (nonatomic, readonly) AudioUnit audioUnit;

    // pub fn audio_unit(&self) -> AudioUnit {
    //  unsafe { msg_send![self, audioUnit] }
    // }

    // #ifdef __OBJC2__
    // /*! @property AUAudioUnit
    //     @abstract An AUAudioUnit wrapping or underlying the implementation's AudioUnit.
    //     @discussion
    //         This provides an AUAudioUnit which either wraps or underlies the implementation's
    //         AudioUnit, depending on how that audio unit is packaged. Applications can interact with this
    //         AUAudioUnit to control custom properties, select presets, change parameters, etc.

    //         As with the audioUnit property, no operations that may conflict with state maintained by the
    //         engine should be performed directly on the audio unit. These include changing initialization
    //         state, stream formats, channel layouts or connections to other audio units.
    // */
    // @property (nonatomic, readonly) AUAudioUnit *AUAudioUnit API_AVAILABLE(macos(10.11), ios(9.0), tvos(9.0));

    pub fn au_audio_unit(&self) -> &AUAudioUnitRef {
        unsafe { msg_send![self, auAudioUnit] }
    }
    // #endif // __OBJC2__

    // /*! @property name
    //     @abstract Name of the audio unit.
    // */
    // @property (nonatomic, readonly) NSString *name;
    pub fn name(&self) -> &str {
        unsafe {
            let name = msg_send![self, name];
            crate::nsstring_as_str(name)
        }
    }

    // /*! @property manufacturerName
    //     @abstract Manufacturer name of the audio unit.
    // */
    // @property (nonatomic, readonly) NSString *manufacturerName;

    pub fn manufacturer_name(&self) -> &str {
        unsafe {
            let name = msg_send![self, manufacturerName];
            crate::nsstring_as_str(name)
        }
    }

    // /*! @property version
    //     @abstract Version number of the audio unit.
    // */
    // @property (nonatomic, readonly) NSUInteger version;

    pub fn version(&self) -> i64 {
        unsafe { msg_send![self, version] }
    }
}
