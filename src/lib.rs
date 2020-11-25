#![link(name = "AVFoundation", kind = "framework")]
#![link(name = "AudioToolbox", kind = "framework")]

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
use objc::runtime::{
    Object,
    NO,
    YES,
};

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

unsafe fn nsarray_to_vec<T: 'static>(array: *const Object) -> Vec<T> {
    let count: NSUInteger = msg_send![array, count];
    let ret = (0..count)
        .map(|i| msg_send![array, objectAtIndex: i])
        // The elements of this array are references---we convert them to owned references
        // (which just means that we increment the reference count here, and it is
        // decremented in the `Drop` impl for `Device`)
        .map(|unit: *mut Object| msg_send![unit, retain])
        .collect();
    let () = msg_send![array, release];
    ret
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

mod au_audio_unit;
pub use au_audio_unit::*;

mod av_audio_buffer;
pub use av_audio_buffer::*;

mod av_audio_channel_layout;
pub use av_audio_channel_layout::*;

mod av_audio_engine;
pub use av_audio_engine::*;

mod av_audio_file;
pub use av_audio_file::*;

mod av_audio_format;
pub use av_audio_format::*;

mod av_audio_io_node;
pub use av_audio_io_node::*;

mod av_audio_node_bus;
pub use av_audio_node_bus::*;

mod av_audio_node;
pub use av_audio_node::*;

mod av_audio_output_node;
pub use av_audio_output_node::*;

mod av_audio_session;
pub use av_audio_session::*;

mod av_audio_time;
pub use av_audio_time::*;

mod av_audio_unit_component_manager;
pub use av_audio_unit_component_manager::*;

mod av_audio_unit_component;
pub use av_audio_unit_component::*;

mod av_audio_unit;
pub use av_audio_unit::*;

mod av_midi_player;
pub use av_midi_player::*;

mod av_audio_player_node;
pub use av_audio_player_node::*;

mod av_audio_sequencer;
pub use av_audio_sequencer::*;
