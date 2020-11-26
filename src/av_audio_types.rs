pub type AVAudioFramePosition = u64;
pub type AVAudioFrameCount = u32;

pub type AVAudioPacketCount = u32;
pub type AVAudioChannelCount = u32;

pub type AVAudioNodeCompletionHandler = block::Block<(), ()>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AVAudioNodeBus {
    pub(crate) inner: i64,
}

impl AVAudioNodeBus {
    pub(crate) fn new(inner: i64) -> Self {
        Self { inner }
    }
}
