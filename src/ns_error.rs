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

// #[macro_export]
// macro_rules! ns_err {
//     ($expr: expr) => {
//         let mut err: *mut NSError = std::ptr::null_mut();
//         let res = $expr;
//         if err.is_null() {
//             Ok(res)
//         } else {
//             let e = err.as_ref().unwrap();
//             Err(e.to_owned())
//         }
//     };
// }

impl std::error::Error for NSError {}

impl NSErrorRef {
    // pub fn new(raw: id) -> Option<Self> {
    //     // if raw == nil { None } else { Some(NSError(raw)) }
    //     todo!()
    // }

    pub fn code(&self) -> usize {
        unsafe { msg_send![self, code] }
    }

    pub fn domain(&self) -> &str {
        unsafe { crate::nsstring_as_str(msg_send![self, domain]) }
    }

    pub fn localized_description(&self) -> &str {
        unsafe { crate::nsstring_as_str(msg_send![self, localizedDescription]) }
    }
}
