use libc::{c_long, rusage, suseconds_t, timeval, time_t, getrusage, RUSAGE_SELF};

pub fn get_time_in_userspace_usecs() -> u64 {
    let mut usage = rusage {
        ru_utime: timeval{ tv_sec: 0 as time_t, tv_usec: 0 as suseconds_t, },
        ru_stime: timeval{ tv_sec: 0 as time_t, tv_usec: 0 as suseconds_t, },
        ru_maxrss: 0 as c_long,
        ru_ixrss: 0 as c_long,
        ru_idrss: 0 as c_long,
        ru_isrss: 0 as c_long,
        ru_minflt: 0 as c_long,
        ru_majflt: 0 as c_long,
        ru_nswap: 0 as c_long,
        ru_inblock: 0 as c_long,
        ru_oublock: 0 as c_long,
        ru_msgsnd: 0 as c_long,
        ru_msgrcv: 0 as c_long,
        ru_nsignals: 0 as c_long,
        ru_nvcsw: 0 as c_long,
        ru_nivcsw: 0 as c_long,
    };

    unsafe { getrusage(RUSAGE_SELF, (&mut usage) as *mut rusage); }

    let secs = usage.ru_utime.tv_sec as u64;
    let usecs = usage.ru_utime.tv_usec as u64;

    (secs * 1_000_000) + usecs
}

macro_rules! bench {
    ($n:expr, $b:block) => {{
        use bencher::get_time_in_userspace_usecs;
        let start_time = get_time_in_userspace_usecs();
        for _ in 0..$n {
            $b;
        }
        let end_time = get_time_in_userspace_usecs();

        end_time - start_time
    }};
}
