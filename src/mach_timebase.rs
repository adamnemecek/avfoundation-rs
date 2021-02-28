// use kern_return::KERN_SUCCESS;
use mach::{
    clock_types::NSEC_PER_SEC,
    kern_return,
};

use core_audio_types::AudioTimeStamp;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MachTimebase {
    inner: mach::mach_time::mach_timebase_info,
}

impl MachTimebase {
    pub fn now() -> Self {
        let mut inner = std::mem::MaybeUninit::<mach::mach_time::mach_timebase_info>::uninit();
        let res = unsafe { mach::mach_time::mach_timebase_info(inner.as_mut_ptr()) };
        if res == mach::kern_return::KERN_SUCCESS {
            Self {
                inner: unsafe { inner.assume_init() },
            }
        } else {
            todo!()
        }
    }

    pub fn zero() -> Self {
        Self {
            inner: Default::default()
        }
    }

    pub fn is_zero(&self) -> bool {
        self == &Self::zero()
    }

    #[inline]
    pub fn numer(&self) -> u32 {
        self.inner.numer
    }

    #[inline]
    pub fn denom(&self) -> u32 {
        self.inner.denom
    }
}

// const NSEC_PER_SEC: u32 = 1000000000;

// use mach::
// adapted from mikmidi midi to audio example
fn midi_time_range(timestamp: &AudioTimeStamp, frames: f64) -> (f64, f64) {
    let nsec_per_sec = NSEC_PER_SEC as f64;
    let sample_rate = 44100.0;
    let start_time_interval = timestamp.m_sample_time / sample_rate;
    let duration = frames / sample_rate;

    let midi_start = start_time_interval * nsec_per_sec;
    let midi_end = midi_start + duration * nsec_per_sec;
    (midi_start, midi_end)
}
