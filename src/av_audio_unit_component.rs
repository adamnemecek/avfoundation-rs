use objc::runtime::{
    Object,
    BOOL,
    NO,
    YES,
};

fn u32_to_string(a: u32) -> String {
    let mut s: [char; 4] = [' ', ' ', ' ', ' '];
    s[0] = ((a >> 24) as u8) as char;
    s[1] = ((a >> 16) as u8) as char;
    s[2] = ((a >> 8) as u8) as char;
    s[3] = (a as u8) as char;

    s.iter().collect()
}
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct AudioComponentDescription {
    pub component_type: u32,
    pub component_sub_type: u32,
    pub component_manufacturer: u32,
    pub component_flags: u32,
    pub component_flags_mask: u32,
}

impl std::fmt::Debug for AudioComponentDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "hello");
        f.write_str(&format!(
            "component_type: {}\n",
            u32_to_string(self.component_type)
        ))?;
        f.write_str(&format!(
            "component_sub_type: {}\n",
            u32_to_string(self.component_sub_type)
        ))?;
        f.write_str(&format!(
            "component_manufacturer: {}\n",
            u32_to_string(self.component_manufacturer)
        ))?;
        f.write_str(&format!(
            "component_flags: {}\n",
            u32_to_string(self.component_flags)
        ))?;
        f.write_str(&format!(
            "component_flags_mask: {}",
            self.component_flags_mask
        ))
        // write!(f, format!("component_flags_mask: {}", self.component_flags_mask));
        // std::fmt::Result::Ok(())
        // f.debug_struct("AudioComponentDescription")
        //     .field("component_type: {}", u32_to_string(&self.component_type))
        //     .field("component_sub_type: {}", u32_to_string(&self.component_sub_type),
        //     )
        //     .field("component_manufacturer: {}", u32_to_string(&self.component_manufacturer),
        //     )
        //     .field("component_flags: {}", u32_to_string(&self.component_flags))
        //     .field("component_flags_mask: {}", self.component_flags_mask)
        //     .finish()
    }
}

pub enum AVAudioUnitComponentFFI {}

foreign_obj_type! {
    type CType = AVAudioUnitComponentFFI;
    pub struct AVAudioUnitComponent;
    pub struct AVAudioUnitComponentRef;
}

impl AVAudioUnitComponentRef {
    // fn audio_component(&self) -> AudioCOmpo

    pub fn name(&self) -> &str {
        unsafe {
            let s = msg_send![self, name];
            crate::nsstring_as_str(s)
        }
    }

    pub fn type_name(&self) -> &str {
        unsafe {
            let s = msg_send![self, typeName];
            crate::nsstring_as_str(s)
        }
    }

    // localizedTypeName

    pub fn manufacturer_name(&self) -> &str {
        unsafe {
            let s = msg_send![self, manufacturerName];
            crate::nsstring_as_str(s)
        }
    }

    //version
    //versionstring
    //sandboxsafe
    // audiocomponent

    pub fn audio_component_description(&self) -> AudioComponentDescription {
        unsafe { msg_send![self, audioComponentDescription] }
    }

    pub fn has_midi_output(&self) -> bool {
        unsafe {
            match msg_send![self, hasMIDIOutput] {
                YES => true,
                NO => false,
            }
        }
    }

    pub fn has_midi_input(&self) -> bool {
        unsafe {
            match msg_send![self, hasMIDIInput] {
                YES => true,
                NO => false,
            }
        }
    }

    pub fn has_custom_view(&self) -> bool {
        unsafe {
            match msg_send![self, hasCustomView] {
                YES => true,
                NO => false,
            }
        }
    }
}

// /*!
// 	@class	AVAudioUnitComponentManager
// 	@brief	A singleton object that provides an easy way to find audio components that are
// 			registered with the system.

// 	AVAudioUnitComponentManager provides methods to search and query various information about the
// 	audio components without opening them.

// 	Currently audio components that are audio units can only be searched.

// 	The class also supports predefined system tags and arbitrary user tags. Each audio unit can be
// 	tagged as part of its definition. Refer to AudioComponent.h for more details. AudioUnit Hosts
// 	such as Logic or GarageBand can present groupings of audio units based on the tags.

// 	Searching for audio units can be done in various ways
// 		- using a NSPredicate that contains search strings for tags or descriptions
// 		- using a block to match on custom criteria
// 		- using an AudioComponentDescription
// */
pub enum AVAudioUnitComponentManagerFFI {}

foreign_obj_type! {
    type CType = AVAudioUnitComponentManagerFFI;
    pub struct AVAudioUnitComponentManager;
    pub struct AVAudioUnitComponentManagerRef;
}

// BOOL (^)(AVAudioUnitComponent *comp, BOOL *stop)

pub enum ShouldStop {
    Stop,
    Continue,
}

// type AudioUnitPredicate<'a> = Block<(&'a AVAudioUnitComponentRef, *mut bool), bool>;

impl AVAudioUnitComponentManager {
    // + (instancetype)sharedAudioUnitComponentManager;

    pub fn shared<'a>() -> &'a AVAudioUnitComponentManagerRef {
        unsafe {
            let class = class!(AVAudioUnitComponentManager);
            msg_send![class, sharedAudioUnitComponentManager]
        }
    }
}

impl AVAudioUnitComponentManagerRef {
    // /*! @discussion
    //  		returns all tags associated with the current user as well as all system tags defined by
    // 		the audio unit(s).
    // */
    // @property (nonatomic, readonly) NSArray<NSString *>		*tagNames;

    // /*! @discussion
    // 		returns the localized standard system tags defined by the audio unit(s).
    // */
    // @property (nonatomic, readonly) NSArray<NSString *>		*standardLocalizedTagNames;

    // /* returns singleton instance of AVAudioUnitComponentManager */
    // /*!
    //  @method componentsMatchingPredicate:
    //  @abstract	returns an array of AVAudioUnitComponent objects that match the search predicate.
    //  @discussion
    //  		AudioComponent's information or tags can be used to build a search criteria.
    //  		For example, "typeName CONTAINS 'Effect'" or tags IN {'Sampler', 'MIDI'}"
    // */
    // - (NSArray<AVAudioUnitComponent *> *)componentsMatchingPredicate:(NSPredicate *)predicate;

    // /*!
    //  @method componentsPassingTest:
    //  @abstract	returns an array of AVAudioUnitComponent objects that pass the user provided block method.
    //  @discussion
    // 		For each AudioComponent found by the manager, the block method will be called. If the return
    //  		value is YES then the AudioComponent is added to the resulting array else it will excluded.
    //  		This gives more control to the block provider to filter out the components returned.
    // */
    // - (NSArray<AVAudioUnitComponent *> *)componentsPassingTest:(BOOL(^)(AVAudioUnitComponent *comp, BOOL *stop))testHandler;

    pub fn components_passing_test(
        &self,
        test: fn(&AVAudioUnitComponentRef) -> (bool, ShouldStop),
    ) -> Vec<AVAudioUnitComponent> {
        let block = block::ConcreteBlock::new(
            move |component: &AVAudioUnitComponentRef, stop: *mut BOOL| -> BOOL {
                let (accept, stop_test) = test(component);
                unsafe {
                    *stop = match stop_test {
                        ShouldStop::Continue => NO,
                        ShouldStop::Stop => YES,
                    };
                }

                if accept {
                    YES
                } else {
                    NO
                }
            },
        )
        .copy();

        unsafe {
            let array: *const Object = msg_send![self, componentsPassingTest: block];
            crate::nsarray_to_vec(array)
        }
    }

    // #if AVAUDIOUNITCOMPONENT_HAVE_AUDIOCOMPONENT
    // /*!
    //  @method componentsMatchingDescription:
    //  @abstract	returns an array of AVAudioUnitComponent objects that match the description.
    //  @discussion
    //  		This method provides a mechanism to search for AudioComponents using AudioComponentDescription
    // 		structure. The type, subtype and manufacturer fields are used to search for audio units. A
    //  		value of 0 for any of these fields is a wildcard and returns the first match found.
    // */
    // - (NSArray<AVAudioUnitComponent *> *)componentsMatchingDescription:(AudioComponentDescription)desc;
    pub fn components_matching_description(
        &self,
        desc: AudioComponentDescription,
    ) -> Vec<AVAudioUnitComponent> {
        unsafe {
            let array: *const Object = msg_send![self, componentsMatchingDescription: desc];
            crate::nsarray_to_vec(array)
        }
    }
}
