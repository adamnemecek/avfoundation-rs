use foreign_types::ForeignTypeRef;

/// auxilliary functions
pub fn run_main_loop() {
    use cocoa_foundation::base::id;
    use cocoa_foundation::foundation::NSRunLoop;
    // let l = NSRunLoop::currentRunLoop();
    let run_loop: id = unsafe { NSRunLoop::currentRunLoop() };
    unsafe {
        let _: () = msg_send![run_loop, run];
    };
}


pub fn class_name(r: &objc::runtime::Object)  -> &str {
    let cls= r.class();
    cls.name()
}

pub fn cls_name(a: &crate::AVAudioUnitRef) -> &str {
    let r: *const objc::runtime::Object = a.as_ptr() as _;
    unsafe {
        class_name(r.as_ref().unwrap())
    }
}
