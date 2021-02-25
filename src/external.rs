/// this file contains functions that should eventually be moved to a crate

#[repr(transparent)]
pub struct AudioUnit(std::ffi::c_void);
