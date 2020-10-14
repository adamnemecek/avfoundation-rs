#![link(name = "AVFoundation", kind = "framework")]
// #[macro_use]
// extern crate bitflags;
// #[macro_use]
// extern crate log;
#[macro_use]
extern crate objc;
#[macro_use]
extern crate foreign_types;

// use std::borrow::{Borrow, ToOwned};
// use std::marker::PhantomData;
// use std::mem;
// use std::ops::Deref;
use std::os::raw::c_void;

use cocoa_foundation::foundation::NSUInteger;
use foreign_types::ForeignType;
use objc::runtime::{Object, NO, YES};

fn nsstring_as_str(nsstr: &objc::runtime::Object) -> &str {
    let bytes = unsafe {
        let bytes: *const std::os::raw::c_char = msg_send![nsstr, UTF8String];
        bytes as *const u8
    };
    let len: NSUInteger = unsafe { msg_send![nsstr, length] };
    unsafe {
        let bytes = std::slice::from_raw_parts(bytes, len as usize);
        std::str::from_utf8(bytes).unwrap()
    }
}

fn nsstring_from_str(string: &str) -> *mut objc::runtime::Object {
    const UTF8_ENCODING: usize = 4;

    let cls = class!(NSString);
    let bytes = string.as_ptr() as *const c_void;
    unsafe {
        let obj: *mut objc::runtime::Object = msg_send![cls, alloc];
        let obj: *mut objc::runtime::Object = msg_send![
            obj,
            initWithBytes:bytes
            length:string.len()
            encoding:UTF8_ENCODING
        ];
        let _: *mut c_void = msg_send![obj, autorelease];
        obj
    }
}

#[inline]
fn obj_drop<T>(p: *mut T) {
    unsafe { msg_send![(p as *mut Object), release] }
}

#[inline]
fn obj_clone<T: 'static>(p: *mut T) -> *mut T {
    unsafe { msg_send![(p as *mut Object), retain] }
}

#[macro_use]
macro_rules! foreign_obj_type {
    {type CType = $raw_ident:ident;
    pub struct $owned_ident:ident;
    pub struct $ref_ident:ident;
    type ParentType = $parent_ref:ident;
    } => {
        foreign_obj_type! {
            type CType = $raw_ident;
            pub struct $owned_ident;
            pub struct $ref_ident;
        }

        impl ::std::ops::Deref for $ref_ident {
            type Target = $parent_ref;

            fn deref(&self) -> &$parent_ref {
                unsafe { &*(self as *const $ref_ident as *const $parent_ref)  }
            }
        }
    };
    {type CType = $raw_ident:ident;
    pub struct $owned_ident:ident;
    pub struct $ref_ident:ident;
    } => {
        foreign_type! {
            type CType = $raw_ident;
            fn drop = crate::obj_drop;
            fn clone = crate::obj_clone;
            pub struct $owned_ident;
            pub struct $ref_ident;
        }

        unsafe impl ::objc::Message for $raw_ident {
        }
        unsafe impl ::objc::Message for $ref_ident {
        }

        impl ::std::fmt::Debug for $ref_ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                unsafe {
                    let string: *mut ::objc::runtime::Object = msg_send![self, debugDescription];
                    write!(f, "{}", crate::nsstring_as_str(&*string))
                }
            }
        }

        impl ::std::fmt::Debug for $owned_ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::ops::Deref::deref(self).fmt(f)
            }
        }
    };
}

mod audio_buffer;
pub use audio_buffer::*;

mod audio_channel_layout;
pub use audio_channel_layout::*;

mod audio_engine;
pub use audio_engine::*;

mod audio_io_node;
pub use audio_io_node::*;

mod audio_output_node;
pub use audio_output_node::*;

mod audio_file;
pub use audio_file::*;

mod audio_format;
pub use audio_format::*;

mod audio_node;
pub use audio_node::*;

mod audio_node_bus;
pub use audio_node_bus::*;

mod audio_time;
pub use audio_time::*;

mod audio_unit;
pub use audio_unit::*;

mod audio_unit_component;
pub use audio_unit_component::*;

mod audio_unit_component_manager;
pub use audio_unit_component_manager::*;
