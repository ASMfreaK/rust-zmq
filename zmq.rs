//! Module: zmq

export context;
export socket;
export socket_util;
export socket_type;
export PAIR;
export PUB;
export SUB;
export REQ;
export REP;
export DEALER;
export ROUTER;
export PULL;
export PUSH;
export XPUB;
export XSUB;
export DONTWAIT;
export SNDMORE;
export version;
export init;
export POLLIN;
export POLLOUT;
export POLLERR;
export poll;
export error;
export to_str;

/// The ZMQ container that manages all the sockets
type context_t = *c_void;

/// A ZMQ socket
type socket_t = *c_void;

/// A message
type msg = {
    content: *c_void,
    flags: u8,
    vsm_size: u8,
    vsm_data0: u32,
    vsm_data1: u32,
    vsm_data2: u32,
    vsm_data3: u32,
    vsm_data4: u32,
    vsm_data5: u32,
    vsm_data6: u32,
};

extern mod zmq {
    fn zmq_version(major: *c_int, minor: *c_int, patch: *c_int);

    fn zmq_init(io_threads: c_int) -> context_t;
    fn zmq_term(ctx: context_t) -> c_int;

    fn zmq_errno() -> c_int;
    fn zmq_strerror(errnum: c_int) -> *c_char;

    fn zmq_socket(ctx: context_t, typ: c_int) -> socket_t;
    fn zmq_close(socket: socket_t) -> c_int;

    fn zmq_getsockopt(
            socket: socket_t,
            opt: c_int,
            optval: *c_void,
            size: *size_t) -> c_int;
    fn zmq_setsockopt(
            socket: socket_t,
            opt: c_int,
            optval: *c_void,
            size: size_t) -> c_int;

    fn zmq_bind(socket: socket_t, endpoint: *c_char) -> c_int;
    fn zmq_connect(socket: socket_t, endpoint: *c_char) -> c_int;

    fn zmq_msg_init(msg: msg) -> c_int;
    fn zmq_msg_init_size(msg: msg, size: size_t) -> c_int;
    fn zmq_msg_data(msg: msg) -> *u8;
    fn zmq_msg_size(msg: msg) -> size_t;
    fn zmq_msg_close(msg: msg) -> c_int;

    fn zmq_send(socket: socket_t, msg: msg, flags: c_int) -> c_int;
    fn zmq_recv(socket: socket_t, msg: msg, flags: c_int) -> c_int;

    fn zmq_poll(items: *pollitem, nitems: c_int, timeout: c_long) -> c_int;
}

/// Socket types
enum socket_type {
    PAIR = 0,
    PUB = 1,
    SUB = 2,
    REQ = 3,
    REP = 4,
    DEALER = 5,
    ROUTER = 6,
    PULL = 7,
    PUSH = 8,
    XPUB = 9,
    XSUB = 10,
}

const DONTWAIT : int = 1;
const SNDMORE : int = 2;

mod constants {
    const ZMQ_HWM : c_int = 1i32;
    const ZMQ_SNDHWM : c_int = 1i32;
    const ZMQ_RCVHWM : c_int = 1i32;
    const ZMQ_SWAP : c_int = 3i32;
    const ZMQ_AFFINITY : c_int = 4i32;
    const ZMQ_IDENTITY : c_int = 5i32;
    const ZMQ_SUBSCRIBE : c_int = 6i32;
    const ZMQ_UNSUBSCRIBE : c_int = 7i32;
    const ZMQ_RATE : c_int = 8i32;
    const ZMQ_RECOVERY_IVL : c_int = 9i32;
    const ZMQ_MCAST_LOOP : c_int = 10i32;
    const ZMQ_SNDBUF : c_int = 11i32;
    const ZMQ_RCVBUF : c_int = 12i32;
    const ZMQ_RCVMORE : c_int = 13i32;
    const ZMQ_FD : c_int = 14i32;
    const ZMQ_EVENTS : c_int = 15i32;
    const ZMQ_TYPE : c_int = 16i32;
    const ZMQ_LINGER : c_int = 17i32;
    const ZMQ_RECONNECT_IVL : c_int = 18i32;
    const ZMQ_BACKLOG : c_int = 19i32;
    const ZMQ_RECOVERY_IVL_MSEC : c_int = 20i32;
    const ZMQ_RECONNECT_IVL_MAX : c_int = 21i32;

    const ZMQ_MAX_VSM_SIZE : c_int = 30i32;
    const ZMQ_DELIMITER : c_int = 31i32;
    const ZMQ_VSM : c_int = 32i32;

    const ZMQ_MSG_MORE : c_int = 1i32;
    const ZMQ_MSG_SHARED : c_int = 128i32;
    const ZMQ_MSG_MASK : c_int = 129i32;

    const ZMQ_HAUSNUMERO : c_int = 156384712i32;
}

enum error {
    ENOTSUP = 156384712 + 1, //ZMQ_HAUSNUMERO + 1,
    EPROTONOSUPPORT = 156384712 + 2, //ZMQ_HAUSNUMERO + 2,
    ENOBUFS = 156384712 + 3, //ZMQ_HAUSNUMERO + 3,
    ENETDOWN = 156384712 + 4, //ZMQ_HAUSNUMERO + 4,
    EADDRINUSE = 156384712 + 5, //ZMQ_HAUSNUMERO + 5,
    EADDRNOTAVAIL = 156384712 + 6, //ZMQ_HAUSNUMERO + 6,
    ECONNREFUSED = 156384712 + 7, //ZMQ_HAUSNUMERO + 7,
    EINPROGRESS = 156384712 + 8, //ZMQ_HAUSNUMERO + 8,
    ENOTSOCK = 156384712 + 9, //ZMQ_HAUSNUMERO + 9,

    EFSM = 156384712 + 51, //ZMQ_HAUSNUMERO + 51,
    ENOCOMPATPROTO = 156384712 + 52, //ZMQ_HAUSNUMERO + 52,
    ETERM = 156384712 + 53, //ZMQ_HAUSNUMERO + 53,
    EMTHREAD = 156384712 + 54, //ZMQ_HAUSNUMERO + 54,
}

// Return the current zeromq version.
fn version() -> (int, int, int) {
    let major = 0i32;
    let minor = 0i32;
    let patch = 0i32;
    zmq::zmq_version(
        ptr::addr_of(major),
        ptr::addr_of(minor),
        ptr::addr_of(patch));
    (major as int, minor as int, patch as int)
}

// Create a zeromq context.
fn init(io_threads: int) -> result<Cell<context>, error> unsafe {
    let ctx = zmq::zmq_init(io_threads as i32);

    if unsafe::reinterpret_cast(ctx) == 0 {
        return err(errno_to_error());
    }

    ok(Cell(context(ctx)))
}

struct context {
    priv {
        let ctx: context_t;
    }

    new(ctx: context_t) {
        self.ctx = ctx;
    }

    fn socket(socket_type: socket_type) -> result<Cell<socket>, error> unsafe {
        let sock = zmq::zmq_socket(self.ctx, socket_type as c_int);

        if unsafe::reinterpret_cast(sock) == 0 {
            return err(errno_to_error());
        }

        ok(Cell(socket(sock as socket_t)))
    }

    fn term() -> result<(), error> {
        let rc = zmq::zmq_term(self.ctx);
        if rc == -1i32 {
            err(errno_to_error())
        } else {
            ok(())
        }
    }
}

struct socket {
    priv {
        let sock: socket_t;
        let mut closed: bool;
    }

    new(sock: socket_t) {
        self.sock = sock;
        self.closed = false;
    }

    drop {
        match self.close() {
          ok(()) => {}
          err(e) => fail e.to_str()
        }
    }

    fn get_socket_type() -> result<socket_type, error> {
        do getsockopt_int(self.sock, constants::ZMQ_TYPE).chain |ty| {
            if ty < PAIR as int || ty > XSUB as int {
                fail ~"socket type is out of range!";
            }
            unsafe { ok(unsafe::reinterpret_cast(ty)) }
        }
    }

    fn get_rcvmore() -> result<bool, error> {
        do getsockopt_i64(self.sock, constants::ZMQ_RCVMORE).chain |o| {
           ok(o == 1i64)
        }
    }

    fn get_hwm() -> result<u64, error> {
        getsockopt_u64(self.sock, constants::ZMQ_HWM)
    }

    fn get_affinity() -> result<u64, error> {
        getsockopt_u64(self.sock, constants::ZMQ_AFFINITY)
    }

    fn get_identity() -> result<~[u8], error> {
        getsockopt_bytes(self.sock, constants::ZMQ_IDENTITY)
    }

    fn get_rate() -> result<i64, error> {
        getsockopt_i64(self.sock, constants::ZMQ_RATE)
    }

    fn get_recovery_ivl() -> result<i64, error> {
        getsockopt_i64(self.sock, constants::ZMQ_RECOVERY_IVL)
    }

    fn get_recovery_ivl_msec() -> result<i64, error> {
        getsockopt_i64(self.sock, constants::ZMQ_RECOVERY_IVL_MSEC)
    }

    fn get_mcast_loop() -> result<bool, error> {
        do getsockopt_i64(self.sock, constants::ZMQ_MCAST_LOOP).chain |o| {
            ok(o == 1i64)
        }
    }

    fn get_sndbuf() -> result<u64, error> {
        getsockopt_u64(self.sock, constants::ZMQ_SNDBUF)
    }

    fn get_rcvbuf() -> result<u64, error> {
        getsockopt_u64(self.sock, constants::ZMQ_RCVBUF)
    }

    fn get_linger() -> result<i64, error> {
        getsockopt_i64(self.sock, constants::ZMQ_LINGER)
    }

    fn get_reconnect_ivl() -> result<int, error> {
        getsockopt_int(self.sock, constants::ZMQ_RECONNECT_IVL)
    }

    fn get_reconnect_ivl_max() -> result<int, error> {
        getsockopt_int(self.sock, constants::ZMQ_RECONNECT_IVL_MAX)
    }

    fn get_backlog() -> result<int, error> {
        getsockopt_int(self.sock, constants::ZMQ_BACKLOG)
    }

    fn get_fd() -> result<i64, error> {
        getsockopt_i64(self.sock, constants::ZMQ_FD)
    }

    fn get_events() -> result<u32, error> {
        getsockopt_u32(self.sock, constants::ZMQ_EVENTS)
    }

    fn set_hwm(value: u64) -> result<(), error> {
        setsockopt_u64(self.sock, constants::ZMQ_HWM, value)
    }

    fn set_affinity(value: u64) -> result<(), error> {
        setsockopt_u64(self.sock, constants::ZMQ_AFFINITY, value)
    }

    fn set_identity(value: &str) -> result<(), error> {
        setsockopt_str(self.sock, constants::ZMQ_IDENTITY, value)
    }

    fn set_subscribe(value: &[u8]) -> result<(), error> {
        setsockopt_bytes(self.sock, constants::ZMQ_SUBSCRIBE, value)
    }

    fn set_unsubscribe(value: &[u8]) -> result<(), error> {
        setsockopt_bytes(self.sock, constants::ZMQ_UNSUBSCRIBE, value)
    }

    fn set_rate(value: i64) -> result<(), error> {
        setsockopt_i64(self.sock, constants::ZMQ_RATE, value)
    }

    fn set_recovery_ivl(value: i64) -> result<(), error> {
        setsockopt_i64(self.sock, constants::ZMQ_RECOVERY_IVL, value)
    }

    fn set_recovery_ivl_msec(value: i64) -> result<(), error> {
        setsockopt_i64(self.sock, constants::ZMQ_RECOVERY_IVL_MSEC, value)
    }

    fn set_mcast_loop(value: bool) -> result<(), error> {
        let value = if value { 1i64 } else { 0i64 };
        setsockopt_i64(self.sock, constants::ZMQ_MCAST_LOOP, value)
    }

    fn set_sndbuf(value: u64) -> result<(), error> {
        setsockopt_u64(self.sock, constants::ZMQ_SNDBUF, value)
    }

    fn set_rcvbuf(value: u64) -> result<(), error> {
        setsockopt_u64(self.sock, constants::ZMQ_RCVBUF, value)
    }

    fn set_linger(value: int) -> result<(), error> {
        setsockopt_int(self.sock, constants::ZMQ_LINGER, value)
    }

    fn set_reconnect_ivl(value: int) -> result<(), error> {
        setsockopt_int(self.sock, constants::ZMQ_RECONNECT_IVL, value)
    }

    fn set_reconnect_ivl_max(value: int) -> result<(), error> {
        setsockopt_int(self.sock, constants::ZMQ_RECONNECT_IVL_MAX, value)
    }

    fn set_backlog(value: int) -> result<(), error> {
        setsockopt_int(self.sock, constants::ZMQ_BACKLOG, value)
    }

    /// Accept connections on a socket.
    fn bind(endpoint: &str) -> result<(), error> unsafe {
        let rc = do str::as_c_str(endpoint) |cstr| {
            zmq::zmq_bind(self.sock, cstr)
        };

        if rc == -1i32 { err(errno_to_error()) } else { ok(()) }
    }

    /// Connect a socket.
    fn connect(endpoint: &str) -> result<(), error> unsafe {
        let rc = do str::as_c_str(endpoint) |cstr| {
            zmq::zmq_connect(self.sock, cstr)
        };

        if rc == -1i32 { err(errno_to_error()) } else { ok(()) }
    }

    fn send(data: &[const u8], flags: int) -> result<(), error> {
        do vec::as_const_buf(data) |base_ptr, len| {
            let msg = {
                content: ptr::null(),
                flags: 0u8,
                vsm_size: 0u8,
                vsm_data0: 0u32,
                vsm_data1: 0u32,
                vsm_data2: 0u32,
                vsm_data3: 0u32,
                vsm_data4: 0u32,
                vsm_data5: 0u32,
                vsm_data6: 0u32,
            };

            // Copy the data into the message.
            zmq::zmq_msg_init_size(msg, len as size_t);

            unsafe {
                ptr::memcpy(
                    zmq::zmq_msg_data(msg),
                    unsafe::reinterpret_cast(base_ptr),
                    len);
            }

            let rc = zmq::zmq_send(self.sock, msg, flags as c_int);

            zmq::zmq_msg_close(msg);

            if rc == -1i32 { err(errno_to_error()) } else { ok(()) }
        }
    }

    fn send_str(data: &str, flags: int) -> result<(), error> {
        str::byte_slice(data, |bytes| self.send(bytes, flags))
    }

    fn recv(flags: int) -> result<Cell<~[u8]>, error> unsafe {
        let msg = {
            content: ptr::null(),
            flags: 0u8,
            vsm_size: 0u8,
            vsm_data0: 0u32,
            vsm_data1: 0u32,
            vsm_data2: 0u32,
            vsm_data3: 0u32,
            vsm_data4: 0u32,
            vsm_data5: 0u32,
            vsm_data6: 0u32,
        };

        zmq::zmq_msg_init(msg);

        let rc = zmq::zmq_recv(self.sock, msg, flags as c_int);
        let msg_data = zmq::zmq_msg_data(msg);
        let msg_size = zmq::zmq_msg_size(msg) as uint;

        // Extract the data out from the message.
        let mut data = ~[];
        vec::reserve(data, msg_size);

        unsafe {
            do vec::as_buf(data) |ptr, _len| {
                ptr::memcpy(ptr, msg_data, msg_size);
            }
            vec::unsafe::set_len(data, msg_size);
        }

        zmq::zmq_msg_close(msg);

        if rc == -1i32 { err(errno_to_error()) } else { ok(Cell(data)) }
    }

    fn recv_str(flags: int) -> result<Cell<~str>, error> unsafe {
        // FIXME: https://github.com/mozilla/rust/issues/2329.
        match self.recv(flags) {
          ok(msg) => ok(Cell(str::from_bytes(msg.take()))),
          err(e) => err(copy e),
        }
    }

    fn close() -> result<(), error> {
        if !self.closed {
            self.closed = true;

            if zmq::zmq_close(self.sock) == -1i32 {
                return err(errno_to_error());
            }
        }

        ok(())
    }
}

const POLLIN : i16 = 1i16;
const POLLOUT : i16 = 2i16;
const POLLERR : i16 = 4i16;

type pollitem = {
    socket: socket_t,
    fd: c_int,
    mut events: i16,
    mut revents: i16,
};

fn poll(items: &[pollitem], timeout: i64) -> result<(), error> unsafe {
    do vec::as_buf(items) |p, len| {
        let rc = zmq::zmq_poll(
            p,
            len as c_int,
            timeout as c_long);
        if rc == -1i32 { err(errno_to_error()) } else { ok(()) }
    }
}

impl error: to_str::ToStr {
    /// Return the error string for an error.
    fn to_str() -> ~str unsafe {
        let s = zmq::zmq_strerror(self as c_int);
        return if unsafe::reinterpret_cast(s) == -1 {
            let s = unsafe::reinterpret_cast(s);
            str::unsafe::from_c_str(s)
        } else {
            ~""
        }
    }
}

/// Convert the errno into an error type.
fn errno_to_error() -> error {
    match zmq::zmq_errno() {
        e if e == ENOTSUP as c_int         => ENOTSUP,
        e if e == EPROTONOSUPPORT as c_int => EPROTONOSUPPORT,
        e if e == ENOBUFS as c_int         => ENOBUFS,
        e if e == ENETDOWN as c_int        => ENETDOWN,
        e if e == EADDRINUSE as c_int      => EADDRINUSE,
        e if e == EADDRNOTAVAIL as c_int   => EADDRNOTAVAIL,
        e if e == ECONNREFUSED as c_int    => ECONNREFUSED,
        e if e == EINPROGRESS as c_int     => EINPROGRESS,
        e if e == ENOTSOCK as c_int        => ENOTSOCK,
        e if e == EFSM as c_int            => EFSM,
        e if e == ENOCOMPATPROTO as c_int  => ENOCOMPATPROTO,
        e if e == ETERM as c_int           => ETERM,
        e if e == EMTHREAD as c_int        => EMTHREAD,
        e => {
            let s = zmq::zmq_strerror(e);
            unsafe {
                fail if unsafe::reinterpret_cast(s) == -1 {
                    #fmt("unknown error: %i", e as int)
                } else {
                    #fmt("unknown error: [%i] %s",
                        e as int,
                        str::unsafe::from_c_str(s))
                }
            }
        }
    }
}

fn getsockopt_int(sock: socket_t, opt: c_int) -> result<int, error> {
    let value = 0u32 as c_int;
    let size = sys::size_of::<c_int>() as size_t;

    let r = zmq::zmq_getsockopt(
        sock,
        opt as c_int,
        ptr::addr_of(value) as *c_void,
        ptr::addr_of(size));

    if r == -1i32 { err(errno_to_error()) } else { ok(value as int) }
}

fn getsockopt_u32(sock: socket_t, opt: c_int) -> result<u32, error> {
    let value = 0u32;
    let size = sys::size_of::<u32>() as size_t;

    let r = zmq::zmq_getsockopt(
        sock,
        opt,
        ptr::addr_of(value) as *c_void,
        ptr::addr_of(size));

    if r == -1i32 { err(errno_to_error()) } else { ok(value) }
}

fn getsockopt_i64(sock: socket_t, opt: c_int) -> result<i64, error> {
    let value = 0i64;
    let size = sys::size_of::<i64>() as size_t;

    let r = zmq::zmq_getsockopt(
        sock,
        opt as c_int,
        ptr::addr_of(value) as *c_void,
        ptr::addr_of(size));

    if r == -1i32 { err(errno_to_error()) } else { ok(value) }
}

fn getsockopt_u64(sock: socket_t, opt: c_int) -> result<u64, error> {
    let value = 0u64;
    let size = sys::size_of::<u64>() as size_t;

    let r = zmq::zmq_getsockopt(
        sock,
        opt,
        ptr::addr_of(value) as *c_void,
        ptr::addr_of(size));

    if r == -1i32 { err(errno_to_error()) } else { ok(value) }
}

fn getsockopt_bytes(sock: socket_t, opt: c_int) ->
  result<~[u8], error> unsafe {
    let mut value = ~[];

    // The only binary option in zeromq is ZMQ_IDENTITY, which can have
    // a max size of 255 bytes.
    let size = 255u as size_t;
    vec::reserve::<u8>(value, size as uint);

    let r = zmq::zmq_getsockopt(
        sock,
        opt as c_int,
        unsafe { vec::unsafe::to_ptr(value) as *c_void },
        ptr::addr_of(size));

    if r == -1i32 {
        err(errno_to_error())
    } else {
        vec::unsafe::set_len(value, size as uint);
        ok(value)
    }
}

fn setsockopt_int(sock: socket_t, opt: c_int, value: int) ->
  result<(), error> {
    let value = value as c_int;
    let r = zmq::zmq_setsockopt(
        sock,
        opt as c_int,
        ptr::addr_of(value) as *c_void,
        sys::size_of::<c_int>() as size_t);

    if r == -1i32 { err(errno_to_error()) } else { ok(()) }
}

fn setsockopt_i64(sock: socket_t, opt: c_int, value: i64) ->
  result<(), error> {
    let r = zmq::zmq_setsockopt(
        sock,
        opt as c_int,
        ptr::addr_of(value) as *c_void,
        sys::size_of::<i64>() as size_t);

    if r == -1i32 { err(errno_to_error()) } else { ok(()) }
}

fn setsockopt_u64(sock: socket_t, opt: c_int, value: u64) ->
  result<(), error> {
    let r = zmq::zmq_setsockopt(
        sock,
        opt as c_int,
        ptr::addr_of(value) as *c_void,
        sys::size_of::<u64>() as size_t);

    if r == -1i32 { err(errno_to_error()) } else { ok(()) }
}

fn setsockopt_buf(sock: socket_t, opt: c_int, p: *u8, len: uint) ->
  result<(), error> unsafe {
    let r = zmq::zmq_setsockopt(
        sock,
        opt as c_int,
        unsafe { p as *c_void },
        len as size_t);

    if r == -1i32 { err(errno_to_error()) } else { ok(()) }
}

fn setsockopt_bytes(sock: socket_t, opt: c_int, value: &[u8]) ->
  result<(), error> unsafe {
    vec::as_buf(value, |p, len| setsockopt_buf(sock, opt, p, len))
}

fn setsockopt_str(sock: socket_t, opt: c_int, value: &str) ->
  result<(), error> unsafe {
    str::as_buf(value, |p, len| setsockopt_buf(sock, opt, p, len))
}
