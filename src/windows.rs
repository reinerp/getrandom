//! Implementation for Windows
use crate::Error;
use core::{ffi::c_void, mem::MaybeUninit, num::NonZeroU32, ptr};

const BCRYPT_USE_SYSTEM_PREFERRED_RNG: u32 = 0x00000002;

#[link(name = "bcrypt")]
extern "system" {
    fn BCryptGenRandom(
        hAlgorithm: *mut c_void,
        pBuffer: *mut u8,
        cbBuffer: u32,
        dwFlags: u32,
    ) -> i32;
}

// Forbidden when targetting UWP
#[cfg(not(target_vendor = "uwp"))]
#[link(name = "advapi32")]
extern "system" {
    #[link_name = "SystemFunction036"]
    fn RtlGenRandom(RandomBuffer: *mut c_void, RandomBufferLength: u32) -> u8;
}

pub fn getrandom_inner(dest: &mut [MaybeUninit<u8>]) -> Result<(), Error> {
    // Prevent overflow of u32
    for chunk in dest.chunks_mut(u32::max_value() as usize) {
        // BCryptGenRandom was introduced in Windows Vista
        let ret = unsafe {
            BCryptGenRandom(
                ptr::null_mut(),
                chunk.as_mut_ptr() as *mut u8,
                chunk.len() as u32,
                BCRYPT_USE_SYSTEM_PREFERRED_RNG,
            )
        };
        let ret = ret as u32;
        // NT_SUCCESS excludes warnings as well as errors.
        if ret & (1 << 31) != 0 {
            // Failed. Try RtlGenRandom as a fallback.
            #[cfg(not(target_vendor = "uwp"))]
            {
                let ret =
                    unsafe { RtlGenRandom(chunk.as_mut_ptr() as *mut c_void, chunk.len() as u32) };
                if ret != 0 {
                    continue;
                }
            }
            // Preserve the OS-code range without assuming a warning's
            // lower bits are nonzero.
            return Err(NonZeroU32::new(ret & 0x7fff_ffff)
                .map(Error::from)
                .unwrap_or(Error::UNEXPECTED));
        }
    }
    Ok(())
}
