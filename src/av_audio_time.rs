use objc::runtime::{
    NO,
    YES,
};

use core_audio_types::AudioTimeStamp;

use crate::AVAudioFramePosition;

pub enum AVAudioTimeFFI {}

foreign_obj_type! {
    type CType = AVAudioTimeFFI;
    pub struct AVAudioTime;
    pub struct AVAudioTimeRef;
}

impl AVAudioTimeRef {
    // pub fn audioTimeStamp(&self) -> AudioTimeStamp {
    //     unsafe { msg_send![self, audioTimeStamp] }
    //  }

    //     /*!	@method initWithAudioTimeStamp:sampleRate:
    // */
    // - (instancetype)initWithAudioTimeStamp: (const AudioTimeStamp *)ts sampleRate: (double)sampleRate;

    // /*! @method initWithHostTime:
    // */
    // - (instancetype)initWithHostTime:(uint64_t)hostTime;

    // /*! @method initWithSampleTime:atRate:
    // */
    // - (instancetype)initWithSampleTime:(AVAudioFramePosition)sampleTime atRate:(double)sampleRate;

    // /*! @method initWithHostTime:sampleTime:atRate:
    // */
    // - (instancetype)initWithHostTime:(uint64_t)hostTime sampleTime:(AVAudioFramePosition)sampleTime atRate:(double)sampleRate;

    // /*! @method timeWithAudioTimeStamp:sampleRate:
    // */
    // + (instancetype)timeWithAudioTimeStamp: (const AudioTimeStamp *)ts sampleRate: (double)sampleRate;

    // /*! @method timeWithHostTime:
    // */
    // + (instancetype)timeWithHostTime:(uint64_t)hostTime;

    // /*! @method timeWithSampleTime:atRate:
    // */
    // + (instancetype)timeWithSampleTime:(AVAudioFramePosition)sampleTime atRate:(double)sampleRate;

    // /*! @method timeWithHostTime:sampleTime:atRate:
    // */
    // + (instancetype)timeWithHostTime:(uint64_t)hostTime sampleTime:(AVAudioFramePosition)sampleTime atRate:(double)sampleRate;

    // /*!	@method hostTimeForSeconds:
    // 	@abstract Convert seconds to host time.
    // */
    // + (uint64_t)hostTimeForSeconds:(NSTimeInterval)seconds;

    // /*!	@method secondsForHostTime:
    // 	@abstract Convert host time to seconds.
    // */
    // + (NSTimeInterval)secondsForHostTime:(uint64_t)hostTime;

    // /*!	@method extrapolateTimeFromAnchor:
    // 	@abstract Converts between host and sample time.
    // 	@param anchorTime
    // 		An AVAudioTime with a more complete AudioTimeStamp than that of the receiver (self).
    // 	@return
    // 		the extrapolated time
    // 	@discussion
    // 		If anchorTime is an AVAudioTime where both host time and sample time are valid,
    // 		and self is another timestamp where only one of the two is valid, this method
    // 		returns a new AVAudioTime copied from self and where any additional valid fields provided by
    // 		the anchor are also valid.

    // 		Note that the anchorTime must have both host and sample time valid, and self must have
    // 		sample rate and at least one of host or sample time valid. Otherwise this method returns nil.

    // <pre>
    // // time0 has a valid audio sample representation, but no host time representation.
    // AVAudioTime *time0 = [AVAudioTime timeWithSampleTime: 0.0 atRate: 44100.0];
    // // anchor has a valid host time representation and sample time representation.
    // AVAudioTime *anchor = [player playerTimeForNodeTime: player.lastRenderTime];
    // // fill in valid host time representation
    // AVAudioTime *fullTime0 = [time0 extrapolateTimeFromAnchor: anchor];
    // </pre>
    // */
    // - (nullable AVAudioTime *)extrapolateTimeFromAnchor:(AVAudioTime *)anchorTime;

    // /*! @property hostTimeValid
    // 	@abstract Whether the hostTime property is valid.
    // */
    // @property (nonatomic, readonly, getter=isHostTimeValid) BOOL hostTimeValid;
    pub fn is_host_time_valid(&self) -> bool {
        unsafe { msg_send![self, isHostTimeValid] }
    }

    // /*! @property hostTime
    // 	@abstract The host time.
    // */
    // @property (nonatomic, readonly) uint64_t hostTime;
    pub fn host_time(&self) -> u64 {
        unsafe { msg_send![self, hostTime] }
    }

    // /*! @property sampleTimeValid
    // 	@abstract Whether the sampleTime and sampleRate properties are valid.
    // */
    // @property (nonatomic, readonly, getter=isSampleTimeValid) BOOL sampleTimeValid;

    pub fn is_sample_time_valid(&self) -> bool {
        unsafe { msg_send![self, isSampleTimeValid] }
    }

    // /*!	@property sampleTime
    // 	@abstract The time as a number of audio samples, as tracked by the current audio device.
    // */
    // @property (nonatomic, readonly) AVAudioFramePosition sampleTime;

    pub fn sample_time(&self) -> AVAudioFramePosition {
        unsafe { msg_send![self, sampleTime] }
    }

    // /*!	@property sampleRate
    // 	@abstract The sample rate at which sampleTime is being expressed.
    // */
    // @property (nonatomic, readonly) double sampleRate;

    pub fn sample_rate(&self) -> f64 {
        unsafe { msg_send![self, sampleRate] }
    }

    // /*! @property audioTimeStamp
    // 	@abstract The time expressed as an AudioTimeStamp structure.
    // 	@discussion
    // 		This may be useful for compatibility with lower-level CoreAudio and AudioToolbox API's.
    // */
    // @property (readonly, nonatomic) AudioTimeStamp audioTimeStamp;
    pub fn audio_time_stamp(&self) -> AudioTimeStamp {
        unsafe { msg_send![self, audioTimeStamp] }
    }
}
