use crate::AVAudioUnitMIDIInstrumentRef;
//
// /*
// 	File:		AVAudioUnitSampler.h
// 	Framework:	AVFoundation
//
// 	Copyright (c) 2014-2015 Apple Inc. All Rights Reserved.
// */
//
// #import <AVFAudio/AVAudioUnitMIDIInstrument.h>
//
// NS_ASSUME_NONNULL_BEGIN
//
// /*!
//  @class AVAudioUnitSampler
//  @abstract Apple's sampler audio unit.
//  @discussion
//     An AVAudioUnit for Apple's Sampler Audio Unit. The sampler can be configured by loading
//     instruments from different types of files such as an aupreset, a DLS or SF2 sound bank,
//     an EXS24 instrument, a single audio file, or an array of audio files.
//
//     The output is a single stereo bus.
// */
// API_AVAILABLE(macos(10.10), ios(8.0), tvos(9.0)) API_UNAVAILABLE(watchos)
// @interface AVAudioUnitSampler : AVAudioUnitMIDIInstrument
//
pub enum AVAudioUnitSamplerFFI {}

foreign_obj_type! {
    type CType = AVAudioUnitSamplerFFI;
    pub struct AVAudioUnitSampler;
    pub struct AVAudioUnitSamplerRef;
    type ParentType = AVAudioUnitMIDIInstrumentRef;
}

impl AVAudioUnitSampler {
    pub fn new() -> Self {
        unsafe {
            let class = class!(AVAudioUnitSampler);
            msg_send![class, new]
        }
    }
}

impl AVAudioUnitSamplerRef {
    // /*! @method loadSoundBankInstrumentAtURL:program:bankMSB:bankLSB:error:
    // 	@abstract loads a specific instrument from the specified sound bank
    // 	@param bankURL
    // 		URL for a Soundbank file. The file can be either a DLS bank (.dls) or a SoundFont bank (.sf2).
    // 	@param program
    // 		program number for the instrument to load
    // 	@param bankMSB
    // 		MSB for the bank number for the instrument to load.  This is usually 0x79 for melodic
    // 		instruments and 0x78 for percussion instruments.
    // 	@param bankLSB
    // 		LSB for the bank number for the instrument to load.  This is often 0, and represents the "bank variation".
    // 	@param outError
    //     	the status of the operation
    // 	@discussion
    //  		This method reads from file and allocates memory, so it should not be called on a real time thread.
    //  */
    // - (BOOL)loadSoundBankInstrumentAtURL:(NSURL *)bankURL program:(uint8_t)program bankMSB:(uint8_t)bankMSB bankLSB:(uint8_t)bankLSB error:(NSError **)outError;
    pub fn load_bank_instrument_at_url(
        &self,
        bank_url: std::path::PathBuf,
        program: u8,
        bank_msb: u8,
        bank_lsb: u8,
    ) -> bool {
        use cocoa_foundation::base::{
            nil,
            NO,
            YES,
        };
        let url = crate::path_to_url(bank_url);

        unsafe {
            match msg_send![self,
               loadSoundBankInstrumentAtURL: url
               program: program
               bankMSB: bank_msb
               bankLSB: bank_lsb
               error: nil
            ] {
                YES => true,
                NO => false,
                _ => unimplemented!(),
            }
        }
    }
    //
    // /*! @method loadInstrumentAtURL:error:
    // 	@abstract configures the sampler by loading the specified preset file.
    // 	@param instrumentURL
    //     	URL to the preset file or audio file
    // 	@param outError
    // 		the status of the operation
    // 	@discussion
    // 		The file can be of one of the following types: Logic/GarageBand EXS24 instrument,
    // 		the Sampler AU's native aupreset, or an audio file (eg. .caf, .aiff, .wav, .mp3).
    //
    // 		If an audio file URL is loaded, it will become the sole sample in a new default instrument.
    // 		Any information contained in the file regarding its keyboard placement (e.g. root key,
    // 		key range) will be used.
    // 		This method reads from file and allocates memory, so it should not be called on a real time thread.
    //
    //  */
    // - (BOOL)loadInstrumentAtURL:(NSURL *)instrumentURL error:(NSError **)outError;
    pub fn load_instrument_at_url(&self, instrument_url: std::path::PathBuf) -> bool {
        use cocoa_foundation::base::{
            nil,
            NO,
            YES,
        };
        let url = crate::path_to_url(instrument_url);
        unsafe {
            match msg_send![self,
               loadInstrumentAtURL: url
               error: nil
            ] {
                YES => true,
                NO => false,
                _ => unimplemented!(),
            }
        }
    }
    //
    // /*! @method loadAudioFilesAtURLs:error:
    // 	@abstract configures the sampler by loading a set of audio files.
    // 	@param audioFiles
    // 		array of URLs for audio files to be loaded
    // 	@param outError
    // 		the status of the operation
    // 	@discussion
    // 		The audio files are loaded into a new default instrument with each audio file placed
    // 		into its own sampler zone. Any information contained in the audio file regarding
    // 		their placement on the keyboard (e.g. root key, key range) will be used.
    // 		This method reads from file and allocates memory, so it should not be called on a real time thread.
    //
    //  */
    // - (BOOL)loadAudioFilesAtURLs:(NSArray<NSURL *> *)audioFiles error:(NSError **)outError;
    //
    pub fn load_audio_files_at_url(&self) -> bool {
        todo!()
    }
    // /*! @property stereoPan
    // 	@abstract
    // 		adjusts the pan for all the notes played.
    // 		Range:     -1 -> +1
    // 		Default:   0
    //  */
    // @property (nonatomic) float     stereoPan;
    pub fn stereo_pan(&self) -> f32 {
        unsafe { msg_send![self, stereoPan] }
    }

    pub fn set_stereo_pan(&self, value: f32) {
        unsafe { msg_send![self, setStereoPan: value] }
    }
    //
    // /*! @property masterGain
    // 	@abstract
    //     	adjusts the gain of all the notes played
    // 		Range:     -90.0 -> +12 db
    // 		Default: 0 db
    //  */
    // @property (nonatomic) float     masterGain;
    pub fn master_gain(&self) -> f32 {
        unsafe { msg_send![self, masterGain] }
    }

    pub fn set_master_gain(&self, value: f32) {
        unsafe { msg_send![self, setMasterGain: value] }
    }
    //
    // /*! @property globalTuning
    // 	@abstract
    // 		adjusts the tuning of all the notes played.
    // 		Range:     -2400 -> +2400 cents
    // 		Default:   0
    //  */
    // @property (nonatomic) float     globalTuning;
    pub fn global_tuning(&self) -> f32 {
        unsafe { msg_send![self, globalTuning] }
    }

    pub fn set_global_tuning(&self, value: f32) {
        unsafe { msg_send![self, setGlobalTuning: value] }
    }
    //
    //
    // @end
    //
    // NS_ASSUME_NONNULL_END
}
