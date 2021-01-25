pub enum NSErrorFFI {}

foreign_obj_type! {
    type CType = NSErrorFFI;
    pub struct NSError;
    pub struct NSErrorRef;
}

impl std::fmt::Display for NSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{}", self.inner)
        todo!()
    }
}

impl std::error::Error for NSError {

}

impl NSErrorRef {
    // pub fn new(raw: id) -> Option<Self> {
    //     // if raw == nil { None } else { Some(NSError(raw)) }
    //     todo!()
    // }

    // pub fn code(&self) -> usize {
    //     unsafe { self.0.code() as usize }
    // }

    // pub fn domain(&self) -> &str {
    //     unsafe { CStr::from_ptr(self.0.domain().UTF8String()).to_str().unwrap_or(&"") }
    // }

    // pub fn localized_description(&self) -> &str {
    //     unsafe {
    //         let s = msg_send![self, localizedDescription]
    //     }
    //     unsafe {
    //         CStr::from_ptr(self.0.localizedDescription().UTF8String()).to_str().unwrap_or(&"")
    //     }
    // }
}