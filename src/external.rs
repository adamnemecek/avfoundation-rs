use crate::{
    AudioUnitParameterID,
    // AudioUnitParameterValue,
    OSStatus,
};
/// this file contains functions that should eventually be moved to a crate

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct AudioUnit(*const std::ffi::c_void);

unsafe impl Send for AudioUnit {}
unsafe impl Sync for AudioUnit {}

// extern "C" {

// }
pub type MusicDeviceInstrumentID = u32;
pub type MusicDeviceGroupID = u32;
pub type NoteInstanceID = u32;
pub type AudioUnitParameterValue = f32;
#[repr(C)]
pub struct NoteParamsControlValue {
    pub m_id: AudioUnitParameterID,
    pub m_value: AudioUnitParameterValue,
}
#[repr(C)]
pub struct MusicDeviceNoteParams {
    pub arg_count: u32,
    pub m_pitch: f32,
    pub m_velocity: f32,
    pub m_controls: [NoteParamsControlValue; 1], /* arbitrary length */
}

// use cocoa::string::CFStringRef;

struct AudioUnitCocoaViewInfo {
    // CFURLRef	mCocoaAUViewBundleLocation;
// bundle: CFStringRef,
// CFStringRef	__nonnull mCocoaAUViewClass[1];
}

#[link(name = "AudioToolbox", kind = "framework")]
extern "C" {
    pub fn MusicDeviceMIDIEvent(
        unit: AudioUnit,
        status: u32,
        data1: u32,
        data2: u32,
        offset_sample_frame: u32,
    ) -> OSStatus;

    pub fn AudioComponentCopyConfigurationInfo() -> OSStatus;

    // pub fn AudioUnitGet

    // fn MusicDeviceStartNote(
    //     in_unit: AudioUnit,
    //     in_instrument: MusicDeviceInstrumentID,
    //     in_group_id: MusicDeviceGroupID,
    //     out_note_instance_id: *const NoteInstanceID,
    //     in_offset_sample_frame: u32,
    //     in_params: *const MusicDeviceNoteParams,
    // ) -> OSStatus;
}

// impl AudioUnit {
//     pub fn midi_event(
//         &self,
//         in_status: u32,
//         in_data1: u32,
//         in_data2: u32,
//         in_offset_sample_frame: u32,
//     ) -> OSStatus {
//         unsafe {
//             MusicDeviceMIDIEvent(
//                 self.0,
//                 in_status,
//                 in_data1,
//                 in_data2,
//                 in_offset_sample_frame,
//             )
//         }
//     }
// }
