use objc::runtime::{
    Object,
    NO,
    YES,
};

use crate::{
    AVAudioEngineRef,
    AVAudioUnitRef,
    NSError,
    NSErrorRef,
    NSTimeInterval,
};
use cocoa_foundation::{
    base::nil,
    foundation::{
        NSInteger,
        NSUInteger,
    },
};

pub struct AVBeatRange {
    pub start: AVMusicTimeStamp,
    pub length: AVMusicTimeStamp,
}

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

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct AVMusicSequenceLoadOptions {
    inner: NSUInteger,
}

impl AVMusicSequenceLoadOptions {
    pub fn empty() -> Self {
        Self { inner: 0 }
    }
}

impl AVAudioSequencerRef {
    // /*! @method loadFromURL:options:error:
    //     @abstract Load the file referenced by the URL and add the events to the sequence
    //     @param fileURL
    //         the URL to the file
    //     @param options
    //         determines how the file's contents are mapped to tracks inside the sequence
    //     @param outError
    //         on exit, if an error occurs, a description of the error
    // */
    // - (BOOL)loadFromURL:(NSURL *)fileURL options:(AVMusicSequenceLoadOptions)options error:(NSError **)outError;
    #[must_use]
    pub fn load_from_url(
        &self,
        url: std::path::PathBuf,
        options: AVMusicSequenceLoadOptions,
    ) -> Result<(), NSError> {
        let url = crate::path_to_url(url);
        unsafe {
            try_bool_objc! { err =>
                msg_send![self, loadFromURL: url
                                    options: options
                                      error: &mut err]
            }
        }
    }

    // /*! @method loadFromData:options:error:
    //     @abstract Parse the data and add the its events to the sequence
    //     @param data
    //         the data to load from
    //     @param options
    //         determines how the contents are mapped to tracks inside the sequence
    //     @param outError
    //         on exit, if an error occurs, a description of the error
    // */
    // - (BOOL)loadFromData:(NSData *)data options:(AVMusicSequenceLoadOptions)options error:(NSError **)outError;
    #[must_use]
    pub fn load_from_data(
        &self,
        data: &[u8],
        options: AVMusicSequenceLoadOptions,
    ) -> Result<(), NSError> {
        // todo!()
        let nsdata = crate::slice_to_nsdata(data);
        unsafe {
            try_bool_objc! { err =>
                msg_send![self, loadFromData: nsdata
                                     options: options
                                       error: &mut err]
            }
        }
    }

    // /*! @method writeToURL:SMPTEResolution:replaceExisting:error:
    //     @abstract Create and write a MIDI file from the events in the sequence
    //     @param fileURL
    //         the path for the file to be created
    //     @param resolution
    //         the relationship between "tick" and quarter note for saving to a Standard MIDI File - pass in
    //         zero to use default - this will be the value that is currently set on the tempo track
    //     @param replace
    //         if the file already exists, YES will cause it to be overwritten with the new data.
    //         Otherwise the call will fail with a permission error.
    //     @param outError
    //         on exit, if an error occurs, a description of the error
    //     @discussion
    //         Only MIDI events are written when writing to the MIDI file. MIDI files are normally beat
    //         based, but can also have a SMPTE (or real-time rather than beat time) representation.
    //         The relationship between "tick" and quarter note for saving to Standard MIDI File
    //         - pass in zero to use default - this will be the value that is currently set on the tempo track
    // */
    // - (BOOL)writeToURL:(NSURL *)fileURL SMPTEResolution:(NSInteger)resolution replaceExisting:(BOOL)replace error:(NSError **)outError;
    #[must_use]
    pub fn write_to_url(
        &self,
        url: std::path::PathBuf,
        resolution: NSInteger,
        replace_existing: bool,
    ) -> Result<(), NSError> {
        let url = crate::path_to_url(url);
        unsafe {
            try_bool_objc! { err =>
                msg_send![self, writeToURL: url
                           SMPTEResolution: resolution
                           replaceExisting: replace_existing
                                     error: &mut err]
            }
        }
    }

    // /*!    @method dataWithSMPTEResolution:error:
    //     @abstract Return a data object containing the events from the sequence
    //     @discussion
    //         All details regarding the SMPTE resolution apply here as well.
    //         The returned NSData lifetime is controlled by the client.
    // */
    // - (NSData *)dataWithSMPTEResolution:(NSInteger)SMPTEResolution error:(NSError **)outError;
    #[must_use]
    pub fn data_with_smpte_resolution(&self) -> Result<(), NSError> {
        todo!()
    }

    // /*!    @method secondsForBeats:
    //     @abstract Get the time in seconds for the given beat position (timestamp) in the track
    // */
    // - (NSTimeInterval)secondsForBeats:(AVMusicTimeStamp)beats;
    pub fn seconds_for_beats(&self, beats: AVMusicTimeStamp) -> NSTimeInterval {
        unsafe { msg_send![self, secondsForBeats: beats] }
    }

    // /*!    @method beatsForSeconds:
    //     @abstract Get the beat position (timestamp) for the given time in the track
    // */
    // - (AVMusicTimeStamp)beatsForSeconds:(NSTimeInterval)seconds;
    pub fn beats_for_seconds(&self, seconds: NSTimeInterval) -> AVMusicTimeStamp {
        unsafe { msg_send![self, beatsForSeconds: seconds] }
    }

    // /* properties */
    // /*!    @property tracks
    //     @abstract An NSArray containing all the tracks in the sequence
    //     @discussion
    //         Track indices count from 0, and do not include the tempo track.
    // */
    // @property (nonatomic, readonly) NSArray<AVMusicTrack *> *tracks;
    pub fn tracks(&self) -> Vec<AVMusicTrack> {
        unsafe {
            let array: *const Object = msg_send![self, tracks];
            crate::nsarray_to_vec(array)
        }
    }

    // /*!    @property tempoTrack
    //     @abstract The tempo track
    //      @discussion
    //          Each sequence has a single tempo track. All tempo events are placed into this track (as well
    //          as other appropriate events (for instance, the time signature from a MIDI file). The tempo
    //          track can be edited and iterated upon as any other track. Non-tempo events in a tempo track
    //          are ignored.
    // */
    // @property (nonatomic, readonly) AVMusicTrack *tempoTrack;
    pub fn tempo_track(&self) -> &AVMusicTrackRef {
        unsafe {
            msg_send![self, tempoTrack]
            // ret.as_ref()
        }
    }

    // /*!    @property userInfo
    //     @abstract A dictionary containing meta-data derived from a sequence
    //     @discussion
    //         The dictionary can contain one or more of the kAFInfoDictionary_* keys
    //         specified in <AudioToolbox/AudioFile.h>
    // */
    // @property (nonatomic, readonly) NSDictionary<NSString *, id> *userInfo;

    // @interface AVAudioSequencer(AVAudioSequencer_Player)

    // /*! @property currentPositionInSeconds
    // 	@abstract The current playback position in seconds
    // 	@discussion
    // 		Setting this positions the sequencer's player to the specified time.  This can be set while
    // 		the player is playing, in which case playback will resume at the new position.
    // */
    // @property(nonatomic) NSTimeInterval currentPositionInSeconds;
    pub fn current_position_in_seconds(&self) -> NSTimeInterval {
        unsafe { msg_send![self, currentPositionInSeconds] }
    }

    pub fn set_current_position_in_seconds(&self, position: NSTimeInterval) {
        unsafe { msg_send![self, setCurrentPositionInSeconds: position] }
    }

    // /*! @property currentPositionInBeats
    // 	@abstract The current playback position in beats
    // 	@discussion
    // 		Setting this positions the sequencer's player to the specified beat.  This can be set while
    // 		the player is playing, in which case playback will resume at the new position.
    // */
    // @property(nonatomic) NSTimeInterval currentPositionInBeats;
    pub fn current_position_in_beats(&self) -> NSTimeInterval {
        unsafe { msg_send![self, currentPositionInBeats] }
    }

    pub fn set_current_position_in_beats(&self, position: NSTimeInterval) {
        unsafe { msg_send![self, setCurrentPositionInBeats: position] }
    }

    // /*! @property playing
    // 	@abstract Indicates whether or not the sequencer's player is playing
    // 	@discussion
    // 		Returns TRUE if the sequencer's player has been started and not stopped. It may have
    // 		"played" past the end of the events in the sequence, but it is still considered to be
    // 		playing (and its time value increasing) until it is explicitly stopped.
    // */
    // @property(nonatomic, readonly, getter=isPlaying) BOOL playing;

    pub fn is_playing(&self) -> bool {
        unsafe { msg_send![self, isPlaying] }
    }

    // /*! @property rate
    // 	@abstract The playback rate of the sequencer's player
    // 	@discussion
    // 		1.0 is normal playback rate.  Rate must be > 0.0.
    // */
    // @property (nonatomic) float rate;

    pub fn rate(&self) -> f32 {
        unsafe { msg_send![self, rate] }
    }

    // /*!	@method hostTimeForBeats:error:
    // 	@abstract Returns the host time that will be (or was) played at the specified beat.
    // 	@discussion
    // 		This call is only valid if the player is playing and will return 0 with an error if the
    // 		player is not playing or if the starting position of the player (its "starting beat") was
    // 		after the specified beat.  The method uses the sequence's tempo map to translate a beat
    // 		time from the starting time and beat of the player.
    // */
    // - (UInt64)hostTimeForBeats:(AVMusicTimeStamp)inBeats error:(NSError **)outError;
    #[must_use]
    pub fn host_time_for_beats(&self, beats: AVMusicTimeStamp) -> Result<u64, NSError> {
        unsafe {
            try_objc! { err =>
                msg_send![self, hostTimeForBeats: beats error: &mut err]
            }
        }
    }

    // /*!	@method beatsForHostTime:error:
    // 	@abstract Returns the beat that will be (or was) played at the specified host time.
    // 	@discussion
    // 		This call is only valid if the player is playing and will return 0 with an error if the
    // 		player is not playing or if the starting time of the player was after the specified host
    // 		time.  The method uses the sequence's tempo map to retrieve a beat time from the starting
    // 		and specified host time.
    // */
    // - (AVMusicTimeStamp)beatsForHostTime:(UInt64)inHostTime error:(NSError **)outError;
    #[must_use]
    pub fn beats_for_host_time(&self, host_time: u64) -> Result<AVMusicTimeStamp, NSError> {
        unsafe {
            try_objc! { err =>
                msg_send![self, beatsForHostTime: host_time error: &mut err]
            }
        }
    }

    // /*! @method prepareToPlay
    // 	@abstract Get ready to play the sequence by prerolling all events
    // 	@discussion
    // 		Happens automatically on play if it has not already been called, but may produce a delay in
    // 		startup.
    // */
    // - (void)prepareToPlay;

    pub fn prepare_to_play(&self) {
        unsafe {
            let _: () = msg_send![self, prepareToPlay];
        }
    }

    // /*!	@method	startAndReturnError:
    // 	@abstract	Start the sequencer's player
    // 	@discussion
    // 		If the AVAudioSequencer has not been prerolled, it will pre-roll itself and then start.
    // 		When the sequencer is associated with an audio engine, the sequencer's player will only
    // 		play if the audio engine is running.
    // */
    // - (BOOL)startAndReturnError:(NSError **)outError;
    #[must_use]
    pub fn start(&self) -> Result<(), NSError> {
        unsafe {
            try_objc! { err =>
                msg_send![self, startAndReturnError: &mut err]
            }
        }
    }

    // /*!	@method	stop
    // 	@abstract	Stop the sequencer's player
    // 	@discussion
    // 		Stopping the player leaves it in an un-prerolled state, but stores the playback position so
    // 		that a subsequent call to startAndReturnError will resume where it left off. This action
    // 		will not stop an associated audio engine.
    // */
    // - (void)stop;
    pub fn stop(&self) {
        unsafe {
            let _: () = msg_send![self, stop];
        }
    }
}

pub enum AVMusicTrackFFI {}

foreign_obj_type! {
    type CType = AVMusicTrackFFI;
    pub struct AVMusicTrack;
    pub struct AVMusicTrackRef;
}

impl AVMusicTrackRef {
    //     /* properties */
    //
    // /*!	@property destinationAudioUnit
    // 	@abstract The AVAudioUnit which will receive the track's events
    // 	@discussion
    // 		This is mutually exclusive with setting a destination MIDIEndpoint.  The AU must already be
    // 		attached to an audio engine, and the track must be part of the AVAudioSequencer associated
    // 		with that engine. When playing, the track will send its events to that AVAudioUnit. The
    // 		destination AU cannot be changed while the track's sequence is playing.
    // */
    // @property (nonatomic, retain, nullable) AVAudioUnit *destinationAudioUnit;
    pub fn destination_audio_unit(&self) -> Option<&AVAudioUnitRef> {
        unsafe { msg_send![self, destinationAudioUnit] }
    }

    pub fn set_destination_audio_unit(&self, audio_unit: Option<&AVAudioUnitRef>) {
        unsafe { msg_send![self, setDestinationAudioUnit: audio_unit] }
    }
    //
    // /*!	@property destinationMIDIEndpoint
    // 	@abstract Set the track's target to the specified MIDI endpoint
    // 	@discussion
    // 		This is mutually exclusive with setting a destination audio unit.  Setting this will remove
    // 		the track's reference to an AVAudioUnit destination.  When played, the track will send its
    // 		events to the MIDI Endpoint.  See also MIDIDestinationCreate.  The endpoint cannot be
    // 		changed while the track's sequence is playing.
    // */
    // #if TARGET_OS_OSX || TARGET_OS_IOS
    // @property (nonatomic) MIDIEndpointRef destinationMIDIEndpoint;
    // #endif
    //
    // /*!	@property loopRange
    // 	@abstract The timestamp range in beats for the loop
    // 	@discussion
    // 		The loop is set by specifying its beat range.
    // */
    // @property (nonatomic) AVBeatRange loopRange;

    pub fn loop_range(&self) -> AVBeatRange {
        unsafe { msg_send![self, loopRange] }
    }

    pub fn set_loop_range(&self, range: AVBeatRange) {
        unsafe { msg_send![self, setLoopRange: range] }
    }
    //
    // /*!	@property loopingEnabled
    // 	@abstract Determines whether or not the track is looped.
    // 	@discussion
    // 		If loopRange has not been set, the full track will be looped.
    // */
    // @property (nonatomic,getter=isLoopingEnabled) BOOL loopingEnabled;
    pub fn is_looping_enabled(&self) -> bool {
        unsafe { msg_send![self, isLoopingEnabled] }
    }

    pub fn set_looping_enabled(&self, v: bool) {
        unsafe { msg_send![self, setLoopingEnabled: v] }
    }
    //
    // typedef NS_ENUM(NSInteger, AVMusicTrackLoopCount) {
    // 	AVMusicTrackLoopCountForever		= -1
    // } NS_ENUM_AVAILABLE(10_10, 8_0);
    //
    // /*!	@property numberOfLoops
    // 	@abstract The number of times that the track's loop will repeat
    // 	@discussion
    // 		If set to AVMusicTrackLoopCountForever, the track will loop forever.
    // 		Otherwise, legal values start with 1.
    // */
    // @property (nonatomic) NSInteger numberOfLoops;

    pub fn number_of_loops(&self) -> NSInteger {
        unsafe { msg_send![self, numberOfLoops] }
    }

    pub fn set_number_of_loops(&self, v: NSInteger) {
        unsafe { msg_send![self, setNumberOfLoops: v] }
    }
    //
    // /*! @property offsetTime
    // 	@abstract Offset the track's start time to the specified time in beats
    // 	@discussion
    // 		By default this value is zero.
    // */
    // @property (nonatomic) AVMusicTimeStamp offsetTime;

    pub fn offset_time(&self) -> AVMusicTimeStamp {
        unsafe { msg_send![self, offsetTime] }
    }

    pub fn set_offset_time(&self, time: AVMusicTimeStamp) {
        unsafe { msg_send![self, setOffsetTime: time] }
    }

    //
    // /*! @property muted
    // 	@abstract Whether the track is muted
    // */
    // @property (nonatomic,getter=isMuted) BOOL muted;
    pub fn is_muted(&self) -> bool {
        unsafe { msg_send![self, isMuted] }
    }

    pub fn set_is_muted(&self, v: bool) {
        unsafe { msg_send![self, setMuted: v] }
    }

    //
    // /*! @property soloed
    // 	@abstract Whether the track is soloed
    // */
    // @property (nonatomic,getter=isSoloed) BOOL soloed;

    pub fn is_soloed(&self) -> bool {
        unsafe { msg_send![self, isSoloed] }
    }

    pub fn set_soloed(&self, v: bool) {
        unsafe { msg_send![self, setSoloed: v] }
    }
    //
    // /*! @property lengthInBeats
    // 	@abstract The total duration of the track in beats
    // 	@discussion
    // 		This will return the beat of the last event in the track plus any additional time that may
    // 		be needed for fading out of ending notes or round a loop point to musical bar, etc.  If this
    // 		has not been set by the user, the track length will always be adjusted to the end of the
    // 		last active event in a track and is adjusted dynamically as events are added or removed.
    //
    // 		The property will return the maximum of the user-set track length, or the calculated length.
    // */
    // @property (nonatomic) AVMusicTimeStamp lengthInBeats;
    //
    pub fn length_in_beats(&self) -> AVMusicTimeStamp {
        unsafe { msg_send![self, lengthInBeats] }
    }

    pub fn set_length_in_beats(&self, length: AVMusicTimeStamp) {
        unsafe { msg_send![self, setLengthInBeats: length] }
    }
    // /*! @property lengthInSeconds
    // 	@abstract The total duration of the track in seconds
    // 	@discussion
    // 		This will return time of the last event in the track plus any additional time that may be
    // 		needed for fading out of ending notes or round a loop point to musical bar, etc.  If this
    // 		has not been set by the user, the track length will always be adjusted to the end of the
    // 		last active event in a track and is adjusted dynamically as events are added or removed.
    //
    // 		The property will return the maximum of the user-set track length, or the calculated length.
    // */
    // @property (nonatomic) NSTimeInterval lengthInSeconds;
    pub fn length_in_seconds(&self) -> NSTimeInterval {
        unsafe { msg_send![self, lengthInSeconds] }
    }

    pub fn set_length_in_seconds(&self, length: NSTimeInterval) {
        unsafe { msg_send![self, setLengthInSeconds: length] }
    }
    //
    //
    // /*! @property timeResolution
    // 	@abstract The time resolution value for the sequence, in ticks (pulses) per quarter note (PPQN)
    // 	@discussion
    // 		If a MIDI file was used to construct the containing sequence, the resolution will be what
    // 		was in the file. If you want to keep a time resolution when writing a new file, you can
    // 		retrieve this value and then specify it when calling -[AVAudioSequencer
    // 		writeToFile:flags:withResolution]. It has no direct bearing on the rendering or notion of
    // 		time of the sequence itself, just its representation in MIDI files. By default this is set
    // 		to either 480 if the sequence was created manually, or a value based on what was in a MIDI
    // 		file if the sequence was created from a MIDI file.
    //
    // 		This can only be retrieved from the tempo track.
    // */
    // @property (nonatomic, readonly) NSUInteger timeResolution;

    pub fn time_resolution(&self) -> NSUInteger {
        unsafe { msg_send![self, timeResolution] }
    }
}
