pub struct AVAudioNodeBus {
    pub(crate) inner: i64,
}

impl AVAudioNodeBus {
    pub(crate) fn new(inner: i64) -> Self {
        Self { inner }
    }
}
