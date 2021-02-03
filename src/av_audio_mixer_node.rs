use crate::{
    AVAudioNodeBus,
    AVAudioNodeRef,
};
pub enum AVAudioMixerNodeFFI {}

foreign_obj_type! {
    type CType = AVAudioMixerNodeFFI;
    pub struct AVAudioMixerNode;
    pub struct AVAudioMixerNodeRef;
    type ParentType = AVAudioNodeRef;
}

impl AVAudioMixerNode {
    // - (instancetype)init NS_DESIGNATED_INITIALIZER;
    pub fn new() -> Self {
        unsafe {
            let class = class!(AVAudioMixerNode);
            msg_send![class, new]
        }
    }
}

impl AVAudioMixerNodeRef {
    /*! @property outputVolume
        @abstract The mixer's output volume.
        @discussion
            This accesses the mixer's output volume (0.0-1.0, inclusive).
    */
    // @property (nonatomic) float outputVolume;
    pub fn output_volume(&self) -> f32 {
        unsafe { msg_send![self, outputVolume] }
    }

    pub fn set_output_volume(&self, v: f32) {
        unsafe { msg_send![self, setOutputVolume: v] }
    }

    // /*! @property nextAvailableInputBus
    //     @abstract Find an unused input bus.
    //     @discussion
    //         This will find and return the first input bus to which no other node is connected.
    // */
    // @property (nonatomic, readonly) AVAudioNodeBus nextAvailableInputBus;
    pub fn next_available_input_bus(&self) -> AVAudioNodeBus {
        unsafe { msg_send![self, nextAvailableInputBus] }
    }
}

// /*
// 	File:		AVAudioMixerNode.h
// 	Framework:	AVFoundation

// 	Copyright (c) 2014-2015 Apple Inc. All Rights Reserved.
// */
// #import <AVFAudio/AVAudioNode.h>
// #import <AVFAudio/AVAudioMixing.h>

// NS_ASSUME_NONNULL_BEGIN

// /*! @class AVAudioMixerNode
// 	@abstract A node that mixes its inputs to a single output.
// 	@discussion
// 		Mixers may have any number of inputs.

// 		The mixer accepts input at any sample rate and efficiently combines sample rate
// 		conversions. It also accepts any channel count and will correctly upmix or downmix
// 		to the output channel count.
// */
// API_AVAILABLE(macos(10.10), ios(8.0), watchos(2.0), tvos(9.0))
// @interface AVAudioMixerNode : AVAudioNode <AVAudioMixing>

// - (instancetype)init NS_DESIGNATED_INITIALIZER;

// /*! @property outputVolume
// 	@abstract The mixer's output volume.
// 	@discussion
// 		This accesses the mixer's output volume (0.0-1.0, inclusive).
// */
// @property (nonatomic) float outputVolume;

// /*! @property nextAvailableInputBus
// 	@abstract Find an unused input bus.
// 	@discussion
// 		This will find and return the first input bus to which no other node is connected.
// */
// @property (nonatomic, readonly) AVAudioNodeBus nextAvailableInputBus;

// @end

// NS_ASSUME_NONNULL_END
