#[cfg(not(unix))] compile_error!("The <sys/sysinfo.h> struct is not present for non-unix platforms.");
use std::os::raw::{c_long, c_ulong, c_ushort, c_uint, c_int, c_char};

// https://stackoverflow.com/questions/349889/how-do-you-determine-the-amount-of-linux-system-ram-in-c
#[repr(C)]
pub struct Sysinfo {
    pub uptime: c_long,             /* Seconds since boot */
    pub loads: [c_ulong; 3],        /* 1, 5, and 15 minute load averages */
    pub totalram: c_ulong,          /* Total usable main RAM size */
    pub freeram: c_ulong,           /* Available memory size */
    pub sharedram: c_ulong,         /* Amount of shared memory */
    pub bufferram: c_ulong,         /* Memory used by buffers */
    pub totalswap: c_ulong,         /* Total swap space size */
    pub freeswap: c_ulong,          /* Swap space still available */
    pub procs: c_ushort,            /* Number of current processes */
    pub pad: c_ushort,              /* Padding for m68k */
    pub totalhigh: c_ulong,         /* Total high memory size */
    pub freehigh: c_ulong,          /* Available high memory size */
    pub mem_unit: c_uint,           /* Memory unit size in bytes */
    pub _f: [c_char; 0],            /* Padding: libc doesn't define this field */
}

extern "C" {
    /// the sysinfo struct. Should be the same as it is in C.
    pub fn sysinfo(info: *mut Sysinfo) -> c_int;
}

/// A wrapper to C to get the sysinfo struct.
///
/// The value returned is a Result that when `unwrap`ped (or `match`ed), works the same way as it does in C. If you want to get the uptime, all you have to do is
/// `info.uptime`.
///
/// # examples
///
/// ```rust
/// use sysinfo_dot_h::try_collect;
///
/// let info = try_collect().unwrap();
/// dbg!(info.uptime); // uptime in seconds
/// ```
///
/// # soundness
///
/// Although this function uses `unsafe{}` internally, it shouldn't cause any memory corruption bugs. The data returned by this function is usuable outside of `unsafe{}`.
pub fn try_collect() -> Result<Sysinfo, String> {
    unsafe {
        let mut info: Sysinfo = std::mem::zeroed();
        let result = sysinfo(&mut info);
        if result == 0 {
            return Ok(info);
        } else {
            return Err("Failed to get the Sysinfo struct".to_string());
        }
    }
}

/// A wrapper to C to get the sysinfo struct.
///
/// The value that this function returns works the same way as it does in C. If you want to get the uptime, all you have to do is
/// `info.uptime`. however, this is error prone and may return a malformed value. try calling
/// `try_collect()` if you need a way to account for errors.
///
/// # examples
///
/// ```rust
/// use sysinfo_dot_h::collect;
///
/// let info = collect();
/// dbg!(info.uptime); // uptime in seconds
/// ```
///
/// # soundness
///
/// Although this function uses `unsafe{}` internally, it shouldn't cause any memory corruption bugs. The data returned by this function is usuable outside of `unsafe{}`.
pub fn collect() -> Sysinfo {
    unsafe {
        let mut info: Sysinfo = std::mem::zeroed();
        sysinfo(&mut info);
        return info;
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
    fn fetch_uptime() {
        let result = try_collect();
        debug_assert!(result.is_ok()); // essentally the collect_sysinfo test
        let unwrapped = result.expect("💀");
        println!("fetch_uptime(): {}", unwrapped.uptime);
    }
}

