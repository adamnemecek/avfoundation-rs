use cocoa_foundation::base::{
    id,
    nil,
};
use objc::runtime::{
    NO,
    YES,
}; //, BOOL, NO, SEL, nil};
   // use cocoa_foundation::foundation::NSURL;
use crate::{
    NSError,
    NSTimeInterval,
};

pub(crate) fn path_to_url(path: std::path::PathBuf) -> id {
    use cocoa_foundation::foundation::{
        NSString,
        NSURL,
    };
    unsafe {
        let string = NSString::alloc(nil).init_str(&path.into_os_string().into_string().unwrap());
        NSURL::alloc(nil).initWithString_(string)
    }
}

pub enum AVMIDIPlayerFFI {}

foreign_obj_type! {
    type CType = AVMIDIPlayerFFI;
    pub struct AVMIDIPlayer;
    pub struct AVMIDIPlayerRef;
}

impl AVMIDIPlayer {
    //     /*!    @method initWithContentsOfURL:soundBankURL:error:
    //      @abstract Create a player with the contents of the file specified by the URL.
    //     @discussion
    //          'bankURL' should contain the path to a SoundFont2 or DLS bank to be used
    //          by the MIDI synthesizer.  For OSX it can be set to nil for the default,
    //          but for iOS it must always refer to a valid bank file.
    // */
    // - (nullable instancetype)initWithContentsOfURL:(NSURL *)inURL soundBankURL:(NSURL * __nullable)bankURL error:(NSError **)outError;

    // todo: result<Self, NSError>
    pub fn with_url(path: std::path::PathBuf, bank: std::path::PathBuf) -> Self {
        use objc::runtime::Object;

        let path_url = path_to_url(path);
        let bank_url = path_to_url(bank);

        unsafe {
            let player: Self = msg_send![class!(AVMIDIPlayer), alloc];
            let ptr: *mut Object = msg_send![player.as_ref(), initWithContentsOfURL: path_url
                                                                       soundBankURL: bank_url
                                                                              error: nil];
            // if ptr.is_null() {
            // None
            // } else {
            // Some(intersector)
            // }
            player
        }
        // unsafe {
        //     let class = class!(AVMIDIPlayer);
        //     let id = class.alloc(nil);
        //     msg_send![class, initWithContentsOfURL: path_url
        //                               soundBankURL: bank_url
        //                                      error: nil]
        // }
    }

    // /*!    @method initWithData:soundBankURL:error:
    //     @abstract Create a player with the contents of the data object
    //     @discussion
    //         'bankURL' should contain the path to a SoundFont2 or DLS bank to be used
    //         by the MIDI synthesizer.  For OSX it can be set to nil for the default,
    //         but for iOS it must always refer to a valid bank file.
    //  */
    // - (nullable instancetype)initWithData:(NSData *)data soundBankURL:(NSURL * __nullable)bankURL error:(NSError **)outError;

    #[must_use]
    pub fn with_data(data: &[u8], bank: std::path::PathBuf) -> Result<Self, NSError> {
        // use cocoa_foundation::
        // core_foundation::URL::new();
        todo!();
        // use cocoa_foundation::foundation::{
        //     NSString,
        //     NSURL,
        // };
        // let path_url = unsafe {
        //     let string =
        //         NSString::alloc(nil).init_str(&path.into_os_string().into_string().unwrap());
        //     NSURL::alloc(nil).initWithString_(string)
        // };

        // let bank_url = unsafe {
        //     let string =
        //         NSString::alloc(nil).init_str(&bank.into_os_string().into_string().unwrap());
        //     NSURL::alloc(nil).initWithString_(string)
        // };
        // unsafe {}
        // // println!("path_string {:?}", );
        // // let url = NSURL::URLWithString_();
        // // let path = foundation::URL::
        // todo!()
    }
}

pub type AVMIDIPlayerCompletionHandler = block::RcBlock<(), ()>;

impl AVMIDIPlayerRef {
    // /* transport control */
    // /*! @method prepareToPlay
    //     @abstract Get ready to play the sequence by prerolling all events
    //     @discussion
    //         Happens automatically on play if it has not already been called, but may produce a delay in startup.
    //  */
    // - (void)prepareToPlay;
    pub fn prepare_to_play(&self) {
        unsafe { msg_send![self, prepareToPlay] }
    }

    // /*! @method play:
    //     @abstract Play the sequence.
    //  */
    // - (void)play:(AVMIDIPlayerCompletionHandler __nullable)completionHandler;
    pub fn play(&self, completion_handler: impl Fn() -> ()) {
        let block = block::ConcreteBlock::new(completion_handler);
        unsafe { msg_send![self, play: block] }
    }

    // /*! @method stop
    //     @abstract Stop playing the sequence.
    //  */
    // - (void)stop;
    pub fn stop(&self) {
        unsafe { msg_send![self, stop] }
    }

    // /* properties */
    // /*! @property duration
    //     @abstract The length of the currently loaded file in seconds.
    //  */
    // @property(nonatomic, readonly) NSTimeInterval duration;
    pub fn duration(&self) -> NSTimeInterval {
        unsafe { msg_send![self, duration] }
    }

    // /*! @property playing
    //     @abstract Indicates whether or not the player is playing
    //  */
    // @property(nonatomic, readonly, getter=isPlaying) BOOL playing;
    pub fn is_playing(&self) -> bool {
        unsafe { msg_send![self, isPlaying] }
    }

    // /*! @property rate
    //     @abstract The playback rate of the player
    //     @discussion
    //         1.0 is normal playback rate.  Rate must be > 0.0.
    //  */
    // @property (nonatomic) float rate;
    pub fn rate(&self) -> f32 {
        unsafe { msg_send![self, rate] }
    }

    pub fn set_rate(&mut self, rate: f32) {
        unsafe { msg_send![self, setRate: rate] }
    }

    // /*! @property currentPosition
    //     @abstract The current playback position in seconds
    //     @discussion
    //         Setting this positions the player to the specified time.  No range checking on the time value is done.
    //          This can be set while the player is playing, in which case playback will resume at the new time.
    //  */
    // @property(nonatomic) NSTimeInterval currentPosition;
    pub fn current_position(&self) -> NSTimeInterval {
        unsafe { msg_send![self, currentPosition] }
    }

    pub fn set_current_position(&self, interval: NSTimeInterval) {
        unsafe { msg_send![self, setCurrentPosition: interval] }
    }
}
