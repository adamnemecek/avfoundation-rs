// /*
//     File:		AVAudioUnitTimeEffect.h
//     Framework:	AVFoundation

//     Copyright (c) 2014-2015 Apple Inc. All Rights Reserved.
//  */
//  #import <AVFAudio/AVAudioUnit.h>

//  NS_ASSUME_NONNULL_BEGIN

//  /*! @class AVAudioUnitTimeEffect
//      @abstract an AVAudioUnit that processes audio in non real-time
//      @discussion
//      An AVAudioUnitTimeEffect represents an audio unit of type aufc.
//      These effects do not process audio in real-time. The varispeed
//      unit is an example of a time effect unit.

//  */
use crate::{
    prelude::*,
    AudioComponentDescription,
};

pub enum AVAudioUnitTimeEffectFFI {}

foreign_obj_type! {
    type CType = AVAudioUnitTimeEffectFFI;
    pub struct AVAudioUnitTimeEffect;
    pub struct AVAudioUnitTimeEffectRef;
    type ParentType = AVAudioUnitRef;

}
//  API_AVAILABLE(macos(10.10), ios(8.0), tvos(9.0)) API_UNAVAILABLE(watchos)
//  @interface AVAudioUnitTimeEffect : AVAudioUnit

//  #if AVAUDIOUNIT_HAVE_AUDIOUNIT
//  /*! @method initWithAudioComponentDescription:
//      @abstract create an AVAudioUnitTimeEffect object

//      @param audioComponentDescription
//      AudioComponentDescription of the audio unit to be initialized
//      @discussion
//      The componentType must be kAudioUnitType_FormatConverter
//  */
//  - (instancetype)initWithAudioComponentDescription:(AudioComponentDescription)audioComponentDescription;
//  #endif

//  /*! @property bypass
//      @abstract bypass state of the audio unit
//  */
//  @property (nonatomic) BOOL bypass;

//  @end

//  NS_ASSUME_NONNULL_END

impl AVAudioUnitTimeEffect {
    pub fn new_with_component_description(desc: AudioComponentDescription) -> Self {
        todo!()
    }
}

impl AVAudioUnitTimeEffectRef {
    pub fn bypass(&self) -> bool {
        unsafe { msg_send![self, bypass] }
    }

    pub fn set_bypass(&self, v: bool) {
        unsafe {
            let _: () = msg_send![self, setBypass: v];
        }
    }
}
