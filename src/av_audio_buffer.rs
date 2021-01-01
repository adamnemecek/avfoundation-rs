use crate::{
    AVAudioFormatRef,
    AVAudioFrameCount,
    AVAudioPacketCount,
};
use cocoa_foundation::foundation::{
    NSInteger,
    NSUInteger,
};
pub enum AVAudioBufferFFI {}

foreign_obj_type! {
    type CType = AVAudioBufferFFI;
    pub struct AVAudioBuffer;
    pub struct AVAudioBufferRef;
}

impl AVAudioBufferRef {
    /// @property format
    /// @abstract The format of the audio in the buffer.

    pub fn format(&self) -> &AVAudioFormatRef {
        unsafe { msg_send![self, format] }
    }
    //    /*!    @property audioBufferList
    //     @abstract The buffer's underlying AudioBufferList.
    //     @discussion
    //         For compatibility with lower-level CoreAudio and AudioToolbox API's, this method accesses
    //         the buffer implementation's internal AudioBufferList. The buffer list structure must
    //         not be modified, though you may modify buffer contents.

    //         The mDataByteSize fields of this AudioBufferList express the buffer's current frameLength.
    // */
    // @property (nonatomic, readonly) const AudioBufferList *audioBufferList;

    // /*!    @property mutableAudioBufferList
    //     @abstract A mutable version of the buffer's underlying AudioBufferList.
    //     @discussion
    //         Some lower-level CoreAudio and AudioToolbox API's require a mutable AudioBufferList,
    //         for example, AudioConverterConvertComplexBuffer.

    //         The mDataByteSize fields of this AudioBufferList express the buffer's current frameCapacity.
    //         If they are altered, you should modify the buffer's frameLength to match.
    // */
    // @property (nonatomic, readonly) AudioBufferList *mutableAudioBufferList;
}

/// @class AVAudioPCMBuffer
/// @abstract A subclass of AVAudioBuffer for use with PCM audio formats.
/// @discussion
///     AVAudioPCMBuffer provides a number of methods useful for manipulating buffers of
///     audio in PCM format.
pub enum AVAudioPCMBufferFFI {}

foreign_obj_type! {
    type CType = AVAudioPCMBufferFFI;
    pub struct AVAudioPCMBuffer;
    pub struct AVAudioPCMBufferRef;
    type ParentType = AVAudioBufferRef;
}

impl AVAudioPCMBuffer {
    //     /*!    @method initWithPCMFormat:frameCapacity:
    //     @abstract Initialize a buffer that is to contain PCM audio samples.
    //     @param format
    //         The format of the PCM audio to be contained in the buffer.
    //     @param frameCapacity
    //         The capacity of the buffer in PCM sample frames.
    //     @discussion
    //         An exception is raised if the format is not PCM.

    //         Returns nil in the following cases:
    //         - if the format has zero bytes per frame (format.streamDescription->mBytesPerFrame == 0)
    //         - if the buffer byte capacity (frameCapacity * format.streamDescription->mBytesPerFrame)
    //           cannot be represented by an uint32_t
    // */
    // - (nullable instancetype)initWithPCMFormat:(AVAudioFormat *)format frameCapacity:(AVAudioFrameCount)frameCapacity NS_DESIGNATED_INITIALIZER;

    pub fn new_with_pcm_format(
        format: &AVAudioFormatRef,
        frame_capacity: AVAudioFrameCount,
    ) -> Self {
        // unsafe {
        //     let class = class!(AVAudioPCMBuffer);
        //     msg_send![class, new]
        // }
        todo!()
    }
}

impl AVAudioPCMBufferRef {
    // /*! @property frameCapacity
    //     @abstract
    //         The buffer's capacity, in audio sample frames.
    // */
    // @property (nonatomic, readonly) AVAudioFrameCount frameCapacity;

    // /*!    @property frameLength
    //     @abstract The current number of valid sample frames in the buffer.
    //     @discussion
    //         You may modify the length of the buffer as part of an operation that modifies its contents.
    //         The length must be less than or equal to the frameCapacity. Modifying frameLength will update
    //         the mDataByteSize in each of the underlying AudioBufferList's AudioBuffer's correspondingly,
    //         and vice versa. Note that in the case of deinterleaved formats, mDataByteSize will refers
    //         the size of one channel's worth of audio samples.
    // */
    // @property (nonatomic) AVAudioFrameCount frameLength;
    pub fn frame_length(&self) -> AVAudioFrameCount {
        unsafe { msg_send![self, frameLength] }
    }

    // /*!    @property stride
    //     @abstract The buffer's number of interleaved channels.
    //     @discussion
    //         Useful in conjunction with floatChannelData etc.
    // */
    // @property (nonatomic, readonly) NSUInteger stride;
    pub fn stride(&self) -> NSUInteger {
        unsafe { msg_send![self, stride] }
    }

    // /*! @property floatChannelData
    //     @abstract Access the buffer's float audio samples.
    //     @discussion
    //         floatChannelData returns pointers to the buffer's audio samples if the buffer's format is
    //         32-bit float, or nil if it is another format.

    //         The returned pointer is to format.channelCount pointers to float. Each of these pointers
    //         is to "frameLength" valid samples, which are spaced by "stride" samples.

    //         If format.interleaved is false (as with the standard deinterleaved float format), then
    //         the pointers will be to separate chunks of memory. "stride" is 1.

    //         If format.interleaved is true, then the pointers will refer into the same chunk of interleaved
    //         samples, each offset by 1 frame. "stride" is the number of interleaved channels.
    // */
    // @property (nonatomic, readonly) float * __nonnull const * __nullable floatChannelData;
    pub fn float_channel_data(&self) -> *const f32 {
        unsafe { msg_send![self, floatChannelData] }
    }

    // /*!    @property int16ChannelData
    //     @abstract Access the buffer's int16_t audio samples.
    //     @discussion
    //         int16ChannelData returns the buffer's audio samples if the buffer's format has 2-byte
    //         integer samples, or nil if it is another format.

    //         See the discussion of floatChannelData.
    // */
    // @property (nonatomic, readonly) int16_t * __nonnull const * __nullable int16ChannelData;
    pub fn int16_channel_data(&self) -> *const i16 {
        unsafe { msg_send![self, int16ChannelData] }
    }

    // /*!    @property int32ChannelData
    //     @abstract Access the buffer's int32_t audio samples.
    //     @discussion
    //         int32ChannelData returns the buffer's audio samples if the buffer's format has 4-byte
    //         integer samples, or nil if it is another format.

    //         See the discussion of floatChannelData.
    // */
    // @property (nonatomic, readonly) int32_t * __nonnull const * __nullable int32ChannelData;
    pub fn int32_channel_data(&self) -> *const i32 {
        unsafe { msg_send![self, int32ChannelData] }
    }
}

/// @class AVAudioCompressedBuffer
/// @abstract A subclass of AVAudioBuffer for use with compressed audio formats.
pub enum AVAudioCompressedBufferFFI {}

foreign_obj_type! {
    type CType = AVAudioCompressedBufferFFI;
    pub struct AVAudioCompressedBuffer;
    pub struct AVAudioCompressedBufferRef;
    type ParentType = AVAudioBufferRef;
}

impl AVAudioCompressedBuffer {
    //     /*!    @method initWithFormat:packetCapacity:maximumPacketSize:
    //     @abstract Initialize a buffer that is to contain compressed audio data.
    //     @param format
    //         The format of the audio to be contained in the buffer.
    //     @param packetCapacity
    //         The capacity of the buffer in packets.
    //     @param maximumPacketSize
    //         The maximum size in bytes of a compressed packet.
    //         The maximum packet size can be obtained from the maximumOutputPacketSize property of an AVAudioConverter configured for encoding this format.
    //     @discussion
    //         An exception is raised if the format is PCM.
    // */
    // - (instancetype)initWithFormat:(AVAudioFormat *)format packetCapacity:(AVAudioPacketCount)packetCapacity maximumPacketSize:(NSInteger)maximumPacketSize;

    // /*!    @method initWithFormat:packetCapacity:
    //     @abstract Initialize a buffer that is to contain constant bytes per packet compressed audio data.
    //     @param format
    //         The format of the audio to be contained in the buffer.
    //     @param packetCapacity
    //         The capacity of the buffer in packets.
    //     @discussion
    //         This fails if the format is PCM or if the format has variable bytes per packet (format.streamDescription->mBytesPerPacket == 0).
    // */
    // - (instancetype)initWithFormat:(AVAudioFormat *)format packetCapacity:(AVAudioPacketCount)packetCapacity;
}

impl AVAudioCompressedBufferRef {
    // /*! @property packetCapacity
    //     @abstract
    //         The number of compressed packets the buffer can contain.
    // */
    // @property (nonatomic, readonly) AVAudioPacketCount packetCapacity;
    pub fn packet_capacity(&self) -> AVAudioPacketCount {
        unsafe { msg_send![self, packetCapacity] }
    }

    // /*!    @property packetCount
    //     @abstract The current number of compressed packets in the buffer.
    //     @discussion
    //         You may modify the packetCount as part of an operation that modifies its contents.
    //         The packetCount must be less than or equal to the packetCapacity.
    // */
    // @property (nonatomic) AVAudioPacketCount packetCount;
    pub fn packet_count(&self) -> AVAudioPacketCount {
        unsafe { msg_send![self, packetCount] }
    }

    // /*!    @property maximumPacketSize
    //     @abstract The maximum size of a compressed packet in bytes.
    // */
    // @property (nonatomic, readonly) NSInteger maximumPacketSize;
    pub fn maximum_packet_size(&self) -> NSInteger {
        unsafe { msg_send![self, maximumPacketSize] }
    }

    // /*! @property data
    //     @abstract Access the buffer's data bytes.
    // */
    // @property (nonatomic, readonly) void *data;

    pub fn data(&self) -> *const std::ffi::c_void {
        unsafe { msg_send![self, data] }
    }

    // /*!
    //     @property byteCapacity
    //     @abstract The buffer's capacity in bytes
    // */
    // @property (nonatomic, readonly) uint32_t byteCapacity API_AVAILABLE(macosx(10.13), ios(11.0), watchos(4.0), tvos(11.0));

    pub fn byte_capacity(&self) -> u32 {
        unsafe { msg_send![self, byteCapacity] }
    }

    // /*!
    //     @property byteLength
    //     @abstract The current number of valid bytes in the buffer.
    //     @discussion
    //         Can be changed as part of an operation that modifies the contents.
    // */
    // @property (nonatomic) uint32_t byteLength API_AVAILABLE(macosx(10.13), ios(11.0), watchos(4.0), tvos(11.0));
    pub fn byte_length(&self) -> u32 {
        unsafe { msg_send![self, byteLength] }
    }

    // /*! @property packetDescriptions
    //     @abstract Access the buffer's array of packet descriptions, if any.
    //     @discussion
    //         If the format has constant bytes per packet (format.streamDescription->mBytesPerPacket != 0), then this will return nil.
    // */
    // @property (nonatomic, readonly, nullable) AudioStreamPacketDescription *packetDescriptions;

    // pub fn packet_descriptions(&self) -> AudioStreamPacketDescription {
    pub fn packet_descriptions(&self) -> ! {
        unsafe { msg_send![self, packetDescriptions] }
    }
}
