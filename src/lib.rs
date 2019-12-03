pub mod syscalls;

pub const LIBURING_UDATA_TIMEOUT: libc::__u64 = libc::__u64::max_value();

// sqe opcode constants
pub const IORING_OP_NOP:                u8 = 0;
pub const IORING_OP_READV:              u8 = 1;
pub const IORING_OP_WRITEV:             u8 = 2;
pub const IORING_OP_FSYNC:              u8 = 3;
pub const IORING_OP_READ_FIXED:         u8 = 4;
pub const IORING_OP_WRITE_FIXED:        u8 = 5;
pub const IORING_OP_POLL_ADD:           u8 = 6;
pub const IORING_OP_POLL_REMOVE:        u8 = 7;
pub const IORING_OP_SYNC_FILE_RANGE:    u8 = 8;
pub const IORING_OP_SENDMSG:            u8 = 9;
pub const IORING_OP_RECVMSG:            u8 = 10;
pub const IORING_OP_TIMEOUT:            u8 = 11;
pub const IORING_OP_TIMEOUT_REMOVE:     u8 = 12;
pub const IORING_OP_ACCEPT:             u8 = 13;
pub const IORING_OP_ASYNC_CANCEL:       u8 = 14;
pub const IORING_OP_LINK_TIMEOUT:       u8 = 15;
pub const IORING_OP_CONNECT:            u8 = 16;

// sqe.flags
pub const  IOSQE_FIXED_FILE:            u8 = 1 << 0;	/* use fixed fileset */
pub const  IOSQE_IO_DRAIN:              u8 = 1 << 1;	/* issue after inflight IO */
pub const  IOSQE_IO_LINK:               u8 = 1 << 2;	/* links next sqe */

// sqe.cmd_flags.fsync_flags
pub const IORING_FSYNC_DATASYNC:        u8 = 1 << 0;

// sqe.cmd_flags.timeout_flags
pub const IORING_TIMEOUT_ABS:           u8 = 1 << 0;

// io_uring_setup flags
pub const IORING_SETUP_IOPOLL:	        u8 = 1 << 0;	/* io_context is polled */
pub const IORING_SETUP_SQPOLL:	        u8 = 1 << 1;	/* SQ poll thread */
pub const IORING_SETUP_SQ_AFF:	        u8 = 1 << 2;	/* sq_thread_cpu is valid */
pub const IORING_SETUP_CQSIZE:	        u8 = 1 << 3;    /* app defines CQ size */

// Magic offsets for the application to mmap the data it needs
pub const IORING_OFF_SQ_RING:           u64 = 0;
pub const IORING_OFF_CQ_RING:           u64 = 0x8000000;
pub const IORING_OFF_SQES:              u64 = 0x10000000;

// sq_ring.kflags
pub const IORING_SQ_NEED_WAKEUP:        u8 = 1 << 0;

// io_uring_enter flags
pub const IORING_ENTER_GETEVENTS:       u8 = 1 << 0;
pub const IORING_ENTER_SQ_WAKEUP:       u8 = 1 << 1;

// io_uring_params.features flags
pub const IORING_FEAT_SINGLE_MMAP:      u8 = 1 << 0;
pub const IORING_FEAT_NODROP:           u8 = 1 << 1;


// io_uring_register opcodes and arguments
pub const IORING_REGISTER_BUFFERS:      u8 = 0;
pub const IORING_UNREGISTER_BUFFERS:    u8 = 1;
pub const IORING_REGISTER_FILES:        u8 = 2;
pub const IORING_UNREGISTER_FILES:      u8 = 3;
pub const IORING_REGISTER_EVENTFD:      u8 = 4;
pub const IORING_UNREGISTER_EVENTFD:    u8 = 5;
pub const IORING_REGISTER_FILES_UPDATE: u8 = 6;

#[repr(C)]
pub struct io_uring {
    pub sq: io_uring_sq,
    pub cq: io_uring_cq,
    pub flags: libc::c_uint,
    pub ring_fd: libc::c_int,
}

#[repr(C)]
pub struct io_uring_sq {
    pub khead: *mut libc::c_uint,
    pub ktail: *mut libc::c_uint,
    pub kring_mask: *mut libc::c_uint,
    pub kring_entries: *mut libc::c_uint,
    pub kflags: *mut libc::c_uint,
    pub kdropped: *mut libc::c_uint,
    pub array: *mut libc::c_uint,
    pub sqes: *mut io_uring_sqe,

    pub sqe_head: libc::c_uint,
    pub sqe_tail: libc::c_uint,

    pub ring_sz: libc::size_t,
    pub ring_ptr: *mut libc::c_void,
}

#[repr(C)]
pub struct io_uring_sqe {
    pub opcode: libc::__u8,     /* type of operation for this sqe */
    pub flags: libc::__u8,      /* IOSQE_ flags */
    pub ioprio: libc::__u16,    /* ioprio for the request */
    pub fd: libc::__s32,        /* file descriptor to do IO on */
    pub off_addr2: off_addr2,
    pub addr: libc::__u64,      /* pointer to buffer or iovecs */
    pub len: libc::__u32,       /* buffer size or number of iovecs */
    pub cmd_flags: cmd_flags,
    pub user_data: libc::__u64, /* data to be passed back at completion time */
    pub buf_index: buf_index,   /* index into fixed buffers, if used */
}

#[repr(C)]
pub union off_addr2 {
    pub off: libc::__u64,
    pub addr2: libc::__u64,
}

#[repr(C)]
pub union cmd_flags {
    pub rw_flags: __kernel_rwf_t,
    pub fsync_flags: libc::__u32,
    pub poll_events: libc::__u16,
    pub sync_range_flags: libc::__u32,
    pub msg_flags: libc::__u32,
    pub timeout_flags: libc::__u32,
    pub accept_flags: libc::__u32,
    pub cancel_flags: libc::__u32,
}

#[allow(non_camel_case_types)]
type __kernel_rwf_t = libc::c_int;

#[repr(C)]
pub union buf_index {
    pub buf_index: libc::__u16,
    pub __pad2: [libc::__u64; 3],
}

#[repr(C)]
pub struct io_uring_cq {
    pub khead: *mut libc::c_uint,
    pub ktail: *mut libc::c_uint,
    pub kring_mask: *mut libc::c_uint,
    pub kring_entries: *mut libc::c_uint,
    pub koverflow: *mut libc::c_uint,
    pub cqes: *mut io_uring_cqe,

    pub ring_sz: libc::size_t,
    pub ring_ptr: *mut libc::c_void,
}

#[repr(C)]
pub struct io_uring_cqe {
    pub user_data: libc::__u64, /* sqe->data submission passed back */
    pub res: libc::__s32,       /* result code for this event */
    pub flags: libc::__u32,
}

#[repr(C)]
pub struct io_uring_params {
    pub sq_entries: libc::__u32,
    pub cq_entries: libc::__u32,
    pub flags: libc::__u32,
    pub sq_thread_cpu: libc::__u32,
    pub sq_thread_idle: libc::__u32,
    pub features: libc::__u32,
    pub resv: [libc::__u32; 4],
    pub sq_off: io_sqring_offsets,
    pub cq_off: io_cqring_offsets,
}

#[repr(C)]
pub struct io_sqring_offsets {
    pub head: libc::__u32,
    pub tail: libc::__u32,
    pub ring_mask: libc::__u32,
    pub ring_entries: libc::__u32,
    pub flags: libc::__u32,
    pub dropped: libc::__u32,
    pub array: libc::__u32,
    pub resv1: libc::__u32,
    pub resv2: libc::__u64,
}

#[repr(C)]
pub struct io_cqring_offsets {
    pub head: libc::__u32,
    pub tail: libc::__u32,
    pub ring_mask: libc::__u32,
    pub ring_entries: libc::__u32,
    pub overflow: libc::__u32,
    pub cqes: libc::__u32,
    pub resv: [libc::__u64; 2],
}


#[repr(C)]
pub struct __kernel_timespec {
    pub tv_sec: i64,
    pub tv_nsec: libc::c_longlong,
}

#[link(name = "uring")]
extern {
    pub fn io_uring_queue_init(
        entries: libc::c_uint,
        ring: *mut io_uring,
        flags: libc::c_uint,
    ) -> libc::c_int;

    pub fn io_uring_queue_init_params(
        entries: libc::c_uint,
        ring: *mut io_uring,
        params: *mut io_uring_params,
    ) -> libc::c_int;

    pub fn io_uring_queue_mmap(
        fd: libc::c_int,
        params: *mut io_uring_params,
        ring: *mut io_uring,
    ) -> libc::c_int;

    pub fn io_uring_queue_exit(ring: *mut io_uring);

    pub fn io_uring_peek_batch_cqe(
        ring: *mut io_uring,
        cqes: *mut *mut io_uring_cqe,
        count: libc::c_uint
    ) -> libc::c_uint;

    pub fn io_uring_wait_cqes(
        ring: *mut io_uring,
        cqe_ptr: *mut *mut io_uring_cqe,
        wait_nr: libc::c_uint,
        ts: *const __kernel_timespec,
        sigmask: *const libc::sigset_t
    ) -> libc::c_int;

    pub fn io_uring_wait_cqe_timeout(
        ring: *mut io_uring,
        cqe_ptr: *mut *mut io_uring_cqe,
        ts: *mut __kernel_timespec
    ) -> libc::c_int;

    pub fn io_uring_submit(ring: *mut io_uring) -> libc::c_int;

    pub fn io_uring_submit_and_wait(ring: *mut io_uring, wait_nr: libc::c_uint) -> libc::c_int;

    pub fn io_uring_get_sqe(ring: *mut io_uring) -> *mut io_uring_sqe;

    pub fn io_uring_register_buffers(
        ring: *mut io_uring,
        iovecs: *const libc::iovec,
        nr_iovecs: libc::c_uint,
    ) -> libc::c_int;

    pub fn io_uring_unregister_buffers(ring: *mut io_uring) -> libc::c_int;

    pub fn io_uring_register_files(
        ring: *mut io_uring,
        files: *const libc::c_int,
        nr_files: libc::c_uint,
    ) -> libc::c_int;

    pub fn io_uring_unregister_files(ring: *mut io_uring) -> libc::c_int;

    pub fn io_uring_register_files_update(
        ring: *mut io_uring,
        off: libc::c_uint,
        files: *const libc::c_int,
        nr_files: libc::c_uint,
    ) -> libc::c_int;

    pub fn io_uring_register_eventfd(ring: *mut io_uring, fd: libc::c_int) -> libc::c_int;

    pub fn io_uring_unregister_eventfd(ring: *mut io_uring) -> libc::c_int;
}

#[link(name = "rusturing")]
extern {
    #[link_name = "rust_io_uring_cq_advance"]
    pub fn io_uring_cq_advance(ring: *mut io_uring, nr: libc::c_uint);

    #[link_name = "rust_io_uring_cqe_seen"]
    pub fn io_uring_cqe_seen(ring: *mut io_uring, cqe: *mut io_uring_cqe);

    #[link_name = "rust_io_uring_sqe_set_data"]
    pub fn io_uring_sqe_set_data(sqe: *mut io_uring_sqe, data: *mut libc::c_void);

    #[link_name = "rust_io_uring_cqe_get_data"]
    pub fn io_uring_cqe_get_data(cqe: *mut io_uring_cqe) -> *mut libc::c_void;

    #[link_name = "rust_io_uring_sqe_set_flags"]
    pub fn io_uring_sqe_set_flags(sqe: *mut io_uring_sqe, flags: libc::c_uint);

    #[link_name = "rust_io_uring_prep_rw"]
    pub fn io_uring_prep_rw(
        op: libc::c_int,
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        addr: *const libc::c_void,
        len: libc::c_uint,
        offset: libc::off_t,
    );

    #[link_name = "rust_io_uring_prep_readv"]
    pub fn io_uring_prep_readv(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        iovecs: *const libc::iovec,
        nr_vecs: libc::c_uint,
        offset: libc::off_t,
    );

    #[link_name = "rust_io_uring_prep_read_fixed"]
    pub fn io_uring_prep_read_fixed(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        buf: *mut libc::c_void,
        nbytes: libc::c_uint,
        offset: libc::off_t,
        buf_index: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_writev"]
    pub fn io_uring_prep_writev(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        iovecs: *const libc::iovec,
        nr_vecs: libc::c_uint,
        offset: libc::off_t,
    );

    #[link_name = "rust_io_uring_prep_write_fixed"]
    pub fn io_uring_prep_write_fixed(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        buf: *const libc::c_void,
        nbytes: libc::c_uint,
        offset: libc::off_t,
        buf_index: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_recvmsg"]
    pub fn io_uring_prep_recvmsg(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        msg: *mut libc::msghdr,
        flags: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_sendmsg"]
    pub fn io_uring_prep_sendmsg(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        msg: *const libc::msghdr,
        flags: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_poll_add"]
    pub fn io_uring_prep_poll_add(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        poll_mask: libc::c_short,
    );

    #[link_name = "rust_io_uring_prep_poll_remove"]
    pub fn io_uring_prep_poll_remove(sqe: *mut io_uring_sqe, user_data: *mut libc::c_void);

    #[link_name = "rust_io_uring_prep_fsync"]
    pub fn io_uring_prep_fsync(sqe: *mut io_uring_sqe, fd: libc::c_int, fsync_flags: libc::c_uint);

    #[link_name = "rust_io_uring_prep_nop"]
    pub fn io_uring_prep_nop(sqe: *mut io_uring_sqe);

    #[link_name = "rust_io_uring_prep_timeout"]
    pub fn io_uring_prep_timeout(
        sqe: *mut io_uring_sqe,
        ts: *mut __kernel_timespec,
        count: libc::c_uint,
        flags: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_timeout_remove"]
    pub fn io_uring_prep_timeout_remove(
        sqe: *mut io_uring_sqe,
        user_data: libc::__u64,
        flags: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_accept"]
    pub fn io_uring_prep_accept(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        addr: *mut libc::sockaddr,
        addrlen: *mut libc::socklen_t,
        flags: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_cancel"]
    pub fn io_uring_prep_cancel(
        sqe: *mut io_uring_sqe,
        user_data: *mut libc::c_void,
        flags: libc::c_int,
    );

    #[link_name = "rust_io_uring_prep_link_timeout"]
    pub fn io_uring_prep_link_timeout(
        sqe: *mut io_uring_sqe,
        ts: *mut __kernel_timespec,
        flags: libc::c_uint,
    );

    #[link_name = "rust_io_uring_prep_connect"]
    pub fn io_uring_prep_connect(
        sqe: *mut io_uring_sqe,
        fd: libc::c_int,
        addr: *mut libc::sockaddr,
        addrlen: libc::socklen_t,
    );

    #[link_name = "rust_io_uring_sq_ready"]
    pub fn io_uring_sq_ready(ring: *mut io_uring) -> libc::c_uint;

    #[link_name = "rust_io_uring_sq_space_left"]
    pub fn io_uring_sq_space_left(ring: *mut io_uring) -> libc::c_uint;

    #[link_name = "rust_io_uring_cq_ready"]
    pub fn io_uring_cq_ready(ring: *mut io_uring) -> libc::c_uint;

    #[link_name = "rust_io_uring_wait_cqe_nr"]
    pub fn io_uring_wait_cqe_nr(
        ring: *mut io_uring,
        cqe_ptr: *mut *mut io_uring_cqe,
        wait_nr: libc::c_uint,
    ) -> libc::c_int;

    #[link_name = "rust_io_uring_peek_cqe"]
    pub fn io_uring_peek_cqe(ring: *mut io_uring, cqe_ptr: *mut *mut io_uring_cqe) -> libc::c_int;

    #[link_name = "rust_io_uring_wait_cqe"]
    pub fn io_uring_wait_cqe(ring: *mut io_uring, cqe_ptr: *mut *mut io_uring_cqe) -> libc::c_int;
}
