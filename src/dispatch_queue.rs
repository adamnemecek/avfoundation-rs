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

// use block::Block;
// use std::libc::sleep;

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn dispatch_queue_create(label: *const i8, attr: DispatchQueueAttr) -> DispatchQueue;
    fn dispatch_async(queue: DispatchQueue, block: &block::Block<(), ()>);
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

    pub fn dispatch_async(&self, f: impl Fn() -> () + 'static) {
        let block = block::ConcreteBlock::new(move || {
            f()
        }).copy();
        unsafe {
            dispatch_async(*self, &block);
        }
    }
}