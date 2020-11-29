pub type AVAudioFramePosition = u64;
pub type AVAudioFrameCount = u32;

pub type AVAudioPacketCount = u32;
pub type AVAudioChannelCount = u32;

pub type AVAudioNodeCompletionHandler = block::Block<(), ()>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AVAudioNodeBus {
    pub(crate) inner: u64,
}

impl AVAudioNodeBus {
    pub fn new(inner: u64) -> Self {
        Self { inner }
    }
}

impl From<u64> for AVAudioNodeBus {
    fn from(a: u64) -> Self {
        Self::new(a)
    }
}