// use block::{
//     Block,
//     // RcBlock,
// };

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct DispatchQueue(*mut std::ffi::c_void);

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct DispatchQueueAttr(*mut std::ffi::c_void);

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct DispatchTime(u64);

impl DispatchTime {
    pub fn now() -> Self {
        Self(0)
    }

    pub fn new(t: u64) -> Self {
        Self(t)
    }
}

// impl From<std::time::Duration> for DispatchTime {
//     fn from(t: std::time::Duration) -> Self {
//         Self(t.as_nanos() as u64)
//     }
// }

// use block::Block;
// use std::libc::sleep;

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn dispatch_queue_create(label: *const i8, attr: DispatchQueueAttr) -> DispatchQueue;
    fn dispatch_async(queue: DispatchQueue, block: &block::Block<(), ()>);
    fn dispatch_after(when: DispatchTime, queue: DispatchQueue, block: &block::Block<(), ()>);
}

impl DispatchQueue {
    pub fn new(label: &str) -> Self {
        unsafe {
            dispatch_queue_create(
                b"label\0".as_ptr() as *const _,
                DispatchQueueAttr(std::ptr::null_mut()),
            )
        }
    }

    pub fn dispatch_after(&self, after: DispatchTime, f: impl Fn() -> () + 'static) {
        let block = block::ConcreteBlock::new(move || f()).copy();
        unsafe { dispatch_after(after, *self, &block) }
    }

    pub fn dispatch_async(&self, f: impl Fn() -> () + 'static) {
        let block = block::ConcreteBlock::new(move || f()).copy();
        unsafe {
            dispatch_async(*self, &block);
        }
    }
}
