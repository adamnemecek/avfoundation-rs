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
                _ => unreachable!(),
            }
        }
    }

    pub fn has_midi_input(&self) -> bool {
        unsafe {
            match msg_send![self, hasMIDIInput] {
                YES => true,
                NO => false,
                _ => unreachable!(),
            }
        }
    }

    pub fn has_custom_view(&self) -> bool {
        unsafe {
            match msg_send![self, hasCustomView] {
                YES => true,
                NO => false,
                _ => unreachable!(),
            }
        }
    }
}

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
    pub fn shared() -> Self {
        unsafe {
            let class = class!(AVAudioUnitComponentManager);
            msg_send![class, sharedAudioUnitComponentManager]
        }
    }
}

impl AVAudioUnitComponentManagerRef {
    // first return value is accept, second is stop
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
