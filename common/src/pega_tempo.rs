
    use std::os::raw::c_double;
    use std::os::raw::c_int;
    use std::os::raw::c_long;
    use std::os::raw::c_void;
    #[repr(C)]
    struct timeval {
        tv_sec: c_long,
        tv_usec: c_long,
    }

    extern "C" {
        fn gettimeofday(tv: *mut timeval, tz: *mut c_void) -> c_int;
    }

    static mut SEC: c_double = -1.0;
    pub fn wtime() -> f64 {
        let mut tv = timeval { tv_sec: 0, tv_usec: 0 };
        let _res = unsafe { gettimeofday(&mut tv, std::ptr::null_mut()) };
        unsafe {if SEC < 0.0 { SEC = tv.tv_sec as c_double};};
        (unsafe{tv.tv_sec as c_double - SEC as c_double} + (tv.tv_usec as c_double) * 1e-6)
    }