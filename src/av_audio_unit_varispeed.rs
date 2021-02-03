// /*
//     File:		AVAudioUnitVarispeed.h
//     Framework:	AVFoundation

//     Copyright (c) 2014-2015 Apple Inc. All Rights Reserved.
//  */
//  #import <AVFAudio/AVAudioUnitTimeEffect.h>

//  NS_ASSUME_NONNULL_BEGIN
use crate::prelude::*;

pub enum AVAudioUnitVarispeedFFI {}

foreign_obj_type! {
    type CType = AVAudioUnitVarispeedFFI;
    pub struct AVAudioUnitVarispeed;
    pub struct AVAudioUnitVarispeedRef;
    type ParentType = AVAudioUnitTimeEffectRef;
}

impl AVAudioUnitVarispeed {}

impl AVAudioUnitVarispeedRef {
    //  /*! @class AVAudioUnitVarispeed
    //      @abstract an AVAudioUnitTimeEffect that can be used to control the playback rate
    //  */
    //  API_AVAILABLE(macos(10.10), ios(8.0), tvos(9.0)) API_UNAVAILABLE(watchos)
    //  @interface AVAudioUnitVarispeed : AVAudioUnitTimeEffect

    //  /*! @property rate
    //      @abstract controls the playback rate of the audio signal
    //      @discussion
    //      Since this unit resamples the input signal, changing the playback rate also changes the pitch.

    //      i.e. changing the rate to 2.0 results in the output audio playing one octave higher.
    //      Similarly changing the rate to 0.5, results in the output audio playing one octave lower.

    //      The playback rate and pitch can be calculated as
    //                    rate  = pow(2, cents/1200.0)
    //          pitch in cents  = 1200.0 * log2(rate)

    //      Where,    1 octave  = 1200 cents
    //      1 musical semitone  = 100 cents

    //      Range:      0.25 -> 4.0
    //      Default:    1.0
    //      Unit:       Generic
    //  */
    //  @property (nonatomic) float rate;
    pub fn rate(&self) -> f32 {
        unsafe { msg_send![self, rate] }
    }

    pub fn set_rate(&self, v: f32) {
        unsafe {
            let _: () = msg_send![self, setRate: v];
        }
    }

    //  @end
}

//  NS_ASSUME_NONNULL_END
