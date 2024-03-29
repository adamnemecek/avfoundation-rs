#![link(name = "AudioToolbox", kind = "framework")]
#![link(name = "AVFoundation", kind = "framework")]
#![link(name = "CoreAudioKit", kind = "framework")]
#![deny(unused_must_use)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

#[macro_use]
extern crate bitflags;
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

pub use cocoa_foundation::foundation::NSUInteger;
use objc::runtime::Object;

pub fn nsstring_as_str(nsstr: &objc::runtime::Object) -> &str {
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

#[macro_export]
macro_rules! opt_nsstring_as_str {
    ($expr: expr) => {{
        #[allow(unused_assignments)]
        let mut s: *mut objc::runtime::Object = std::ptr::null_mut();
        s = $expr;
        if s.is_null() {
            None
        } else {
            Some(crate::nsstring_as_str(s.as_ref().unwrap()))
        }
    }};
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

/// from metal-rs
// pub fn start_capture(&self, descriptor: &CaptureDescriptorRef) -> Result<(), String> {
//     unsafe {
//         try_objc! { err =>
//             msg_send![self, startCaptureWithDescriptor: descriptor
//                             error: &mut err]
//         }
//     }
// }
macro_rules! try_objc {
    {
        $err: ident => $body:expr
    } => {
        {
            let mut $err: *mut NSError = ::std::ptr::null_mut();
            let value = $body;
            if !$err.is_null() {
                // let desc: *mut Object = msg_send![$err_name, localizedDescription];
                // let compile_error: *const std::os::raw::c_char = msg_send![desc, UTF8String];
                // let message = CStr::from_ptr(compile_error).to_string_lossy().into_owned();
                // let () = msg_send![$err, release];
                let e = $err.as_ref().unwrap();
                return Err(e.to_owned());
                // return Err($err.as_ref().unwrap());
            }
            Ok(value)
        }
    };
}

// macro_rules! get_bool {
//     ($body: expr) => {
//         match $body {
//             cocoa_foundation::base::YES => true,
//             cocoa_foundation::base::NO => false,
//         }
//     }
// }

// macro_rules! to_nil {
//     { $ref: ident => $body:expr } => {
//         use cocoa_foundation::base::nil;
//         if $ref.is_none() {
//             let $ref
//         } else {

//         }

//     }
// }

// macro_rules! try_objc {
//     {
//         $err_name: ident => $body:expr
//     } => {
//         {
//             let mut $err_name: *mut ::objc::runtime::Object = ::std::ptr::null_mut();
//             let value = $body;
//             if !$err_name.is_null() {
//                 let desc: *mut Object = msg_send![$err_name, localizedDescription];
//                 let compile_error: *const std::os::raw::c_char = msg_send![desc, UTF8String];
//                 let message = CStr::from_ptr(compile_error).to_string_lossy().into_owned();
//                 let () = msg_send![$err_name, release];
//                 return Err(message);
//             }
//             value
//         }
//     };
// }

/// some functions have the following signature
/// -(bool)method:(Arg)arg error:(NSError**)error;
/// this is a wrapper for those functions
macro_rules! try_bool_objc {
    {
        $err: ident => $body:expr
    } => {
        {
            let mut $err: *mut NSError = ::std::ptr::null_mut();

            let res: bool = $body;
            if !$err.is_null() {
                // println!("here");
                assert!(!res);
                // let desc: *mut Object = msg_send![$err_name, localizedDescription];
                // let compile_error: *const std::os::raw::c_char = msg_send![desc, UTF8String];
                // let message = CStr::from_ptr(compile_error).to_string_lossy().into_owned();
                // let () = msg_send![$err, release];
                let e = $err.as_ref().unwrap();
                return Err(e.to_owned());
                // return Err($err.as_ref().unwrap());
            }
            assert!(res);
            Ok(())
        }
    };
}

pub mod prelude;

mod aux;
pub use aux::*;

mod au_component;
pub use au_component::*;

mod ns_error;
pub use ns_error::*;

mod au_audio_unit;
pub use au_audio_unit::*;

mod au_audio_unit_implementation;
pub use au_audio_unit_implementation::*;

mod au_parameters;
pub use au_parameters::*;

mod av_audio_buffer;
pub use av_audio_buffer::*;

mod av_audio_channel_layout;
pub use av_audio_channel_layout::*;

mod av_audio_connection_point;
pub use av_audio_connection_point::*;

mod av_audio_converter;
pub use av_audio_converter::*;

mod av_audio_engine;
pub use av_audio_engine::*;

mod av_audio_environment_node;
pub use av_audio_environment_node::*;

mod av_audio_file;
pub use av_audio_file::*;

mod av_audio_format;
pub use av_audio_format::*;

mod av_audio_io_node;
pub use av_audio_io_node::*;

mod av_audio_mixer_node;
pub use av_audio_mixer_node::*;

mod av_audio_mixing;
pub use av_audio_mixing::*;

mod av_audio_node;
pub use av_audio_node::*;

mod av_audio_player_node;
pub use av_audio_player_node::*;

mod av_audio_player;
pub use av_audio_player::*;

mod av_audio_recorder;
pub use av_audio_recorder::*;

mod av_audio_sequencer;
pub use av_audio_sequencer::*;

mod av_audio_session;
pub use av_audio_session::*;

mod av_audio_settings;
pub use av_audio_settings::*;

mod av_audio_sink_node;
pub use av_audio_sink_node::*;

mod av_audio_source_node;
pub use av_audio_source_node::*;

mod av_audio_time;
pub use av_audio_time::*;

mod av_audio_types;
pub use av_audio_types::*;

mod av_audio_unit_component;
pub use av_audio_unit_component::*;

mod av_audio_unit_delay;
pub use av_audio_unit_delay::*;

mod av_audio_unit_distortion;
pub use av_audio_unit_distortion::*;

mod av_audio_unit_effect;
pub use av_audio_unit_effect::*;

mod av_audio_unit_eq;
pub use av_audio_unit_eq::*;

mod av_audio_unit_generator;
pub use av_audio_unit_generator::*;

mod av_audio_unit_midi_instrument;
pub use av_audio_unit_midi_instrument::*;

mod av_audio_unit_reverb;
pub use av_audio_unit_reverb::*;

mod av_audio_unit_sampler;
pub use av_audio_unit_sampler::*;

mod av_audio_unit_time_effect;
pub use av_audio_unit_time_effect::*;

mod av_audio_unit_time_pitch;
pub use av_audio_unit_time_pitch::*;

mod av_audio_unit_varispeed;
pub use av_audio_unit_varispeed::*;

mod av_audio_unit;
pub use av_audio_unit::*;

mod av_midi_player;
pub use av_midi_player::*;

mod av_speech_synthesis;
pub use av_speech_synthesis::*;

mod au_view_controller;
pub use au_view_controller::*;

mod dispatch_queue;
pub use dispatch_queue::*;

mod external;
pub use external::*;

mod mach_timebase;
pub use mach_timebase::*;
