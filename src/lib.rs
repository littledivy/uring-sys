#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod test {
    use std::{
        mem::{self, MaybeUninit},
        os::fd::AsRawFd,
        ptr,
    };

    use super::*;

    #[test]
    fn check_version() {
        unsafe {
            assert!(io_uring_check_version(2, 5));
        }
    }

    #[test]
    fn cq_size() {
        unsafe {
            assert!(unsafe_cq_size(4) == 0);
            assert!(unsafe_cq_size(0) == -libc::EINVAL);
        }
    }

    unsafe fn unsafe_cq_size(cq_size: u32) -> i32 {
        let mut params: io_uring_params = mem::zeroed();
        params.flags = IORING_SETUP_CQSIZE;
        params.cq_entries = cq_size;
        let mut ring = MaybeUninit::uninit();
        let ret = io_uring_queue_init_params(4, ring.as_mut_ptr(), &mut params);
        let mut ring = ring.assume_init();
        if cq_size > 0 {
            assert!(params.cq_entries >= cq_size);
        }
        io_uring_queue_exit(&mut ring);
        ret
    }
}
