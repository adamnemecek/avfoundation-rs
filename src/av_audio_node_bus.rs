pub struct AudioNodeBus {
    pub(crate) inner: i64,
}

impl AudioNodeBus {
    pub(crate) fn new(inner: i64) -> Self {
        Self { inner }
    }
}
