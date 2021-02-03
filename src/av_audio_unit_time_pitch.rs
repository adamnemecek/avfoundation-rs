// /*
//     File:		AVAudioUnitTimePitch.h
//     Framework:	AVFoundation

//     Copyright (c) 2014-2015 Apple Inc. All Rights Reserved.
// */
// #import <AVFAudio/AVAudioUnitTimeEffect.h>
use crate::prelude::*;

// NS_ASSUME_NONNULL_BEGIN

// /*! @class AVAudioUnitTimePitch
//     @abstract an AVAudioUnitTimeEffect that provides good quality time stretching and pitch shifting
//     @discussion
//         In this time effect, the playback rate and pitch parameters function independently of each other

// */
// API_AVAILABLE(macos(10.10), ios(8.0), tvos(9.0)) API_UNAVAILABLE(watchos)
// @interface AVAudioUnitTimePitch : AVAudioUnitTimeEffect

pub enum AVAudioUnitTimePitchFFI {}

foreign_obj_type! {
    type CType = AVAudioUnitTimePitchFFI;
    pub struct AVAudioUnitTimePitch;
    pub struct AVAudioUnitTimePitchRef;
}

impl AVAudioUnitTimePitchRef {
    // /*! @property rate
    //     @abstract playback rate of the input signal

    //     Range:      1/32 -> 32.0
    //     Default:    1.0
    //     Unit:       Generic
    // */
    // @property (nonatomic) float rate;
    pub fn rate(&self) -> f32 {
        unsafe { msg_send![self, rate] }
    }

    pub fn set_rate(&self, v: f32) {
        unsafe {
            let _: () = msg_send![self, setRate: v];
        }
    }

    // /*! @property pitch
    //     @abstract amount by which the input signal is pitch shifted
    //     @discussion
    //               1 octave  = 1200 cents
    //     1 musical semitone  = 100 cents

    //     Range:      -2400 -> 2400
    //     Default:    0.0
    //     Unit:       Cents
    // */
    // @property (nonatomic) float pitch;
    pub fn pitch(&self) -> f32 {
        unsafe { msg_send![self, pitch] }
    }

    pub fn set_pitch(&self, v: f32) {
        unsafe {
            let _: () = msg_send![self, setPitch: v];
        }
    }

    // /*! @property overlap
    //     @abstract amount of overlap between segments of the input audio signal
    //     @discussion
    //     A higher value results in fewer artifacts in the output signal.
    //     This parameter also impacts the amount of CPU used.

    //     Range:      3.0 -> 32.0
    //     Default:    8.0
    //     Unit:       Generic
    // */
    // @property (nonatomic) float overlap;
    pub fn overlap(&self) -> f32 {
        unsafe { msg_send![self, overlap] }
    }

    pub fn set_overlap(&self, v: f32) {
        unsafe {
            let _: () = msg_send![self, setOverlap: v];
        }
    }

    // @end
}

// NS_ASSUME_NONNULL_END
