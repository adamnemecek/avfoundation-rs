use crate::{
    AudioUnitParameterID,
    // AudioUnitParameterValue,
    OSStatus,
};
/// this file contains functions that should eventually be moved to a crate

#[repr(transparent)]
pub struct AudioUnit(std::ffi::c_void);

// extern "C" {

// }
pub type MusicDeviceInstrumentID = u32;
pub type MusicDeviceGroupID = u32;
pub type NoteInstanceID = u32;
pub type AudioUnitParameterValue = f32;

pub struct NoteParamsControlValue {
    pub m_id: AudioUnitParameterID,
    pub m_value: AudioUnitParameterValue,
}

pub struct MusicDeviceNoteParams {
    pub arg_count: u32,
    pub m_pitch: f32,
    pub m_velocity: f32,
    pub m_controls: [NoteParamsControlValue; 1], /* arbitrary length */
}

#[link(name = "AudioToolbox", kind = "framework")]
extern "C" {
    fn MusicDeviceMIDIEvent(
        in_unit: AudioUnit,
        in_status: u32,
        in_data1: u32,
        in_data2: u32,
        in_offset_sample_frame: u32,
    ) -> OSStatus;

    fn MusicDeviceStartNote(
        in_unit: AudioUnit,
        in_instrument: MusicDeviceInstrumentID,
        in_group_id: MusicDeviceGroupID,
        out_note_instance_id: *const NoteInstanceID,
        in_offset_sample_frame: u32,
        in_params: *const MusicDeviceNoteParams,
    ) -> OSStatus;
}
