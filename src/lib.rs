//! This crate uses raw FFI bindings to easily and quickly get and use the <sys/sysinfo.h> struct.
//! Please read the [README](https://github.com/WilliamAnimate/sysinfo_dot_h?tab=readme-ov-file#sysinfo-dot-h) if you want to know more.
//!
//! Internally, this crate calls `unsafe {}` because of FFI. You, the programmer, are ultimately
//! responsible for any downtime in prod or similar
#[cfg(not(target_os = "linux"))] compile_error!("The <sys/sysinfo.h> calls are only present in Linux.");
use std::os::raw::{c_long, c_ulong, c_ushort, c_uint, c_int, c_char};

// https://stackoverflow.com/questions/349889/how-do-you-determine-the-amount-of-linux-system-ram-in-c
#[repr(C)]
#[allow(non_camel_case_types)] // if uppercase, this may be a breaking change. fix in v1.
#[derive(Debug, Copy, Clone)]
pub struct sysinfo {
    /// Seconds since boot
    pub uptime: c_long,
    /// 1, 5, and 15 minute load averages
    pub loads: [c_ulong; 3],
    /// Total usable main RAM size
    pub totalram: c_ulong,
    /// Available memory size
    pub freeram: c_ulong,
    /// Amount of shared memory
    pub sharedram: c_ulong,
    /// Memory used by buffers
    pub bufferram: c_ulong,
    /// Total swap space size
    pub totalswap: c_ulong,
    /// Swap space still available
    pub freeswap: c_ulong,
    /// Number of current processes
    pub procs: c_ushort,
    /// Padding for m68k
    pub pad: c_ushort,
    /// Total high memory size
    pub totalhigh: c_ulong,
    /// Available high memory size
    pub freehigh: c_ulong,
    /// Memory unit size in bytes
    pub mem_unit: c_uint,
    /// Padding (you cant access this)
    _f: [c_char; 0],
}

extern "C" {
    /// The sysinfo struct. Should be the same as it is in C.
    ///
    /// # Available fields:
    ///
    /// - uptime: Seconds since boot
    /// - totalram: total usuable main RAM size (in bytes)
    /// - freeram: unused ram size (in bytes). freeram != available memory
    /// - sharedram: amount of shared memory (in bytes)
    /// - bufferram: memory used by buffers (in bytes)
    /// - totalswap: total swap memory (in bytes)
    /// - freeswap: available swap space (in bytes)
    /// - procs: number of current processes
    /// - pad: padding for m68k
    /// - totalhigh: total high memory size
    /// - freehigh: available high memory size
    /// - mem_unit: memory unit size in bytes
    pub fn sysinfo(info: *mut sysinfo) -> c_int;
}

/// A wrapper to C to get the sysinfo struct.
///
/// The value returned is a Result that when `unwrap`ped (or `match`ed), works the same way as it does in C. If you want to get the uptime, all you have to do is
/// `info.uptime`.
///
/// # Examples
///
/// ```rust
/// use sysinfo_dot_h::try_collect;
///
/// let info = try_collect().unwrap();
/// dbg!(info.uptime); // uptime in seconds
/// ```
///
/// # Errors
///
/// If the FFI call to `sysinfo()` fails, this function will return an `Err` type. This is
/// unlikely to occur but heee's a heads up.
///
/// # Safety
///
/// Although this function uses `unsafe{}` internally, it shouldn't cause any memory corruption bugs. The data returned by this function is usuable outside of `unsafe{}`.
pub fn try_collect() -> Result<sysinfo, String> {
    unsafe {
        let mut info: sysinfo = std::mem::zeroed();
        let result = sysinfo(&mut info);
        if result == 0 {
            Ok(info)
        } else {
           Err("Failed to get the sysinfo struct".to_string())
        }
    }
}

/// A wrapper to C to get the sysinfo struct.
///
/// The value that this function returns works the same way as it does in C. If you want to get the uptime, all you have to do is
/// `info.uptime`. however, this is error prone and may return a malformed value if the call to
/// `sysinfo()` fails. Try calling `try_collect()` if you need a way to account for errors.
///
/// # Examples
///
/// ```rust
/// use sysinfo_dot_h::collect;
///
/// let info = collect();
/// dbg!(info.uptime); // uptime in seconds
/// ```
///
/// # Safety
///
/// Although this function uses `unsafe{}` internally, it shouldn't cause any memory corruption bugs. The data returned by this function is usuable outside of `unsafe{}`.
#[must_use] pub fn collect() -> sysinfo {
    unsafe {
        let mut info: sysinfo = std::mem::zeroed();
        sysinfo(&mut info);
        info
    }
}

// cargo test -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_collect_sysinfo() {
        let result = try_collect();
        assert!(result.is_ok());
    }

    #[test]
    fn try_fetch_uptime() {
        let result = try_collect();
        debug_assert!(result.is_ok()); // essentally the collect_sysinfo test
        let unwrapped = result.expect("💀");
        println!("try_fetch_uptime(): {}", unwrapped.uptime);
    }

    #[test]
    fn fetch_uptime() {
        let result = collect();
        println!("fetch_uptime(): {}", result.uptime);
    }
}

