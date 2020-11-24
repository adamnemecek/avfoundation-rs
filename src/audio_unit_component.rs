fn u32_to_string(a: u32) -> String {
    let mut s: [char; 4] = [' ', ' ', ' ', ' '];
    s[3] = (a as u8) as char;
    s[2] = ((a >> 8) as u8) as char;
    s[1] = ((a >> 16) as u8) as char;
    s[0] = ((a >> 24) as u8) as char;
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
        f.write_str(&format!("component_type: {}\n", u32_to_string(self.component_type)))?;
        f.write_str(&format!("component_sub_type: {}\n", u32_to_string(self.component_sub_type)))?;
        f.write_str(&format!("component_manufacturer: {}\n", u32_to_string(self.component_manufacturer)))?;
        f.write_str(&format!("component_flags: {}\n", u32_to_string(self.component_flags)))?;
        f.write_str(&format!("component_flags_mask: {}", self.component_flags_mask))
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

pub enum AVAudioUnitComponentNative {}

foreign_obj_type! {
    type CType = AVAudioUnitComponentNative;
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

    pub fn manufacturer_name(&self) -> &str {
        unsafe {
            let s = msg_send![self, manufacturerName];
            crate::nsstring_as_str(s)
        }
    }

    pub fn audio_component_description(&self) -> AudioComponentDescription {
        unsafe { msg_send![self, audioComponentDescription] }
    }
}
