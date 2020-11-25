use cocoa_foundation::base::{
    id,
    nil,
};
use objc::runtime::{
    NO,
    YES,
}; //, BOOL, NO, SEL, nil};
   // use cocoa_foundation::foundation::NSURL;

pub enum AVMIDIPlayerFFI {}

foreign_obj_type! {
    type CType = AVMIDIPlayerFFI;
    pub struct AVMIDIPlayer;
    pub struct AVMIDIPlayerRef;
}

impl AVMIDIPlayer {
    pub fn with_url(url: id, bank_url: id) -> Self {
        unsafe {
            let class = class!(AVMIDIPlayer);
            msg_send![class, initWithContentsOfURL: url
                                           bankURL: bank_url
                                             error: nil]
        }
    }

    pub fn with_data() -> Self {
        todo!()
    }
}

impl AVMIDIPlayerRef {
    // pub fn prepare_to_play(&self) {

    // }

    pub fn is_playing(&self) -> bool {
        unsafe {
            match msg_send![self, isPlaying] {
                YES => true,
                NO => false,
                _ => unreachable!(),
            }
        }
    }
}
