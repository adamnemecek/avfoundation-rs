/*
    File:  AVAudioSettings.h

    Framework:  AVFoundation

    Copyright 2008-2013 Apple Inc. All rights reserved.
*/

// #import <Foundation/NSObject.h>
// #import <Availability.h>

/* This file's methods are available with iPhone 3.0 or later */

/* property keys - values for all keys defined below are NSNumbers */

/* keys for all formats */
// extern NSString *const AVFormatIDKey                                API_AVAILABLE(macos(10.7), ios(3.0), watchos(3.0), tvos(9.0));  /* value is an integer (format ID) from CoreAudioTypes.h */
// extern NSString *const AVSampleRateKey                                API_AVAILABLE(macos(10.7), ios(3.0), watchos(3.0), tvos(9.0));  /* value is floating point in Hertz */
// extern NSString *const AVNumberOfChannelsKey                         API_AVAILABLE(macos(10.7), ios(3.0), watchos(3.0), tvos(9.0));  /* value is an integer */
// /* linear PCM keys */
// extern NSString *const AVLinearPCMBitDepthKey                        API_AVAILABLE(macos(10.7), ios(3.0), watchos(3.0), tvos(9.0));  /* value is an integer, one of: 8, 16, 24, 32 */
// extern NSString *const AVLinearPCMIsBigEndianKey                     API_AVAILABLE(macos(10.7), ios(3.0), watchos(3.0), tvos(9.0));  /* value is a BOOL */
// extern NSString *const AVLinearPCMIsFloatKey                        API_AVAILABLE(macos(10.7), ios(3.0), watchos(3.0), tvos(9.0));  /* value is a BOOL */
// extern NSString *const AVLinearPCMIsNonInterleaved                  API_AVAILABLE(macos(10.7), ios(4.0), watchos(2.0), tvos(9.0));   /* value is a BOOL */
// #define AVLinearPCMIsNonInterleavedKey AVLinearPCMIsNonInterleaved

// /* audio file type key */
// extern NSString *const AVAudioFileTypeKey                           API_AVAILABLE(macos(10.13), ios(11.0), watchos(4.0), tvos(11.0)); /* value is an integer (audio file type) from AudioFile.h */
// /* encoder property keys */
// extern NSString *const AVEncoderAudioQualityKey                        API_AVAILABLE(macos(10.7), ios(3.0), watchos(3.0), tvos(9.0)); /* value is an integer from enum AVAudioQuality */
// extern NSString *const AVEncoderAudioQualityForVBRKey               API_AVAILABLE(macos(10.9), ios(7.0), watchos(2.0), tvos(9.0)); /* value is an integer from enum AVAudioQuality. only relevant for AVAudioBitRateStrategy_Variable */
//     /* only one of AVEncoderBitRateKey and AVEncoderBitRatePerChannelKey should be provided. */
// extern NSString *const AVEncoderBitRateKey                             API_AVAILABLE(macos(10.7), ios(3.0), watchos(3.0), tvos(9.0)); /* value is an integer. */
// extern NSString *const AVEncoderBitRatePerChannelKey                API_AVAILABLE(macos(10.7), ios(4.0), watchos(2.0), tvos(9.0)); /* value is an integer */
// extern NSString *const AVEncoderBitRateStrategyKey                  API_AVAILABLE(macos(10.9), ios(7.0), watchos(2.0), tvos(9.0)); /* value is an AVAudioBitRateStrategy constant. see below. */
// extern NSString *const AVEncoderBitDepthHintKey                        API_AVAILABLE(macos(10.7), ios(3.0), watchos(3.0), tvos(9.0)); /* value is an integer from 8 to 32 */
// /* sample rate converter property keys */
// extern NSString *const AVSampleRateConverterAlgorithmKey            API_AVAILABLE(macos(10.9), ios(7.0), watchos(2.0), tvos(9.0)); /* value is an AVSampleRateConverterAlgorithm constant. see below. */
// extern NSString *const AVSampleRateConverterAudioQualityKey         API_AVAILABLE(macos(10.9), ios(7.0), watchos(2.0), tvos(9.0)); /* value is an integer from enum AVAudioQuality */
// /* channel layout */
// extern NSString *const AVChannelLayoutKey                           API_AVAILABLE(macos(10.7), ios(4.0), watchos(2.0), tvos(9.0));    /* value is an NSData containing an AudioChannelLayout */
// /* property values */
// /* values for AVEncoderBitRateStrategyKey */
// extern NSString *const AVAudioBitRateStrategy_Constant              API_AVAILABLE(macos(10.9), ios(7.0), watchos(2.0), tvos(9.0));
// extern NSString *const AVAudioBitRateStrategy_LongTermAverage       API_AVAILABLE(macos(10.9), ios(7.0), watchos(2.0), tvos(9.0));
// extern NSString *const AVAudioBitRateStrategy_VariableConstrained   API_AVAILABLE(macos(10.9), ios(7.0), watchos(2.0), tvos(9.0));
// extern NSString *const AVAudioBitRateStrategy_Variable              API_AVAILABLE(macos(10.9), ios(7.0), watchos(2.0), tvos(9.0));

// /* values for AVSampleRateConverterAlgorithmKey */
// extern NSString *const AVSampleRateConverterAlgorithm_Normal        API_AVAILABLE(macos(10.9), ios(7.0), watchos(2.0), tvos(9.0));
// extern NSString *const AVSampleRateConverterAlgorithm_Mastering     API_AVAILABLE(macos(10.9), ios(7.0), watchos(2.0), tvos(9.0));
// extern NSString *const AVSampleRateConverterAlgorithm_MinimumPhase  API_AVAILABLE(macos(10.12), ios(10.0), watchos(3.0), tvos(10.0));

#[allow(non_upper_case_globals)]
pub const AVFormatIDKey: &str = "AVFormatIDKey";

#[allow(non_upper_case_globals)]
pub const AVSampleRateKey: &str = "AVSampleRateKey";

#[allow(non_upper_case_globals)]
pub const AVNumberOfChannelsKey: &str = "AVNumberOfChannelsKey";

#[allow(non_upper_case_globals)]
pub const AVLinearPCMBitDepthKey: &str = "AVLinearPCMBitDepthKey";

#[allow(non_upper_case_globals)]
pub const AVLinearPCMIsBigEndianKey: &str = "AVLinearPCMIsBigEndianKey";

#[allow(non_upper_case_globals)]
pub const AVLinearPCMIsFloatKey: &str = "AVLinearPCMIsFloatKey";

#[allow(non_upper_case_globals)]
pub const AVLinearPCMIsNonInterleaved: &str = "AVLinearPCMIsNonInterleaved";

#[allow(non_upper_case_globals)]
pub const AVAudioFileTypeKey: &str = "AVAudioFileTypeKey";

#[allow(non_upper_case_globals)]
pub const AVEncoderAudioQualityKey: &str = "AVEncoderQualityKey";

#[allow(non_upper_case_globals)]
pub const AVEncoderAudioQualityForVBRKey: &str = "AVEncoderQualityForVBRKey";

#[allow(non_upper_case_globals)]
pub const AVEncoderBitRateKey: &str = "AVEncoderBitRateKey";

#[allow(non_upper_case_globals)]
pub const AVEncoderBitRatePerChannelKey: &str = "AVEncoderBitRatePerChannelKey";

#[allow(non_upper_case_globals)]
pub const AVEncoderBitRateStrategyKey: &str = "AVEncoderBitRateStrategyKey";

#[allow(non_upper_case_globals)]
pub const AVEncoderBitDepthHintKey: &str = "AVEncoderBitDepthHintKey";

#[allow(non_upper_case_globals)]
pub const AVSampleRateConverterAlgorithmKey: &str = "AVSampleRateConverterAlgorithmKey";

#[allow(non_upper_case_globals)]
pub const AVSampleRateConverterAudioQualityKey: &str = "AVSampleRateConverterQualityKey";

#[allow(non_upper_case_globals)]
pub const AVChannelLayoutKey: &str = "AVChannelLayoutKey";

#[allow(non_upper_case_globals)]
pub const AVAudioBitRateStrategy_Constant: &str = "AVAudioBitRateStrategy_Constant";

#[allow(non_upper_case_globals)]
pub const AVAudioBitRateStrategy_LongTermAverage: &str = "AVAudioBitRateStrategy_LongTermAverage";

#[allow(non_upper_case_globals)]
pub const AVAudioBitRateStrategy_VariableConstrained: &str =
    "AVAudioBitRateStrategy_VariableConstrained";

#[allow(non_upper_case_globals)]
pub const AVAudioBitRateStrategy_Variable: &str = "AVAudioBitRateStrategy_Variable";

#[allow(non_upper_case_globals)]
pub const AVSampleRateConverterAlgorithm_Normal: &str = "AVSampleRateConverterAlgorithm_Normal";

#[allow(non_upper_case_globals)]
pub const AVSampleRateConverterAlgorithm_Mastering: &str =
    "AVSampleRateConverterAlgorithm_Mastering";

#[allow(non_upper_case_globals)]
pub const AVSampleRateConverterAlgorithm_MinimumPhase: &str =
    "AVSampleRateConverterAlgorithm_MinimumPhase";

// typedef NS_ENUM(NSInteger, AVAudioQuality) {
//     AVAudioQualityMin    = 0,
//     AVAudioQualityLow    = 0x20,
//     AVAudioQualityMedium = 0x40,
//     AVAudioQualityHigh   = 0x60,
//     AVAudioQualityMax    = 0x7F
// };
#[repr(i64)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AVAudioQuality {
    Min = 0,
    Low = 0x20,
    Medium = 0x40,
    High = 0x60,
    Max = 0x7F,
}
