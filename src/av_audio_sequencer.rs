use crate::AVAudioEngineRef;
use objc::runtime::{
    Object,
    NO,
    YES,
};

pub type AVMusicTimeStamp = f64;

pub enum AVAudioSequencerFFI {}

foreign_obj_type! {
    type CType = AVAudioSequencerFFI;
    pub struct AVAudioSequencer;
    pub struct AVAudioSequencerRef;
}

impl AVAudioSequencer {
    pub fn with_engine(engine: &AVAudioEngineRef) -> Self {
        unsafe {
            let class = class!(AVAudioSequencer);
            let alloc: *const AVAudioSequencerRef = msg_send![class, alloc];
            msg_send![alloc, initWithAudioEngine: engine]
        }
    }
}

impl AVAudioSequencerRef {
    // /*! @method loadFromURL:options:error:
    // 	@abstract Load the file referenced by the URL and add the events to the sequence
    // 	@param fileURL
    //         the URL to the file
    // 	@param options
    // 		determines how the file's contents are mapped to tracks inside the sequence
    // 	@param outError
    //         on exit, if an error occurs, a description of the error
    // */
    // - (BOOL)loadFromURL:(NSURL *)fileURL options:(AVMusicSequenceLoadOptions)options error:(NSError **)outError;

    // /*! @method loadFromData:options:error:
    // 	@abstract Parse the data and add the its events to the sequence
    // 	@param data
    //         the data to load from
    // 	@param options
    // 		determines how the contents are mapped to tracks inside the sequence
    // 	@param outError
    //         on exit, if an error occurs, a description of the error
    // */
    // - (BOOL)loadFromData:(NSData *)data options:(AVMusicSequenceLoadOptions)options error:(NSError **)outError;

    // /*! @method writeToURL:SMPTEResolution:replaceExisting:error:
    // 	@abstract Create and write a MIDI file from the events in the sequence
    // 	@param fileURL
    // 		the path for the file to be created
    // 	@param resolution
    // 		the relationship between "tick" and quarter note for saving to a Standard MIDI File - pass in
    // 		zero to use default - this will be the value that is currently set on the tempo track
    // 	@param replace
    // 		if the file already exists, YES will cause it to be overwritten with the new data.
    // 		Otherwise the call will fail with a permission error.
    // 	@param outError
    //         on exit, if an error occurs, a description of the error
    // 	@discussion
    // 		Only MIDI events are written when writing to the MIDI file. MIDI files are normally beat
    // 		based, but can also have a SMPTE (or real-time rather than beat time) representation.
    // 		The relationship between "tick" and quarter note for saving to Standard MIDI File
    // 		- pass in zero to use default - this will be the value that is currently set on the tempo track
    // */
    // - (BOOL)writeToURL:(NSURL *)fileURL SMPTEResolution:(NSInteger)resolution replaceExisting:(BOOL)replace error:(NSError **)outError;

    // /*!	@method dataWithSMPTEResolution:error:
    // 	@abstract Return a data object containing the events from the sequence
    // 	@discussion
    // 		All details regarding the SMPTE resolution apply here as well.
    // 		The returned NSData lifetime is controlled by the client.
    // */
    // - (NSData *)dataWithSMPTEResolution:(NSInteger)SMPTEResolution error:(NSError **)outError;

    // /*!	@method secondsForBeats:
    // 	@abstract Get the time in seconds for the given beat position (timestamp) in the track
    // */
    // - (NSTimeInterval)secondsForBeats:(AVMusicTimeStamp)beats;
    pub fn beats(&self) -> AVMusicTimeStamp {
        unsafe { msg_send![self, beats] }
    }

    // /*!	@method beatsForSeconds:
    // 	@abstract Get the beat position (timestamp) for the given time in the track
    // */
    // - (AVMusicTimeStamp)beatsForSeconds:(NSTimeInterval)seconds;

    // /* properties */
    // /*!	@property tracks
    // 	@abstract An NSArray containing all the tracks in the sequence
    // 	@discussion
    // 		Track indices count from 0, and do not include the tempo track.
    // */
    // @property (nonatomic, readonly) NSArray<AVMusicTrack *> *tracks;

    // /*!	@property tempoTrack
    // 	@abstract The tempo track
    // 	 @discussion
    // 		 Each sequence has a single tempo track. All tempo events are placed into this track (as well
    // 		 as other appropriate events (for instance, the time signature from a MIDI file). The tempo
    // 		 track can be edited and iterated upon as any other track. Non-tempo events in a tempo track
    // 		 are ignored.
    // */
    // @property (nonatomic, readonly) AVMusicTrack *tempoTrack;

    // /*!	@property userInfo
    // 	@abstract A dictionary containing meta-data derived from a sequence
    // 	@discussion
    // 		The dictionary can contain one or more of the kAFInfoDictionary_* keys
    // 		specified in <AudioToolbox/AudioFile.h>
    // */
    // @property (nonatomic, readonly) NSDictionary<NSString *, id> *userInfo;
    pub fn current_position_in_seconds(&self) -> f64 {
        unsafe { msg_send![self, currentPositionInSeconds] }
    }

    pub fn rate(&self) -> f32 {
        unsafe { msg_send![self, rate] }
    }

    pub fn is_playing(&self) -> bool {
        unsafe {
            match msg_send![self, isPlaying] {
                YES => true,
                NO => false,
                _ => unreachable!(),
            }
        }
    }

    pub fn prepare_to_play(&self) {
        unsafe {
            let _: () = msg_send![self, prepareToPlay];
        }
    }

    pub fn stop(&self) {
        unsafe {
            let _: () = msg_send![self, stop];
        }
    }

    pub fn tempo_track(&self) -> Option<&AVMusicTrackRef> {
        unsafe {
            let ret: *const AVMusicTrackRef = msg_send![self, tempoTrack];
            ret.as_ref()
        }
    }

    pub fn tracks(&self) -> Vec<AVMusicTrack> {
        unsafe {
            let array: *const Object = msg_send![self, tracks];
            crate::nsarray_to_vec(array)
        }
    }
}

pub enum AVMusicTrackFFI {}

foreign_obj_type! {
    type CType = AVMusicTrackFFI;
    pub struct AVMusicTrack;
    pub struct AVMusicTrackRef;
}
