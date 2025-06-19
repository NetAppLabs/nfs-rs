#[allow(dead_code)]
pub mod wasi {
    #[allow(dead_code)]
    pub mod clocks {
        #[allow(dead_code, clippy::all)]
        pub mod monotonic_clock {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Pollable = super::super::super::wasi::io::poll::Pollable;
            /// An instant in time, in nanoseconds. An instant is relative to an
            /// unspecified initial value, and can only be compared to instances from
            /// the same monotonic-clock.
            pub type Instant = u64;
            /// A duration of time, in nanoseconds.
            pub type Duration = u64;
            #[allow(unused_unsafe, clippy::all)]
            /// Read the current value of the clock.
            ///
            /// The clock is monotonic, therefore calling this function repeatedly will
            /// produce a sequence of non-decreasing values.
            pub fn now() -> Instant {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:clocks/monotonic-clock@0.2.0")]
                    extern "C" {
                        #[link_name = "now"]
                        fn wit_import() -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    ret as u64
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Query the resolution of the clock. Returns the duration of time
            /// corresponding to a clock tick.
            pub fn resolution() -> Duration {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:clocks/monotonic-clock@0.2.0")]
                    extern "C" {
                        #[link_name = "resolution"]
                        fn wit_import() -> i64;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i64 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    ret as u64
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Create a `pollable` which will resolve once the specified instant
            /// occured.
            pub fn subscribe_instant(when: Instant) -> Pollable {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:clocks/monotonic-clock@0.2.0")]
                    extern "C" {
                        #[link_name = "subscribe-instant"]
                        fn wit_import(_: i64) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_i64(when));
                    super::super::super::wasi::io::poll::Pollable::from_handle(
                        ret as u32,
                    )
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Create a `pollable` which will resolve once the given duration has
            /// elapsed, starting at the time at which this function was called.
            /// occured.
            pub fn subscribe_duration(when: Duration) -> Pollable {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:clocks/monotonic-clock@0.2.0")]
                    extern "C" {
                        #[link_name = "subscribe-duration"]
                        fn wit_import(_: i64) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i64) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_i64(when));
                    super::super::super::wasi::io::poll::Pollable::from_handle(
                        ret as u32,
                    )
                }
            }
        }
    }
    #[allow(dead_code)]
    pub mod io {
        #[allow(dead_code, clippy::all)]
        pub mod poll {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            /// `pollable` represents a single I/O event which may be ready, or not.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Pollable {
                handle: _rt::Resource<Pollable>,
            }
            impl Pollable {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Pollable {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:io/poll@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]pollable"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl Pollable {
                #[allow(unused_unsafe, clippy::all)]
                /// Return the readiness of a pollable. This function never blocks.
                ///
                /// Returns `true` when the pollable is ready, and `false` otherwise.
                pub fn ready(&self) -> bool {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/poll@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]pollable.ready"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        _rt::bool_lift(ret as u8)
                    }
                }
            }
            impl Pollable {
                #[allow(unused_unsafe, clippy::all)]
                /// `block` returns immediately if the pollable is ready, and otherwise
                /// blocks until ready.
                ///
                /// This function is equivalent to calling `poll.poll` on a list
                /// containing only this pollable.
                pub fn block(&self) {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/poll@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]pollable.block"]
                            fn wit_import(_: i32);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32);
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Poll for completion on a set of pollables.
            ///
            /// This function takes a list of pollables, which identify I/O sources of
            /// interest, and waits until one or more of the events is ready for I/O.
            ///
            /// The result `list<u32>` contains one or more indices of handles in the
            /// argument list that is ready for I/O.
            ///
            /// If the list contains more elements than can be indexed with a `u32`
            /// value, this function traps.
            ///
            /// A timeout can be implemented by adding a pollable from the
            /// wasi-clocks API to the list.
            ///
            /// This function does not return a `result`; polling in itself does not
            /// do any I/O so it doesn't fail. If any of the I/O sources identified by
            /// the pollables has an error, it is indicated by marking the source as
            /// being reaedy for I/O.
            pub fn poll(in_: &[&Pollable]) -> _rt::Vec<u32> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let vec0 = in_;
                    let len0 = vec0.len();
                    let layout0 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec0.len() * 4,
                        4,
                    );
                    let result0 = if layout0.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout0).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout0);
                        }
                        ptr
                    } else {
                        { ::core::ptr::null_mut() }
                    };
                    for (i, e) in vec0.into_iter().enumerate() {
                        let base = result0.add(i * 4);
                        {
                            *base.add(0).cast::<i32>() = (e).handle() as i32;
                        }
                    }
                    let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:io/poll@0.2.0")]
                    extern "C" {
                        #[link_name = "poll"]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(result0, len0, ptr1);
                    let l2 = *ptr1.add(0).cast::<*mut u8>();
                    let l3 = *ptr1.add(4).cast::<usize>();
                    let len4 = l3;
                    if layout0.size() != 0 {
                        _rt::alloc::dealloc(result0.cast(), layout0);
                    }
                    _rt::Vec::from_raw_parts(l2.cast(), len4, len4)
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod error {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            /// A resource which represents some error information.
            ///
            /// The only method provided by this resource is `to-debug-string`,
            /// which provides some human-readable information about the error.
            ///
            /// In the `wasi:io` package, this resource is returned through the
            /// `wasi:io/streams/stream-error` type.
            ///
            /// To provide more specific error information, other interfaces may
            /// provide functions to further "downcast" this error into more specific
            /// error information. For example, `error`s returned in streams derived
            /// from filesystem types to be described using the filesystem's own
            /// error-code type, using the function
            /// `wasi:filesystem/types/filesystem-error-code`, which takes a parameter
            /// `borrow<error>` and returns
            /// `option<wasi:filesystem/types/error-code>`.
            ///
            /// The set of functions which can "downcast" an `error` into a more
            /// concrete type is open.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Error {
                handle: _rt::Resource<Error>,
            }
            impl Error {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Error {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:io/error@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]error"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl Error {
                #[allow(unused_unsafe, clippy::all)]
                /// Returns a string that is suitable to assist humans in debugging
                /// this error.
                ///
                /// WARNING: The returned string should not be consumed mechanically!
                /// It may change across platforms, hosts, or other implementation
                /// details. Parsing this string is a major platform-compatibility
                /// hazard.
                pub fn to_debug_string(&self) -> _rt::String {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/error@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]error.to-debug-string"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let len3 = l2;
                        let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                        _rt::string_lift(bytes3)
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod streams {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Error = super::super::super::wasi::io::error::Error;
            pub type Pollable = super::super::super::wasi::io::poll::Pollable;
            /// An error for input-stream and output-stream operations.
            pub enum StreamError {
                /// The last operation (a write or flush) failed before completion.
                ///
                /// More information is available in the `error` payload.
                LastOperationFailed(Error),
                /// The stream is closed: no more input will be accepted by the
                /// stream. A closed output-stream will return this error on all
                /// future operations.
                Closed,
            }
            impl ::core::fmt::Debug for StreamError {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        StreamError::LastOperationFailed(e) => {
                            f.debug_tuple("StreamError::LastOperationFailed")
                                .field(e)
                                .finish()
                        }
                        StreamError::Closed => {
                            f.debug_tuple("StreamError::Closed").finish()
                        }
                    }
                }
            }
            impl ::core::fmt::Display for StreamError {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
            impl std::error::Error for StreamError {}
            /// An input bytestream.
            ///
            /// `input-stream`s are *non-blocking* to the extent practical on underlying
            /// platforms. I/O operations always return promptly; if fewer bytes are
            /// promptly available than requested, they return the number of bytes promptly
            /// available, which could even be zero. To wait for data to be available,
            /// use the `subscribe` function to obtain a `pollable` which can be polled
            /// for using `wasi:io/poll`.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct InputStream {
                handle: _rt::Resource<InputStream>,
            }
            impl InputStream {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for InputStream {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]input-stream"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            /// An output bytestream.
            ///
            /// `output-stream`s are *non-blocking* to the extent practical on
            /// underlying platforms. Except where specified otherwise, I/O operations also
            /// always return promptly, after the number of bytes that can be written
            /// promptly, which could even be zero. To wait for the stream to be ready to
            /// accept data, the `subscribe` function to obtain a `pollable` which can be
            /// polled for using `wasi:io/poll`.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct OutputStream {
                handle: _rt::Resource<OutputStream>,
            }
            impl OutputStream {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for OutputStream {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]output-stream"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl InputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Perform a non-blocking read from the stream.
                ///
                /// When the source of a `read` is binary data, the bytes from the source
                /// are returned verbatim. When the source of a `read` is known to the
                /// implementation to be text, bytes containing the UTF-8 encoding of the
                /// text are returned.
                ///
                /// This function returns a list of bytes containing the read data,
                /// when successful. The returned list will contain up to `len` bytes;
                /// it may return fewer than requested, but not more. The list is
                /// empty when no bytes are available for reading at this time. The
                /// pollable given by `subscribe` will be ready when more bytes are
                /// available.
                ///
                /// This function fails with a `stream-error` when the operation
                /// encounters an error, giving `last-operation-failed`, or when the
                /// stream is closed, giving `closed`.
                ///
                /// When the caller gives a `len` of 0, it represents a request to
                /// read 0 bytes. If the stream is still open, this call should
                /// succeed and return an empty list, or otherwise fail with `closed`.
                ///
                /// The `len` parameter is a `u64`, which could represent a list of u8 which
                /// is not possible to allocate in wasm32, or not desirable to allocate as
                /// as a return value by the callee. The callee may return a list of bytes
                /// less than `len` in size while more bytes are available for reading.
                pub fn read(&self, len: u64) -> Result<_rt::Vec<u8>, StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]input-stream.read"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&len), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;
                                    _rt::Vec::from_raw_parts(l2.cast(), len4, len4)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l5 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v7 = match l5 {
                                        0 => {
                                            let e7 = {
                                                let l6 = *ptr0.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l6 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e7)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v7
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl InputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Read bytes from a stream, after blocking until at least one byte can
                /// be read. Except for blocking, behavior is identical to `read`.
                pub fn blocking_read(
                    &self,
                    len: u64,
                ) -> Result<_rt::Vec<u8>, StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]input-stream.blocking-read"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&len), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;
                                    _rt::Vec::from_raw_parts(l2.cast(), len4, len4)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l5 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v7 = match l5 {
                                        0 => {
                                            let e7 = {
                                                let l6 = *ptr0.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l6 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e7)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v7
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl InputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Skip bytes from a stream. Returns number of bytes skipped.
                ///
                /// Behaves identical to `read`, except instead of returning a list
                /// of bytes, returns the number of bytes consumed from the stream.
                pub fn skip(&self, len: u64) -> Result<u64, StreamError> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]input-stream.skip"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&len), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr0.add(12).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl InputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Skip bytes from a stream, after blocking until at least one byte
                /// can be skipped. Except for blocking behavior, identical to `skip`.
                pub fn blocking_skip(&self, len: u64) -> Result<u64, StreamError> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]input-stream.blocking-skip"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&len), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr0.add(12).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl InputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Create a `pollable` which will resolve once either the specified stream
                /// has bytes available to read or the other end of the stream has been
                /// closed.
                /// The created `pollable` is a child resource of the `input-stream`.
                /// Implementations may trap if the `input-stream` is dropped before
                /// all derived `pollable`s created with this function are dropped.
                pub fn subscribe(&self) -> Pollable {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]input-stream.subscribe"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(
                            ret as u32,
                        )
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Check readiness for writing. This function never blocks.
                ///
                /// Returns the number of bytes permitted for the next call to `write`,
                /// or an error. Calling `write` with more bytes than this function has
                /// permitted will trap.
                ///
                /// When this function returns 0 bytes, the `subscribe` pollable will
                /// become ready when this function will report at least 1 byte, or an
                /// error.
                pub fn check_write(&self) -> Result<u64, StreamError> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.check-write"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr0.add(12).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Perform a write. This function never blocks.
                ///
                /// When the destination of a `write` is binary data, the bytes from
                /// `contents` are written verbatim. When the destination of a `write` is
                /// known to the implementation to be text, the bytes of `contents` are
                /// transcoded from UTF-8 into the encoding of the destination and then
                /// written.
                ///
                /// Precondition: check-write gave permit of Ok(n) and contents has a
                /// length of less than or equal to n. Otherwise, this function will trap.
                ///
                /// returns Err(closed) without writing if the stream has closed since
                /// the last call to check-write provided a permit.
                pub fn write(&self, contents: &[u8]) -> Result<(), StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let vec0 = contents;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.write"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr1.add(4).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr1.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Perform a write of up to 4096 bytes, and then flush the stream. Block
                /// until all of these operations are complete, or an error occurs.
                ///
                /// This is a convenience wrapper around the use of `check-write`,
                /// `subscribe`, `write`, and `flush`, and is implemented with the
                /// following pseudo-code:
                ///
                /// ```text
                /// let pollable = this.subscribe();
                /// while !contents.is_empty() {
                /// // Wait for the stream to become writable
                /// pollable.block();
                /// let Ok(n) = this.check-write(); // eliding error handling
                /// let len = min(n, contents.len());
                /// let (chunk, rest) = contents.split_at(len);
                /// this.write(chunk  );            // eliding error handling
                /// contents = rest;
                /// }
                /// this.flush();
                /// // Wait for completion of `flush`
                /// pollable.block();
                /// // Check for any errors that arose during `flush`
                /// let _ = this.check-write();         // eliding error handling
                /// ```
                pub fn blocking_write_and_flush(
                    &self,
                    contents: &[u8],
                ) -> Result<(), StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let vec0 = contents;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();
                        let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.blocking-write-and-flush"]
                            fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                        let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                        match l2 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr1.add(4).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr1.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Request to flush buffered output. This function never blocks.
                ///
                /// This tells the output-stream that the caller intends any buffered
                /// output to be flushed. the output which is expected to be flushed
                /// is all that has been passed to `write` prior to this call.
                ///
                /// Upon calling this function, the `output-stream` will not accept any
                /// writes (`check-write` will return `ok(0)`) until the flush has
                /// completed. The `subscribe` pollable will become ready when the
                /// flush has completed and the stream can accept more writes.
                pub fn flush(&self) -> Result<(), StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.flush"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v4 = match l2 {
                                        0 => {
                                            let e4 = {
                                                let l3 = *ptr0.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l3 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e4)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Request to flush buffered output, and block until flush completes
                /// and stream is ready for writing again.
                pub fn blocking_flush(&self) -> Result<(), StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.blocking-flush"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v4 = match l2 {
                                        0 => {
                                            let e4 = {
                                                let l3 = *ptr0.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l3 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e4)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Create a `pollable` which will resolve once the output-stream
                /// is ready for more writing, or an error has occured. When this
                /// pollable is ready, `check-write` will return `ok(n)` with n>0, or an
                /// error.
                ///
                /// If the stream is closed, this pollable is always ready immediately.
                ///
                /// The created `pollable` is a child resource of the `output-stream`.
                /// Implementations may trap if the `output-stream` is dropped before
                /// all derived `pollable`s created with this function are dropped.
                pub fn subscribe(&self) -> Pollable {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.subscribe"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(
                            ret as u32,
                        )
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Write zeroes to a stream.
                ///
                /// This should be used precisely like `write` with the exact same
                /// preconditions (must use check-write first), but instead of
                /// passing a list of bytes, you simply pass the number of zero-bytes
                /// that should be written.
                pub fn write_zeroes(&self, len: u64) -> Result<(), StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.write-zeroes"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&len), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v4 = match l2 {
                                        0 => {
                                            let e4 = {
                                                let l3 = *ptr0.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l3 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e4)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Perform a write of up to 4096 zeroes, and then flush the stream.
                /// Block until all of these operations are complete, or an error
                /// occurs.
                ///
                /// This is a convenience wrapper around the use of `check-write`,
                /// `subscribe`, `write-zeroes`, and `flush`, and is implemented with
                /// the following pseudo-code:
                ///
                /// ```text
                /// let pollable = this.subscribe();
                /// while num_zeroes != 0 {
                /// // Wait for the stream to become writable
                /// pollable.block();
                /// let Ok(n) = this.check-write(); // eliding error handling
                /// let len = min(n, num_zeroes);
                /// this.write-zeroes(len);         // eliding error handling
                /// num_zeroes -= len;
                /// }
                /// this.flush();
                /// // Wait for completion of `flush`
                /// pollable.block();
                /// // Check for any errors that arose during `flush`
                /// let _ = this.check-write();         // eliding error handling
                /// ```
                pub fn blocking_write_zeroes_and_flush(
                    &self,
                    len: u64,
                ) -> Result<(), StreamError> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.blocking-write-zeroes-and-flush"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&len), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    let v4 = match l2 {
                                        0 => {
                                            let e4 = {
                                                let l3 = *ptr0.add(8).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l3 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e4)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v4
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Read from one stream and write to another.
                ///
                /// The behavior of splice is equivelant to:
                /// 1. calling `check-write` on the `output-stream`
                /// 2. calling `read` on the `input-stream` with the smaller of the
                /// `check-write` permitted length and the `len` provided to `splice`
                /// 3. calling `write` on the `output-stream` with that read data.
                ///
                /// Any error reported by the call to `check-write`, `read`, or
                /// `write` ends the splice and reports that error.
                ///
                /// This function returns the number of bytes transferred; it may be less
                /// than `len`.
                pub fn splice(
                    &self,
                    src: &InputStream,
                    len: u64,
                ) -> Result<u64, StreamError> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.splice"]
                            fn wit_import(_: i32, _: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (src).handle() as i32,
                            _rt::as_i64(&len),
                            ptr0,
                        );
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr0.add(12).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl OutputStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Read from one stream and write to another, with blocking.
                ///
                /// This is similar to `splice`, except that it blocks until the
                /// `output-stream` is ready for writing, and the `input-stream`
                /// is ready for reading, before performing the `splice`.
                pub fn blocking_splice(
                    &self,
                    src: &InputStream,
                    len: u64,
                ) -> Result<u64, StreamError> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:io/streams@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]output-stream.blocking-splice"]
                            fn wit_import(_: i32, _: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (src).handle() as i32,
                            _rt::as_i64(&len),
                            ptr0,
                        );
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    let v5 = match l3 {
                                        0 => {
                                            let e5 = {
                                                let l4 = *ptr0.add(12).cast::<i32>();
                                                super::super::super::wasi::io::error::Error::from_handle(
                                                    l4 as u32,
                                                )
                                            };
                                            StreamError::LastOperationFailed(e5)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            StreamError::Closed
                                        }
                                    };
                                    v5
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
        }
    }
    #[allow(dead_code)]
    pub mod sockets {
        #[allow(dead_code, clippy::all)]
        pub mod network {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            /// An opaque resource that represents access to (a subset of) the network.
            /// This enables context-based security for networking.
            /// There is no need for this to map 1:1 to a physical network interface.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Network {
                handle: _rt::Resource<Network>,
            }
            impl Network {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for Network {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:sockets/network@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]network"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            /// Error codes.
            ///
            /// In theory, every API can return any error code.
            /// In practice, API's typically only return the errors documented per API
            /// combined with a couple of errors that are always possible:
            /// - `unknown`
            /// - `access-denied`
            /// - `not-supported`
            /// - `out-of-memory`
            /// - `concurrency-conflict`
            ///
            /// See each individual API for what the POSIX equivalents are. They sometimes differ per API.
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, PartialEq)]
            pub enum ErrorCode {
                /// Unknown error
                Unknown,
                /// Access denied.
                ///
                /// POSIX equivalent: EACCES, EPERM
                AccessDenied,
                /// The operation is not supported.
                ///
                /// POSIX equivalent: EOPNOTSUPP
                NotSupported,
                /// One of the arguments is invalid.
                ///
                /// POSIX equivalent: EINVAL
                InvalidArgument,
                /// Not enough memory to complete the operation.
                ///
                /// POSIX equivalent: ENOMEM, ENOBUFS, EAI_MEMORY
                OutOfMemory,
                /// The operation timed out before it could finish completely.
                Timeout,
                /// This operation is incompatible with another asynchronous operation that is already in progress.
                ///
                /// POSIX equivalent: EALREADY
                ConcurrencyConflict,
                /// Trying to finish an asynchronous operation that:
                /// - has not been started yet, or:
                /// - was already finished by a previous `finish-*` call.
                ///
                /// Note: this is scheduled to be removed when `future`s are natively supported.
                NotInProgress,
                /// The operation has been aborted because it could not be completed immediately.
                ///
                /// Note: this is scheduled to be removed when `future`s are natively supported.
                WouldBlock,
                /// The operation is not valid in the socket's current state.
                InvalidState,
                /// A new socket resource could not be created because of a system limit.
                NewSocketLimit,
                /// A bind operation failed because the provided address is not an address that the `network` can bind to.
                AddressNotBindable,
                /// A bind operation failed because the provided address is already in use or because there are no ephemeral ports available.
                AddressInUse,
                /// The remote address is not reachable
                RemoteUnreachable,
                /// The TCP connection was forcefully rejected
                ConnectionRefused,
                /// The TCP connection was reset.
                ConnectionReset,
                /// A TCP connection was aborted.
                ConnectionAborted,
                /// The size of a datagram sent to a UDP socket exceeded the maximum
                /// supported size.
                DatagramTooLarge,
                /// Name does not exist or has no suitable associated IP addresses.
                NameUnresolvable,
                /// A temporary failure in name resolution occurred.
                TemporaryResolverFailure,
                /// A permanent failure in name resolution occurred.
                PermanentResolverFailure,
            }
            impl ErrorCode {
                pub fn name(&self) -> &'static str {
                    match self {
                        ErrorCode::Unknown => "unknown",
                        ErrorCode::AccessDenied => "access-denied",
                        ErrorCode::NotSupported => "not-supported",
                        ErrorCode::InvalidArgument => "invalid-argument",
                        ErrorCode::OutOfMemory => "out-of-memory",
                        ErrorCode::Timeout => "timeout",
                        ErrorCode::ConcurrencyConflict => "concurrency-conflict",
                        ErrorCode::NotInProgress => "not-in-progress",
                        ErrorCode::WouldBlock => "would-block",
                        ErrorCode::InvalidState => "invalid-state",
                        ErrorCode::NewSocketLimit => "new-socket-limit",
                        ErrorCode::AddressNotBindable => "address-not-bindable",
                        ErrorCode::AddressInUse => "address-in-use",
                        ErrorCode::RemoteUnreachable => "remote-unreachable",
                        ErrorCode::ConnectionRefused => "connection-refused",
                        ErrorCode::ConnectionReset => "connection-reset",
                        ErrorCode::ConnectionAborted => "connection-aborted",
                        ErrorCode::DatagramTooLarge => "datagram-too-large",
                        ErrorCode::NameUnresolvable => "name-unresolvable",
                        ErrorCode::TemporaryResolverFailure => {
                            "temporary-resolver-failure"
                        }
                        ErrorCode::PermanentResolverFailure => {
                            "permanent-resolver-failure"
                        }
                    }
                }
                pub fn message(&self) -> &'static str {
                    match self {
                        ErrorCode::Unknown => "Unknown error",
                        ErrorCode::AccessDenied => {
                            "Access denied.

            POSIX equivalent: EACCES, EPERM"
                        }
                        ErrorCode::NotSupported => {
                            "The operation is not supported.

            POSIX equivalent: EOPNOTSUPP"
                        }
                        ErrorCode::InvalidArgument => {
                            "One of the arguments is invalid.

            POSIX equivalent: EINVAL"
                        }
                        ErrorCode::OutOfMemory => {
                            "Not enough memory to complete the operation.

            POSIX equivalent: ENOMEM, ENOBUFS, EAI_MEMORY"
                        }
                        ErrorCode::Timeout => {
                            "The operation timed out before it could finish completely."
                        }
                        ErrorCode::ConcurrencyConflict => {
                            "This operation is incompatible with another asynchronous operation that is already in progress.

            POSIX equivalent: EALREADY"
                        }
                        ErrorCode::NotInProgress => {
                            "Trying to finish an asynchronous operation that:
            - has not been started yet, or:
            - was already finished by a previous `finish-*` call.

            Note: this is scheduled to be removed when `future`s are natively supported."
                        }
                        ErrorCode::WouldBlock => {
                            "The operation has been aborted because it could not be completed immediately.

            Note: this is scheduled to be removed when `future`s are natively supported."
                        }
                        ErrorCode::InvalidState => {
                            "The operation is not valid in the socket's current state."
                        }
                        ErrorCode::NewSocketLimit => {
                            "A new socket resource could not be created because of a system limit."
                        }
                        ErrorCode::AddressNotBindable => {
                            "A bind operation failed because the provided address is not an address that the `network` can bind to."
                        }
                        ErrorCode::AddressInUse => {
                            "A bind operation failed because the provided address is already in use or because there are no ephemeral ports available."
                        }
                        ErrorCode::RemoteUnreachable => {
                            "The remote address is not reachable"
                        }
                        ErrorCode::ConnectionRefused => {
                            "The TCP connection was forcefully rejected"
                        }
                        ErrorCode::ConnectionReset => "The TCP connection was reset.",
                        ErrorCode::ConnectionAborted => "A TCP connection was aborted.",
                        ErrorCode::DatagramTooLarge => {
                            "The size of a datagram sent to a UDP socket exceeded the maximum
            supported size."
                        }
                        ErrorCode::NameUnresolvable => {
                            "Name does not exist or has no suitable associated IP addresses."
                        }
                        ErrorCode::TemporaryResolverFailure => {
                            "A temporary failure in name resolution occurred."
                        }
                        ErrorCode::PermanentResolverFailure => {
                            "A permanent failure in name resolution occurred."
                        }
                    }
                }
            }
            impl ::core::fmt::Debug for ErrorCode {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("ErrorCode")
                        .field("code", &(*self as i32))
                        .field("name", &self.name())
                        .field("message", &self.message())
                        .finish()
                }
            }
            impl ::core::fmt::Display for ErrorCode {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    write!(f, "{} (error {})", self.name(), * self as i32)
                }
            }
            impl std::error::Error for ErrorCode {}
            impl ErrorCode {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> ErrorCode {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => ErrorCode::Unknown,
                        1 => ErrorCode::AccessDenied,
                        2 => ErrorCode::NotSupported,
                        3 => ErrorCode::InvalidArgument,
                        4 => ErrorCode::OutOfMemory,
                        5 => ErrorCode::Timeout,
                        6 => ErrorCode::ConcurrencyConflict,
                        7 => ErrorCode::NotInProgress,
                        8 => ErrorCode::WouldBlock,
                        9 => ErrorCode::InvalidState,
                        10 => ErrorCode::NewSocketLimit,
                        11 => ErrorCode::AddressNotBindable,
                        12 => ErrorCode::AddressInUse,
                        13 => ErrorCode::RemoteUnreachable,
                        14 => ErrorCode::ConnectionRefused,
                        15 => ErrorCode::ConnectionReset,
                        16 => ErrorCode::ConnectionAborted,
                        17 => ErrorCode::DatagramTooLarge,
                        18 => ErrorCode::NameUnresolvable,
                        19 => ErrorCode::TemporaryResolverFailure,
                        20 => ErrorCode::PermanentResolverFailure,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, PartialEq)]
            pub enum IpAddressFamily {
                /// Similar to `AF_INET` in POSIX.
                Ipv4,
                /// Similar to `AF_INET6` in POSIX.
                Ipv6,
            }
            impl ::core::fmt::Debug for IpAddressFamily {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        IpAddressFamily::Ipv4 => {
                            f.debug_tuple("IpAddressFamily::Ipv4").finish()
                        }
                        IpAddressFamily::Ipv6 => {
                            f.debug_tuple("IpAddressFamily::Ipv6").finish()
                        }
                    }
                }
            }
            impl IpAddressFamily {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> IpAddressFamily {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => IpAddressFamily::Ipv4,
                        1 => IpAddressFamily::Ipv6,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
            pub type Ipv4Address = (u8, u8, u8, u8);
            pub type Ipv6Address = (u16, u16, u16, u16, u16, u16, u16, u16);
            #[derive(Clone, Copy)]
            pub enum IpAddress {
                Ipv4(Ipv4Address),
                Ipv6(Ipv6Address),
            }
            impl ::core::fmt::Debug for IpAddress {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        IpAddress::Ipv4(e) => {
                            f.debug_tuple("IpAddress::Ipv4").field(e).finish()
                        }
                        IpAddress::Ipv6(e) => {
                            f.debug_tuple("IpAddress::Ipv6").field(e).finish()
                        }
                    }
                }
            }
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct Ipv4SocketAddress {
                /// sin_port
                pub port: u16,
                /// sin_addr
                pub address: Ipv4Address,
            }
            impl ::core::fmt::Debug for Ipv4SocketAddress {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Ipv4SocketAddress")
                        .field("port", &self.port)
                        .field("address", &self.address)
                        .finish()
                }
            }
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct Ipv6SocketAddress {
                /// sin6_port
                pub port: u16,
                /// sin6_flowinfo
                pub flow_info: u32,
                /// sin6_addr
                pub address: Ipv6Address,
                /// sin6_scope_id
                pub scope_id: u32,
            }
            impl ::core::fmt::Debug for Ipv6SocketAddress {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    f.debug_struct("Ipv6SocketAddress")
                        .field("port", &self.port)
                        .field("flow-info", &self.flow_info)
                        .field("address", &self.address)
                        .field("scope-id", &self.scope_id)
                        .finish()
                }
            }
            #[derive(Clone, Copy)]
            pub enum IpSocketAddress {
                Ipv4(Ipv4SocketAddress),
                Ipv6(Ipv6SocketAddress),
            }
            impl ::core::fmt::Debug for IpSocketAddress {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        IpSocketAddress::Ipv4(e) => {
                            f.debug_tuple("IpSocketAddress::Ipv4").field(e).finish()
                        }
                        IpSocketAddress::Ipv6(e) => {
                            f.debug_tuple("IpSocketAddress::Ipv6").field(e).finish()
                        }
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod instance_network {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            pub type Network = super::super::super::wasi::sockets::network::Network;
            #[allow(unused_unsafe, clippy::all)]
            /// Get a handle to the default network.
            pub fn instance_network() -> Network {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:sockets/instance-network@0.2.0")]
                    extern "C" {
                        #[link_name = "instance-network"]
                        fn wit_import() -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    super::super::super::wasi::sockets::network::Network::from_handle(
                        ret as u32,
                    )
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod ip_name_lookup {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Pollable = super::super::super::wasi::io::poll::Pollable;
            pub type Network = super::super::super::wasi::sockets::network::Network;
            pub type ErrorCode = super::super::super::wasi::sockets::network::ErrorCode;
            pub type IpAddress = super::super::super::wasi::sockets::network::IpAddress;
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct ResolveAddressStream {
                handle: _rt::Resource<ResolveAddressStream>,
            }
            impl ResolveAddressStream {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for ResolveAddressStream {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:sockets/ip-name-lookup@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]resolve-address-stream"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Resolve an internet host name to a list of IP addresses.
            ///
            /// Unicode domain names are automatically converted to ASCII using IDNA encoding.
            /// If the input is an IP address string, the address is parsed and returned
            /// as-is without making any external requests.
            ///
            /// See the wasi-socket proposal README.md for a comparison with getaddrinfo.
            ///
            /// This function never blocks. It either immediately fails or immediately
            /// returns successfully with a `resolve-address-stream` that can be used
            /// to (asynchronously) fetch the results.
            ///
            /// # Typical errors
            /// - `invalid-argument`: `name` is a syntactically invalid domain name or IP address.
            ///
            /// # References:
            /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getaddrinfo.html>
            /// - <https://man7.org/linux/man-pages/man3/getaddrinfo.3.html>
            /// - <https://learn.microsoft.com/en-us/windows/win32/api/ws2tcpip/nf-ws2tcpip-getaddrinfo>
            /// - <https://man.freebsd.org/cgi/man.cgi?query=getaddrinfo&sektion=3>
            pub fn resolve_addresses(
                network: &Network,
                name: &str,
            ) -> Result<ResolveAddressStream, ErrorCode> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let vec0 = name;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:sockets/ip-name-lookup@0.2.0")]
                    extern "C" {
                        #[link_name = "resolve-addresses"]
                        fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: *mut u8, _: usize, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import((network).handle() as i32, ptr0.cast_mut(), len0, ptr1);
                    let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                    match l2 {
                        0 => {
                            let e = {
                                let l3 = *ptr1.add(4).cast::<i32>();
                                ResolveAddressStream::from_handle(l3 as u32)
                            };
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l4 = i32::from(*ptr1.add(4).cast::<u8>());
                                super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                    l4 as u8,
                                )
                            };
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
            impl ResolveAddressStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Returns the next address from the resolver.
                ///
                /// This function should be called multiple times. On each call, it will
                /// return the next address in connection order preference. If all
                /// addresses have been exhausted, this function returns `none`.
                ///
                /// This function never returns IPv4-mapped IPv6 addresses.
                ///
                /// # Typical errors
                /// - `name-unresolvable`:          Name does not exist or has no suitable associated IP addresses. (EAI_NONAME, EAI_NODATA, EAI_ADDRFAMILY)
                /// - `temporary-resolver-failure`: A temporary failure in name resolution occurred. (EAI_AGAIN)
                /// - `permanent-resolver-failure`: A permanent failure in name resolution occurred. (EAI_FAIL)
                /// - `would-block`:                A result is not available yet. (EWOULDBLOCK, EAGAIN)
                pub fn resolve_next_address(
                    &self,
                ) -> Result<Option<IpAddress>, ErrorCode> {
                    unsafe {
                        #[repr(align(2))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 22]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 22],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/ip-name-lookup@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]resolve-address-stream.resolve-next-address"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(2).cast::<u8>());
                                    match l2 {
                                        0 => None,
                                        1 => {
                                            let e = {
                                                let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                                use super::super::super::wasi::sockets::network::IpAddress as V16;
                                                let v16 = match l3 {
                                                    0 => {
                                                        let e16 = {
                                                            let l4 = i32::from(*ptr0.add(6).cast::<u8>());
                                                            let l5 = i32::from(*ptr0.add(7).cast::<u8>());
                                                            let l6 = i32::from(*ptr0.add(8).cast::<u8>());
                                                            let l7 = i32::from(*ptr0.add(9).cast::<u8>());
                                                            (l4 as u8, l5 as u8, l6 as u8, l7 as u8)
                                                        };
                                                        V16::Ipv4(e16)
                                                    }
                                                    n => {
                                                        debug_assert_eq!(n, 1, "invalid enum discriminant");
                                                        let e16 = {
                                                            let l8 = i32::from(*ptr0.add(6).cast::<u16>());
                                                            let l9 = i32::from(*ptr0.add(8).cast::<u16>());
                                                            let l10 = i32::from(*ptr0.add(10).cast::<u16>());
                                                            let l11 = i32::from(*ptr0.add(12).cast::<u16>());
                                                            let l12 = i32::from(*ptr0.add(14).cast::<u16>());
                                                            let l13 = i32::from(*ptr0.add(16).cast::<u16>());
                                                            let l14 = i32::from(*ptr0.add(18).cast::<u16>());
                                                            let l15 = i32::from(*ptr0.add(20).cast::<u16>());
                                                            (
                                                                l8 as u16,
                                                                l9 as u16,
                                                                l10 as u16,
                                                                l11 as u16,
                                                                l12 as u16,
                                                                l13 as u16,
                                                                l14 as u16,
                                                                l15 as u16,
                                                            )
                                                        };
                                                        V16::Ipv6(e16)
                                                    }
                                                };
                                                v16
                                            };
                                            Some(e)
                                        }
                                        _ => _rt::invalid_enum_discriminant(),
                                    }
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l17 = i32::from(*ptr0.add(2).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l17 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl ResolveAddressStream {
                #[allow(unused_unsafe, clippy::all)]
                /// Create a `pollable` which will resolve once the stream is ready for I/O.
                ///
                /// Note: this function is here for WASI Preview2 only.
                /// It's planned to be removed when `future` is natively supported in Preview3.
                pub fn subscribe(&self) -> Pollable {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/ip-name-lookup@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]resolve-address-stream.subscribe"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(
                            ret as u32,
                        )
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod tcp {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type InputStream = super::super::super::wasi::io::streams::InputStream;
            pub type OutputStream = super::super::super::wasi::io::streams::OutputStream;
            pub type Pollable = super::super::super::wasi::io::poll::Pollable;
            pub type Duration = super::super::super::wasi::clocks::monotonic_clock::Duration;
            pub type Network = super::super::super::wasi::sockets::network::Network;
            pub type ErrorCode = super::super::super::wasi::sockets::network::ErrorCode;
            pub type IpSocketAddress = super::super::super::wasi::sockets::network::IpSocketAddress;
            pub type IpAddressFamily = super::super::super::wasi::sockets::network::IpAddressFamily;
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, PartialEq)]
            pub enum ShutdownType {
                /// Similar to `SHUT_RD` in POSIX.
                Receive,
                /// Similar to `SHUT_WR` in POSIX.
                Send,
                /// Similar to `SHUT_RDWR` in POSIX.
                Both,
            }
            impl ::core::fmt::Debug for ShutdownType {
                fn fmt(
                    &self,
                    f: &mut ::core::fmt::Formatter<'_>,
                ) -> ::core::fmt::Result {
                    match self {
                        ShutdownType::Receive => {
                            f.debug_tuple("ShutdownType::Receive").finish()
                        }
                        ShutdownType::Send => {
                            f.debug_tuple("ShutdownType::Send").finish()
                        }
                        ShutdownType::Both => {
                            f.debug_tuple("ShutdownType::Both").finish()
                        }
                    }
                }
            }
            impl ShutdownType {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> ShutdownType {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => ShutdownType::Receive,
                        1 => ShutdownType::Send,
                        2 => ShutdownType::Both,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
            /// A TCP socket resource.
            ///
            /// The socket can be in one of the following states:
            /// - `unbound`
            /// - `bind-in-progress`
            /// - `bound` (See note below)
            /// - `listen-in-progress`
            /// - `listening`
            /// - `connect-in-progress`
            /// - `connected`
            /// - `closed`
            /// See <https://github.com/WebAssembly/wasi-sockets/TcpSocketOperationalSemantics.md>
            /// for a more information.
            ///
            /// Note: Except where explicitly mentioned, whenever this documentation uses
            /// the term "bound" without backticks it actually means: in the `bound` state *or higher*.
            /// (i.e. `bound`, `listen-in-progress`, `listening`, `connect-in-progress` or `connected`)
            ///
            /// In addition to the general error codes documented on the
            /// `network::error-code` type, TCP socket methods may always return
            /// `error(invalid-state)` when in the `closed` state.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct TcpSocket {
                handle: _rt::Resource<TcpSocket>,
            }
            impl TcpSocket {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for TcpSocket {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]tcp-socket"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Bind the socket to a specific network on the provided IP address and port.
                ///
                /// If the IP address is zero (`0.0.0.0` in IPv4, `::` in IPv6), it is left to the implementation to decide which
                /// network interface(s) to bind to.
                /// If the TCP/UDP port is zero, the socket will be bound to a random free port.
                ///
                /// Bind can be attempted multiple times on the same socket, even with
                /// different arguments on each iteration. But never concurrently and
                /// only as long as the previous bind failed. Once a bind succeeds, the
                /// binding can't be changed anymore.
                ///
                /// # Typical errors
                /// - `invalid-argument`:          The `local-address` has the wrong address family. (EAFNOSUPPORT, EFAULT on Windows)
                /// - `invalid-argument`:          `local-address` is not a unicast address. (EINVAL)
                /// - `invalid-argument`:          `local-address` is an IPv4-mapped IPv6 address. (EINVAL)
                /// - `invalid-state`:             The socket is already bound. (EINVAL)
                /// - `address-in-use`:            No ephemeral ports available. (EADDRINUSE, ENOBUFS on Windows)
                /// - `address-in-use`:            Address is already in use. (EADDRINUSE)
                /// - `address-not-bindable`:      `local-address` is not an address that the `network` can bind to. (EADDRNOTAVAIL)
                /// - `not-in-progress`:           A `bind` operation is not in progress.
                /// - `would-block`:               Can't finish the operation, it is still in progress. (EWOULDBLOCK, EAGAIN)
                ///
                /// # Implementors note
                /// When binding to a non-zero port, this bind operation shouldn't be affected by the TIME_WAIT
                /// state of a recently closed socket on the same local address. In practice this means that the SO_REUSEADDR
                /// socket option should be set implicitly on all platforms, except on Windows where this is the default behavior
                /// and SO_REUSEADDR performs something different entirely.
                ///
                /// Unlike in POSIX, in WASI the bind operation is async. This enables
                /// interactive WASI hosts to inject permission prompts. Runtimes that
                /// don't want to make use of this ability can simply call the native
                /// `bind` as part of either `start-bind` or `finish-bind`.
                ///
                /// # References
                /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/bind.html>
                /// - <https://man7.org/linux/man-pages/man2/bind.2.html>
                /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-bind>
                /// - <https://man.freebsd.org/cgi/man.cgi?query=bind&sektion=2&format=html>
                pub fn start_bind(
                    &self,
                    network: &Network,
                    local_address: IpSocketAddress,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        use super::super::super::wasi::sockets::network::IpSocketAddress as V4;
                        let (
                            result5_0,
                            result5_1,
                            result5_2,
                            result5_3,
                            result5_4,
                            result5_5,
                            result5_6,
                            result5_7,
                            result5_8,
                            result5_9,
                            result5_10,
                            result5_11,
                        ) = match local_address {
                            V4::Ipv4(e) => {
                                let super::super::super::wasi::sockets::network::Ipv4SocketAddress {
                                    port: port0,
                                    address: address0,
                                } = e;
                                let (t1_0, t1_1, t1_2, t1_3) = address0;
                                (
                                    0i32,
                                    _rt::as_i32(port0),
                                    _rt::as_i32(t1_0),
                                    _rt::as_i32(t1_1),
                                    _rt::as_i32(t1_2),
                                    _rt::as_i32(t1_3),
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                )
                            }
                            V4::Ipv6(e) => {
                                let super::super::super::wasi::sockets::network::Ipv6SocketAddress {
                                    port: port2,
                                    flow_info: flow_info2,
                                    address: address2,
                                    scope_id: scope_id2,
                                } = e;
                                let (t3_0, t3_1, t3_2, t3_3, t3_4, t3_5, t3_6, t3_7) = address2;
                                (
                                    1i32,
                                    _rt::as_i32(port2),
                                    _rt::as_i32(flow_info2),
                                    _rt::as_i32(t3_0),
                                    _rt::as_i32(t3_1),
                                    _rt::as_i32(t3_2),
                                    _rt::as_i32(t3_3),
                                    _rt::as_i32(t3_4),
                                    _rt::as_i32(t3_5),
                                    _rt::as_i32(t3_6),
                                    _rt::as_i32(t3_7),
                                    _rt::as_i32(scope_id2),
                                )
                            }
                        };
                        let ptr6 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.start-bind"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (network).handle() as i32,
                            result5_0,
                            result5_1,
                            result5_2,
                            result5_3,
                            result5_4,
                            result5_5,
                            result5_6,
                            result5_7,
                            result5_8,
                            result5_9,
                            result5_10,
                            result5_11,
                            ptr6,
                        );
                        let l7 = i32::from(*ptr6.add(0).cast::<u8>());
                        match l7 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l8 = i32::from(*ptr6.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l8 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn finish_bind(&self) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.finish-bind"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Connect to a remote endpoint.
                ///
                /// On success:
                /// - the socket is transitioned into the `connection` state.
                /// - a pair of streams is returned that can be used to read & write to the connection
                ///
                /// After a failed connection attempt, the socket will be in the `closed`
                /// state and the only valid action left is to `drop` the socket. A single
                /// socket can not be used to connect more than once.
                ///
                /// # Typical errors
                /// - `invalid-argument`:          The `remote-address` has the wrong address family. (EAFNOSUPPORT)
                /// - `invalid-argument`:          `remote-address` is not a unicast address. (EINVAL, ENETUNREACH on Linux, EAFNOSUPPORT on MacOS)
                /// - `invalid-argument`:          `remote-address` is an IPv4-mapped IPv6 address. (EINVAL, EADDRNOTAVAIL on Illumos)
                /// - `invalid-argument`:          The IP address in `remote-address` is set to INADDR_ANY (`0.0.0.0` / `::`). (EADDRNOTAVAIL on Windows)
                /// - `invalid-argument`:          The port in `remote-address` is set to 0. (EADDRNOTAVAIL on Windows)
                /// - `invalid-argument`:          The socket is already attached to a different network. The `network` passed to `connect` must be identical to the one passed to `bind`.
                /// - `invalid-state`:             The socket is already in the `connected` state. (EISCONN)
                /// - `invalid-state`:             The socket is already in the `listening` state. (EOPNOTSUPP, EINVAL on Windows)
                /// - `timeout`:                   Connection timed out. (ETIMEDOUT)
                /// - `connection-refused`:        The connection was forcefully rejected. (ECONNREFUSED)
                /// - `connection-reset`:          The connection was reset. (ECONNRESET)
                /// - `connection-aborted`:        The connection was aborted. (ECONNABORTED)
                /// - `remote-unreachable`:        The remote address is not reachable. (EHOSTUNREACH, EHOSTDOWN, ENETUNREACH, ENETDOWN, ENONET)
                /// - `address-in-use`:            Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE, EADDRNOTAVAIL on Linux, EAGAIN on BSD)
                /// - `not-in-progress`:           A connect operation is not in progress.
                /// - `would-block`:               Can't finish the operation, it is still in progress. (EWOULDBLOCK, EAGAIN)
                ///
                /// # Implementors note
                /// The POSIX equivalent of `start-connect` is the regular `connect` syscall.
                /// Because all WASI sockets are non-blocking this is expected to return
                /// EINPROGRESS, which should be translated to `ok()` in WASI.
                ///
                /// The POSIX equivalent of `finish-connect` is a `poll` for event `POLLOUT`
                /// with a timeout of 0 on the socket descriptor. Followed by a check for
                /// the `SO_ERROR` socket option, in case the poll signaled readiness.
                ///
                /// # References
                /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/connect.html>
                /// - <https://man7.org/linux/man-pages/man2/connect.2.html>
                /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-connect>
                /// - <https://man.freebsd.org/cgi/man.cgi?connect>
                pub fn start_connect(
                    &self,
                    network: &Network,
                    remote_address: IpSocketAddress,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        use super::super::super::wasi::sockets::network::IpSocketAddress as V4;
                        let (
                            result5_0,
                            result5_1,
                            result5_2,
                            result5_3,
                            result5_4,
                            result5_5,
                            result5_6,
                            result5_7,
                            result5_8,
                            result5_9,
                            result5_10,
                            result5_11,
                        ) = match remote_address {
                            V4::Ipv4(e) => {
                                let super::super::super::wasi::sockets::network::Ipv4SocketAddress {
                                    port: port0,
                                    address: address0,
                                } = e;
                                let (t1_0, t1_1, t1_2, t1_3) = address0;
                                (
                                    0i32,
                                    _rt::as_i32(port0),
                                    _rt::as_i32(t1_0),
                                    _rt::as_i32(t1_1),
                                    _rt::as_i32(t1_2),
                                    _rt::as_i32(t1_3),
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                    0i32,
                                )
                            }
                            V4::Ipv6(e) => {
                                let super::super::super::wasi::sockets::network::Ipv6SocketAddress {
                                    port: port2,
                                    flow_info: flow_info2,
                                    address: address2,
                                    scope_id: scope_id2,
                                } = e;
                                let (t3_0, t3_1, t3_2, t3_3, t3_4, t3_5, t3_6, t3_7) = address2;
                                (
                                    1i32,
                                    _rt::as_i32(port2),
                                    _rt::as_i32(flow_info2),
                                    _rt::as_i32(t3_0),
                                    _rt::as_i32(t3_1),
                                    _rt::as_i32(t3_2),
                                    _rt::as_i32(t3_3),
                                    _rt::as_i32(t3_4),
                                    _rt::as_i32(t3_5),
                                    _rt::as_i32(t3_6),
                                    _rt::as_i32(t3_7),
                                    _rt::as_i32(scope_id2),
                                )
                            }
                        };
                        let ptr6 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.start-connect"]
                            fn wit_import(
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: i32,
                                _: *mut u8,
                            );
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: i32,
                            _: *mut u8,
                        ) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            (network).handle() as i32,
                            result5_0,
                            result5_1,
                            result5_2,
                            result5_3,
                            result5_4,
                            result5_5,
                            result5_6,
                            result5_7,
                            result5_8,
                            result5_9,
                            result5_10,
                            result5_11,
                            ptr6,
                        );
                        let l7 = i32::from(*ptr6.add(0).cast::<u8>());
                        match l7 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l8 = i32::from(*ptr6.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l8 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn finish_connect(
                    &self,
                ) -> Result<(InputStream, OutputStream), ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 12],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.finish-connect"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    let l3 = *ptr0.add(8).cast::<i32>();
                                    (
                                        super::super::super::wasi::io::streams::InputStream::from_handle(
                                            l2 as u32,
                                        ),
                                        super::super::super::wasi::io::streams::OutputStream::from_handle(
                                            l3 as u32,
                                        ),
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l4 = i32::from(*ptr0.add(4).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l4 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Start listening for new connections.
                ///
                /// Transitions the socket into the `listening` state.
                ///
                /// Unlike POSIX, the socket must already be explicitly bound.
                ///
                /// # Typical errors
                /// - `invalid-state`:             The socket is not bound to any local address. (EDESTADDRREQ)
                /// - `invalid-state`:             The socket is already in the `connected` state. (EISCONN, EINVAL on BSD)
                /// - `invalid-state`:             The socket is already in the `listening` state.
                /// - `address-in-use`:            Tried to perform an implicit bind, but there were no ephemeral ports available. (EADDRINUSE)
                /// - `not-in-progress`:           A listen operation is not in progress.
                /// - `would-block`:               Can't finish the operation, it is still in progress. (EWOULDBLOCK, EAGAIN)
                ///
                /// # Implementors note
                /// Unlike in POSIX, in WASI the listen operation is async. This enables
                /// interactive WASI hosts to inject permission prompts. Runtimes that
                /// don't want to make use of this ability can simply call the native
                /// `listen` as part of either `start-listen` or `finish-listen`.
                ///
                /// # References
                /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/listen.html>
                /// - <https://man7.org/linux/man-pages/man2/listen.2.html>
                /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-listen>
                /// - <https://man.freebsd.org/cgi/man.cgi?query=listen&sektion=2>
                pub fn start_listen(&self) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.start-listen"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn finish_listen(&self) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.finish-listen"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Accept a new client socket.
                ///
                /// The returned socket is bound and in the `connected` state. The following properties are inherited from the listener socket:
                /// - `address-family`
                /// - `keep-alive-enabled`
                /// - `keep-alive-idle-time`
                /// - `keep-alive-interval`
                /// - `keep-alive-count`
                /// - `hop-limit`
                /// - `receive-buffer-size`
                /// - `send-buffer-size`
                ///
                /// On success, this function returns the newly accepted client socket along with
                /// a pair of streams that can be used to read & write to the connection.
                ///
                /// # Typical errors
                /// - `invalid-state`:      Socket is not in the `listening` state. (EINVAL)
                /// - `would-block`:        No pending connections at the moment. (EWOULDBLOCK, EAGAIN)
                /// - `connection-aborted`: An incoming connection was pending, but was terminated by the client before this listener could accept it. (ECONNABORTED)
                /// - `new-socket-limit`:   The new socket resource could not be created because of a system limit. (EMFILE, ENFILE)
                ///
                /// # References
                /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/accept.html>
                /// - <https://man7.org/linux/man-pages/man2/accept.2.html>
                /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-accept>
                /// - <https://man.freebsd.org/cgi/man.cgi?query=accept&sektion=2>
                pub fn accept(
                    &self,
                ) -> Result<(TcpSocket, InputStream, OutputStream), ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.accept"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    let l3 = *ptr0.add(8).cast::<i32>();
                                    let l4 = *ptr0.add(12).cast::<i32>();
                                    (
                                        TcpSocket::from_handle(l2 as u32),
                                        super::super::super::wasi::io::streams::InputStream::from_handle(
                                            l3 as u32,
                                        ),
                                        super::super::super::wasi::io::streams::OutputStream::from_handle(
                                            l4 as u32,
                                        ),
                                    )
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l5 = i32::from(*ptr0.add(4).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l5 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Get the bound local address.
                ///
                /// POSIX mentions:
                /// > If the socket has not been bound to a local name, the value
                /// > stored in the object pointed to by `address` is unspecified.
                ///
                /// WASI is stricter and requires `local-address` to return `invalid-state` when the socket hasn't been bound yet.
                ///
                /// # Typical errors
                /// - `invalid-state`: The socket is not bound to any local address.
                ///
                /// # References
                /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getsockname.html>
                /// - <https://man7.org/linux/man-pages/man2/getsockname.2.html>
                /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getsockname>
                /// - <https://man.freebsd.org/cgi/man.cgi?getsockname>
                pub fn local_address(&self) -> Result<IpSocketAddress, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 36]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 36],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.local-address"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    use super::super::super::wasi::sockets::network::IpSocketAddress as V19;
                                    let v19 = match l2 {
                                        0 => {
                                            let e19 = {
                                                let l3 = i32::from(*ptr0.add(8).cast::<u16>());
                                                let l4 = i32::from(*ptr0.add(10).cast::<u8>());
                                                let l5 = i32::from(*ptr0.add(11).cast::<u8>());
                                                let l6 = i32::from(*ptr0.add(12).cast::<u8>());
                                                let l7 = i32::from(*ptr0.add(13).cast::<u8>());
                                                super::super::super::wasi::sockets::network::Ipv4SocketAddress {
                                                    port: l3 as u16,
                                                    address: (l4 as u8, l5 as u8, l6 as u8, l7 as u8),
                                                }
                                            };
                                            V19::Ipv4(e19)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            let e19 = {
                                                let l8 = i32::from(*ptr0.add(8).cast::<u16>());
                                                let l9 = *ptr0.add(12).cast::<i32>();
                                                let l10 = i32::from(*ptr0.add(16).cast::<u16>());
                                                let l11 = i32::from(*ptr0.add(18).cast::<u16>());
                                                let l12 = i32::from(*ptr0.add(20).cast::<u16>());
                                                let l13 = i32::from(*ptr0.add(22).cast::<u16>());
                                                let l14 = i32::from(*ptr0.add(24).cast::<u16>());
                                                let l15 = i32::from(*ptr0.add(26).cast::<u16>());
                                                let l16 = i32::from(*ptr0.add(28).cast::<u16>());
                                                let l17 = i32::from(*ptr0.add(30).cast::<u16>());
                                                let l18 = *ptr0.add(32).cast::<i32>();
                                                super::super::super::wasi::sockets::network::Ipv6SocketAddress {
                                                    port: l8 as u16,
                                                    flow_info: l9 as u32,
                                                    address: (
                                                        l10 as u16,
                                                        l11 as u16,
                                                        l12 as u16,
                                                        l13 as u16,
                                                        l14 as u16,
                                                        l15 as u16,
                                                        l16 as u16,
                                                        l17 as u16,
                                                    ),
                                                    scope_id: l18 as u32,
                                                }
                                            };
                                            V19::Ipv6(e19)
                                        }
                                    };
                                    v19
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l20 = i32::from(*ptr0.add(4).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l20 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Get the remote address.
                ///
                /// # Typical errors
                /// - `invalid-state`: The socket is not connected to a remote address. (ENOTCONN)
                ///
                /// # References
                /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/getpeername.html>
                /// - <https://man7.org/linux/man-pages/man2/getpeername.2.html>
                /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-getpeername>
                /// - <https://man.freebsd.org/cgi/man.cgi?query=getpeername&sektion=2&n=1>
                pub fn remote_address(&self) -> Result<IpSocketAddress, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 36]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 36],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.remote-address"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(4).cast::<u8>());
                                    use super::super::super::wasi::sockets::network::IpSocketAddress as V19;
                                    let v19 = match l2 {
                                        0 => {
                                            let e19 = {
                                                let l3 = i32::from(*ptr0.add(8).cast::<u16>());
                                                let l4 = i32::from(*ptr0.add(10).cast::<u8>());
                                                let l5 = i32::from(*ptr0.add(11).cast::<u8>());
                                                let l6 = i32::from(*ptr0.add(12).cast::<u8>());
                                                let l7 = i32::from(*ptr0.add(13).cast::<u8>());
                                                super::super::super::wasi::sockets::network::Ipv4SocketAddress {
                                                    port: l3 as u16,
                                                    address: (l4 as u8, l5 as u8, l6 as u8, l7 as u8),
                                                }
                                            };
                                            V19::Ipv4(e19)
                                        }
                                        n => {
                                            debug_assert_eq!(n, 1, "invalid enum discriminant");
                                            let e19 = {
                                                let l8 = i32::from(*ptr0.add(8).cast::<u16>());
                                                let l9 = *ptr0.add(12).cast::<i32>();
                                                let l10 = i32::from(*ptr0.add(16).cast::<u16>());
                                                let l11 = i32::from(*ptr0.add(18).cast::<u16>());
                                                let l12 = i32::from(*ptr0.add(20).cast::<u16>());
                                                let l13 = i32::from(*ptr0.add(22).cast::<u16>());
                                                let l14 = i32::from(*ptr0.add(24).cast::<u16>());
                                                let l15 = i32::from(*ptr0.add(26).cast::<u16>());
                                                let l16 = i32::from(*ptr0.add(28).cast::<u16>());
                                                let l17 = i32::from(*ptr0.add(30).cast::<u16>());
                                                let l18 = *ptr0.add(32).cast::<i32>();
                                                super::super::super::wasi::sockets::network::Ipv6SocketAddress {
                                                    port: l8 as u16,
                                                    flow_info: l9 as u32,
                                                    address: (
                                                        l10 as u16,
                                                        l11 as u16,
                                                        l12 as u16,
                                                        l13 as u16,
                                                        l14 as u16,
                                                        l15 as u16,
                                                        l16 as u16,
                                                        l17 as u16,
                                                    ),
                                                    scope_id: l18 as u32,
                                                }
                                            };
                                            V19::Ipv6(e19)
                                        }
                                    };
                                    v19
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l20 = i32::from(*ptr0.add(4).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l20 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Whether the socket is in the `listening` state.
                ///
                /// Equivalent to the SO_ACCEPTCONN socket option.
                pub fn is_listening(&self) -> bool {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.is-listening"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        _rt::bool_lift(ret as u8)
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Whether this is a IPv4 or IPv6 socket.
                ///
                /// Equivalent to the SO_DOMAIN socket option.
                pub fn address_family(&self) -> IpAddressFamily {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.address-family"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::sockets::network::IpAddressFamily::_lift(
                            ret as u8,
                        )
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Hints the desired listen queue size. Implementations are free to ignore this.
                ///
                /// If the provided value is 0, an `invalid-argument` error is returned.
                /// Any other value will never cause an error, but it might be silently clamped and/or rounded.
                ///
                /// # Typical errors
                /// - `not-supported`:        (set) The platform does not support changing the backlog size after the initial listen.
                /// - `invalid-argument`:     (set) The provided value was 0.
                /// - `invalid-state`:        (set) The socket is in the `connect-in-progress` or `connected` state.
                pub fn set_listen_backlog_size(
                    &self,
                    value: u64,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.set-listen-backlog-size"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Enables or disables keepalive.
                ///
                /// The keepalive behavior can be adjusted using:
                /// - `keep-alive-idle-time`
                /// - `keep-alive-interval`
                /// - `keep-alive-count`
                /// These properties can be configured while `keep-alive-enabled` is false, but only come into effect when `keep-alive-enabled` is true.
                ///
                /// Equivalent to the SO_KEEPALIVE socket option.
                pub fn keep_alive_enabled(&self) -> Result<bool, ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.keep-alive-enabled"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    _rt::bool_lift(l2 as u8)
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_keep_alive_enabled(
                    &self,
                    value: bool,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.set-keep-alive-enabled"]
                            fn wit_import(_: i32, _: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            match &value {
                                true => 1,
                                false => 0,
                            },
                            ptr0,
                        );
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Amount of time the connection has to be idle before TCP starts sending keepalive packets.
                ///
                /// If the provided value is 0, an `invalid-argument` error is returned.
                /// Any other value will never cause an error, but it might be silently clamped and/or rounded.
                /// I.e. after setting a value, reading the same setting back may return a different value.
                ///
                /// Equivalent to the TCP_KEEPIDLE socket option. (TCP_KEEPALIVE on MacOS)
                ///
                /// # Typical errors
                /// - `invalid-argument`:     (set) The provided value was 0.
                pub fn keep_alive_idle_time(&self) -> Result<Duration, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.keep-alive-idle-time"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_keep_alive_idle_time(
                    &self,
                    value: Duration,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.set-keep-alive-idle-time"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// The time between keepalive packets.
                ///
                /// If the provided value is 0, an `invalid-argument` error is returned.
                /// Any other value will never cause an error, but it might be silently clamped and/or rounded.
                /// I.e. after setting a value, reading the same setting back may return a different value.
                ///
                /// Equivalent to the TCP_KEEPINTVL socket option.
                ///
                /// # Typical errors
                /// - `invalid-argument`:     (set) The provided value was 0.
                pub fn keep_alive_interval(&self) -> Result<Duration, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.keep-alive-interval"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_keep_alive_interval(
                    &self,
                    value: Duration,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.set-keep-alive-interval"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// The maximum amount of keepalive packets TCP should send before aborting the connection.
                ///
                /// If the provided value is 0, an `invalid-argument` error is returned.
                /// Any other value will never cause an error, but it might be silently clamped and/or rounded.
                /// I.e. after setting a value, reading the same setting back may return a different value.
                ///
                /// Equivalent to the TCP_KEEPCNT socket option.
                ///
                /// # Typical errors
                /// - `invalid-argument`:     (set) The provided value was 0.
                pub fn keep_alive_count(&self) -> Result<u32, ErrorCode> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 8],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.keep-alive-count"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<i32>();
                                    l2 as u32
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_keep_alive_count(&self, value: u32) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.set-keep-alive-count"]
                            fn wit_import(_: i32, _: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i32(&value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Equivalent to the IP_TTL & IPV6_UNICAST_HOPS socket options.
                ///
                /// If the provided value is 0, an `invalid-argument` error is returned.
                ///
                /// # Typical errors
                /// - `invalid-argument`:     (set) The TTL value must be 1 or higher.
                pub fn hop_limit(&self) -> Result<u8, ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.hop-limit"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    l2 as u8
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_hop_limit(&self, value: u8) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.set-hop-limit"]
                            fn wit_import(_: i32, _: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i32(&value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// The kernel buffer space reserved for sends/receives on this socket.
                ///
                /// If the provided value is 0, an `invalid-argument` error is returned.
                /// Any other value will never cause an error, but it might be silently clamped and/or rounded.
                /// I.e. after setting a value, reading the same setting back may return a different value.
                ///
                /// Equivalent to the SO_RCVBUF and SO_SNDBUF socket options.
                ///
                /// # Typical errors
                /// - `invalid-argument`:     (set) The provided value was 0.
                pub fn receive_buffer_size(&self) -> Result<u64, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.receive-buffer-size"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_receive_buffer_size(
                    &self,
                    value: u64,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.set-receive-buffer-size"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn send_buffer_size(&self) -> Result<u64, ErrorCode> {
                    unsafe {
                        #[repr(align(8))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 16]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 16],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.send-buffer-size"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = {
                                    let l2 = *ptr0.add(8).cast::<i64>();
                                    l2 as u64
                                };
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l3 = i32::from(*ptr0.add(8).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l3 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                pub fn set_send_buffer_size(&self, value: u64) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.set-send-buffer-size"]
                            fn wit_import(_: i32, _: i64, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i64, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, _rt::as_i64(&value), ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Create a `pollable` which can be used to poll for, or block on,
                /// completion of any of the asynchronous operations of this socket.
                ///
                /// When `finish-bind`, `finish-listen`, `finish-connect` or `accept`
                /// return `error(would-block)`, this pollable can be used to wait for
                /// their success or failure, after which the method can be retried.
                ///
                /// The pollable is not limited to the async operation that happens to be
                /// in progress at the time of calling `subscribe` (if any). Theoretically,
                /// `subscribe` only has to be called once per socket and can then be
                /// (re)used for the remainder of the socket's lifetime.
                ///
                /// See <https://github.com/WebAssembly/wasi-sockets/TcpSocketOperationalSemantics.md#Pollable-readiness>
                /// for a more information.
                ///
                /// Note: this function is here for WASI Preview2 only.
                /// It's planned to be removed when `future` is natively supported in Preview3.
                pub fn subscribe(&self) -> Pollable {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.subscribe"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        super::super::super::wasi::io::poll::Pollable::from_handle(
                            ret as u32,
                        )
                    }
                }
            }
            impl TcpSocket {
                #[allow(unused_unsafe, clippy::all)]
                /// Initiate a graceful shutdown.
                ///
                /// - `receive`: The socket is not expecting to receive any data from
                /// the peer. The `input-stream` associated with this socket will be
                /// closed. Any data still in the receive queue at time of calling
                /// this method will be discarded.
                /// - `send`: The socket has no more data to send to the peer. The `output-stream`
                /// associated with this socket will be closed and a FIN packet will be sent.
                /// - `both`: Same effect as `receive` & `send` combined.
                ///
                /// This function is idempotent. Shutting a down a direction more than once
                /// has no effect and returns `ok`.
                ///
                /// The shutdown function does not close (drop) the socket.
                ///
                /// # Typical errors
                /// - `invalid-state`: The socket is not in the `connected` state. (ENOTCONN)
                ///
                /// # References
                /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/shutdown.html>
                /// - <https://man7.org/linux/man-pages/man2/shutdown.2.html>
                /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-shutdown>
                /// - <https://man.freebsd.org/cgi/man.cgi?query=shutdown&sektion=2>
                pub fn shutdown(
                    &self,
                    shutdown_type: ShutdownType,
                ) -> Result<(), ErrorCode> {
                    unsafe {
                        #[repr(align(1))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 2]);
                        let mut ret_area = RetArea(
                            [::core::mem::MaybeUninit::uninit(); 2],
                        );
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "wasi:sockets/tcp@0.2.0")]
                        extern "C" {
                            #[link_name = "[method]tcp-socket.shutdown"]
                            fn wit_import(_: i32, _: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import(
                            (self).handle() as i32,
                            shutdown_type.clone() as i32,
                            ptr0,
                        );
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => {
                                let e = ();
                                Ok(e)
                            }
                            1 => {
                                let e = {
                                    let l2 = i32::from(*ptr0.add(1).cast::<u8>());
                                    super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                        l2 as u8,
                                    )
                                };
                                Err(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod tcp_create_socket {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type ErrorCode = super::super::super::wasi::sockets::network::ErrorCode;
            pub type IpAddressFamily = super::super::super::wasi::sockets::network::IpAddressFamily;
            pub type TcpSocket = super::super::super::wasi::sockets::tcp::TcpSocket;
            #[allow(unused_unsafe, clippy::all)]
            /// Create a new TCP socket.
            ///
            /// Similar to `socket(AF_INET or AF_INET6, SOCK_STREAM, IPPROTO_TCP)` in POSIX.
            /// On IPv6 sockets, IPV6_V6ONLY is enabled by default and can't be configured otherwise.
            ///
            /// This function does not require a network capability handle. This is considered to be safe because
            /// at time of creation, the socket is not bound to any `network` yet. Up to the moment `bind`/`connect`
            /// is called, the socket is effectively an in-memory configuration object, unable to communicate with the outside world.
            ///
            /// All sockets are non-blocking. Use the wasi-poll interface to block on asynchronous operations.
            ///
            /// # Typical errors
            /// - `not-supported`:     The specified `address-family` is not supported. (EAFNOSUPPORT)
            /// - `new-socket-limit`:  The new socket resource could not be created because of a system limit. (EMFILE, ENFILE)
            ///
            /// # References
            /// - <https://pubs.opengroup.org/onlinepubs/9699919799/functions/socket.html>
            /// - <https://man7.org/linux/man-pages/man2/socket.2.html>
            /// - <https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-wsasocketw>
            /// - <https://man.freebsd.org/cgi/man.cgi?query=socket&sektion=2>
            pub fn create_tcp_socket(
                address_family: IpAddressFamily,
            ) -> Result<TcpSocket, ErrorCode> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "wasi:sockets/tcp-create-socket@0.2.0")]
                    extern "C" {
                        #[link_name = "create-tcp-socket"]
                        fn wit_import(_: i32, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(address_family.clone() as i32, ptr0);
                    let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                    match l1 {
                        0 => {
                            let e = {
                                let l2 = *ptr0.add(4).cast::<i32>();
                                super::super::super::wasi::sockets::tcp::TcpSocket::from_handle(
                                    l2 as u32,
                                )
                            };
                            Ok(e)
                        }
                        1 => {
                            let e = {
                                let l3 = i32::from(*ptr0.add(4).cast::<u8>());
                                super::super::super::wasi::sockets::network::ErrorCode::_lift(
                                    l3 as u8,
                                )
                            };
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod component {
        #[allow(dead_code)]
        pub mod nfs_rs {
            #[allow(dead_code, clippy::all)]
            pub mod nfs {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                /// File handle
                pub type Fh = _rt::Vec<u8>;
                /// List/vector/array of bytes
                pub type Bytes = _rt::Vec<u8>;
                /// Time
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct Time {
                    pub seconds: u32,
                    pub nseconds: u32,
                }
                impl ::core::fmt::Debug for Time {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Time")
                            .field("seconds", &self.seconds)
                            .field("nseconds", &self.nseconds)
                            .finish()
                    }
                }
                /// Attributes
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct Attr {
                    pub attr_type: u32,
                    pub file_mode: u32,
                    pub nlink: u32,
                    pub uid: u32,
                    pub gid: u32,
                    pub filesize: u64,
                    pub used: u64,
                    pub spec_data: (u32, u32),
                    pub fsid: u64,
                    pub fileid: u64,
                    pub atime: Time,
                    pub mtime: Time,
                    pub ctime: Time,
                }
                impl ::core::fmt::Debug for Attr {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Attr")
                            .field("attr-type", &self.attr_type)
                            .field("file-mode", &self.file_mode)
                            .field("nlink", &self.nlink)
                            .field("uid", &self.uid)
                            .field("gid", &self.gid)
                            .field("filesize", &self.filesize)
                            .field("used", &self.used)
                            .field("spec-data", &self.spec_data)
                            .field("fsid", &self.fsid)
                            .field("fileid", &self.fileid)
                            .field("atime", &self.atime)
                            .field("mtime", &self.mtime)
                            .field("ctime", &self.ctime)
                            .finish()
                    }
                }
                /// Object response
                #[derive(Clone)]
                pub struct ObjRes {
                    pub obj: Fh,
                    pub attr: Option<Attr>,
                }
                impl ::core::fmt::Debug for ObjRes {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("ObjRes")
                            .field("obj", &self.obj)
                            .field("attr", &self.attr)
                            .finish()
                    }
                }
                /// Path configuration
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct PathConf {
                    pub attr: Option<Attr>,
                    pub linkmax: u32,
                    pub name_max: u32,
                    pub no_trunc: bool,
                    pub chown_restricted: bool,
                    pub case_insensitive: bool,
                    pub case_preserving: bool,
                }
                impl ::core::fmt::Debug for PathConf {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("PathConf")
                            .field("attr", &self.attr)
                            .field("linkmax", &self.linkmax)
                            .field("name-max", &self.name_max)
                            .field("no-trunc", &self.no_trunc)
                            .field("chown-restricted", &self.chown_restricted)
                            .field("case-insensitive", &self.case_insensitive)
                            .field("case-preserving", &self.case_preserving)
                            .finish()
                    }
                }
                /// Directory entry as returned from `nfs-mount::readdir`
                #[derive(Clone)]
                pub struct ReaddirEntry {
                    pub fileid: u64,
                    pub file_name: _rt::String,
                }
                impl ::core::fmt::Debug for ReaddirEntry {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("ReaddirEntry")
                            .field("fileid", &self.fileid)
                            .field("file-name", &self.file_name)
                            .finish()
                    }
                }
                /// Directory entry as returned from `nfs-mount::readdirplus`
                #[derive(Clone)]
                pub struct ReaddirplusEntry {
                    pub fileid: u64,
                    pub file_name: _rt::String,
                    pub attr: Option<Attr>,
                    pub handle: Fh,
                }
                impl ::core::fmt::Debug for ReaddirplusEntry {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("ReaddirplusEntry")
                            .field("fileid", &self.fileid)
                            .field("file-name", &self.file_name)
                            .field("attr", &self.attr)
                            .field("handle", &self.handle)
                            .finish()
                    }
                }
                /// NFS version
                #[repr(u8)]
                #[derive(Clone, Copy, Eq, PartialEq)]
                pub enum NfsVersion {
                    NfsV3,
                    NfsV4,
                    NfsV4p1,
                }
                impl ::core::fmt::Debug for NfsVersion {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            NfsVersion::NfsV3 => {
                                f.debug_tuple("NfsVersion::NfsV3").finish()
                            }
                            NfsVersion::NfsV4 => {
                                f.debug_tuple("NfsVersion::NfsV4").finish()
                            }
                            NfsVersion::NfsV4p1 => {
                                f.debug_tuple("NfsVersion::NfsV4p1").finish()
                            }
                        }
                    }
                }
                impl NfsVersion {
                    #[doc(hidden)]
                    pub unsafe fn _lift(val: u8) -> NfsVersion {
                        if !cfg!(debug_assertions) {
                            return ::core::mem::transmute(val);
                        }
                        match val {
                            0 => NfsVersion::NfsV3,
                            1 => NfsVersion::NfsV4,
                            2 => NfsVersion::NfsV4p1,
                            _ => panic!("invalid enum discriminant"),
                        }
                    }
                }
                /// Error
                #[derive(Clone)]
                pub struct Error {
                    pub nfs_error_code: Option<i32>,
                    pub message: _rt::String,
                }
                impl ::core::fmt::Debug for Error {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("Error")
                            .field("nfs-error-code", &self.nfs_error_code)
                            .field("message", &self.message)
                            .finish()
                    }
                }
                impl ::core::fmt::Display for Error {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        write!(f, "{:?}", self)
                    }
                }
                impl std::error::Error for Error {}
                /// Version agnostic NFS mount
                ///
                /// Due to the NFS mount being version agnostic, calling functions not supported by the NFS version being used will
                /// result in error being returned.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct NfsMount {
                    handle: _rt::Resource<NfsMount>,
                }
                type _NfsMountRep<T> = Option<T>;
                impl NfsMount {
                    /// Creates a new resource from the specified representation.
                    ///
                    /// This function will create a new resource handle by moving `val` onto
                    /// the heap and then passing that heap pointer to the component model to
                    /// create a handle. The owned handle is then returned as `NfsMount`.
                    pub fn new<T: GuestNfsMount>(val: T) -> Self {
                        Self::type_guard::<T>();
                        let val: _NfsMountRep<T> = Some(val);
                        let ptr: *mut _NfsMountRep<T> = _rt::Box::into_raw(
                            _rt::Box::new(val),
                        );
                        unsafe { Self::from_handle(T::_resource_new(ptr.cast())) }
                    }
                    /// Gets access to the underlying `T` which represents this resource.
                    pub fn get<T: GuestNfsMount>(&self) -> &T {
                        let ptr = unsafe { &*self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    /// Gets mutable access to the underlying `T` which represents this
                    /// resource.
                    pub fn get_mut<T: GuestNfsMount>(&mut self) -> &mut T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_mut().unwrap()
                    }
                    /// Consumes this resource and returns the underlying `T`.
                    pub fn into_inner<T: GuestNfsMount>(self) -> T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.take().unwrap()
                    }
                    #[doc(hidden)]
                    pub unsafe fn from_handle(handle: u32) -> Self {
                        Self {
                            handle: _rt::Resource::from_handle(handle),
                        }
                    }
                    #[doc(hidden)]
                    pub fn take_handle(&self) -> u32 {
                        _rt::Resource::take_handle(&self.handle)
                    }
                    #[doc(hidden)]
                    pub fn handle(&self) -> u32 {
                        _rt::Resource::handle(&self.handle)
                    }
                    #[doc(hidden)]
                    fn type_guard<T: 'static>() {
                        use core::any::TypeId;
                        static mut LAST_TYPE: Option<TypeId> = None;
                        unsafe {
                            assert!(! cfg!(target_feature = "atomics"));
                            let id = TypeId::of::<T>();
                            match LAST_TYPE {
                                Some(ty) => {
                                    assert!(
                                        ty == id, "cannot use two types with this resource type"
                                    )
                                }
                                None => LAST_TYPE = Some(id),
                            }
                        }
                    }
                    #[doc(hidden)]
                    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {
                        Self::type_guard::<T>();
                        let _ = _rt::Box::from_raw(handle as *mut _NfsMountRep<T>);
                    }
                    fn as_ptr<T: GuestNfsMount>(&self) -> *mut _NfsMountRep<T> {
                        NfsMount::type_guard::<T>();
                        T::_resource_rep(self.handle()).cast()
                    }
                }
                /// A borrowed version of [`NfsMount`] which represents a borrowed value
                /// with the lifetime `'a`.
                #[derive(Debug)]
                #[repr(transparent)]
                pub struct NfsMountBorrow<'a> {
                    rep: *mut u8,
                    _marker: core::marker::PhantomData<&'a NfsMount>,
                }
                impl<'a> NfsMountBorrow<'a> {
                    #[doc(hidden)]
                    pub unsafe fn lift(rep: usize) -> Self {
                        Self {
                            rep: rep as *mut u8,
                            _marker: core::marker::PhantomData,
                        }
                    }
                    /// Gets access to the underlying `T` in this resource.
                    pub fn get<T: GuestNfsMount>(&self) -> &T {
                        let ptr = unsafe { &mut *self.as_ptr::<T>() };
                        ptr.as_ref().unwrap()
                    }
                    fn as_ptr<T: 'static>(&self) -> *mut _NfsMountRep<T> {
                        NfsMount::type_guard::<T>();
                        self.rep.cast()
                    }
                }
                unsafe impl _rt::WasmResource for NfsMount {
                    #[inline]
                    unsafe fn drop(_handle: u32) {
                        #[cfg(not(target_arch = "wasm32"))]
                        unreachable!();
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]component:nfs-rs/nfs")]
                            extern "C" {
                                #[link_name = "[resource-drop]nfs-mount"]
                                fn drop(_: u32);
                            }
                            drop(_handle);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_parse_url_and_mount_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = T::parse_url_and_mount(_rt::string_lift(bytes0));
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            *ptr2.add(4).cast::<i32>() = (e).take_handle() as i32;
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code3,
                                message: message3,
                            } = e;
                            match nfs_error_code3 {
                                Some(e) => {
                                    *ptr2.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr2.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr2.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec4 = (message3.into_bytes()).into_boxed_slice();
                            let ptr4 = vec4.as_ptr().cast::<u8>();
                            let len4 = vec4.len();
                            ::core::mem::forget(vec4);
                            *ptr2.add(16).cast::<usize>() = len4;
                            *ptr2.add(12).cast::<*mut u8>() = ptr4.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_parse_url_and_mount<T: Guest>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(12).cast::<*mut u8>();
                            let l2 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_null_op_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::null_op(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Ok(_) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                        }
                        Err(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code2,
                                message: message2,
                            } = e;
                            match nfs_error_code2 {
                                Some(e) => {
                                    *ptr1.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr1.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr1.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec3 = (message2.into_bytes()).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *ptr1.add(16).cast::<usize>() = len3;
                            *ptr1.add(12).cast::<*mut u8>() = ptr3.cast_mut();
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_null_op<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(12).cast::<*mut u8>();
                            let l2 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_access_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let result1 = T::access(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::Vec::from_raw_parts(arg1.cast(), len0, len0),
                        arg3 as u32,
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            *ptr2.add(4).cast::<i32>() = _rt::as_i32(e);
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code3,
                                message: message3,
                            } = e;
                            match nfs_error_code3 {
                                Some(e) => {
                                    *ptr2.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr2.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr2.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec4 = (message3.into_bytes()).into_boxed_slice();
                            let ptr4 = vec4.as_ptr().cast::<u8>();
                            let len4 = vec4.len();
                            ::core::mem::forget(vec4);
                            *ptr2.add(16).cast::<usize>() = len4;
                            *ptr2.add(12).cast::<*mut u8>() = ptr4.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_access<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(12).cast::<*mut u8>();
                            let l2 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_access_path_cabi<
                    T: GuestNfsMount,
                >(arg0: *mut u8, arg1: *mut u8, arg2: usize, arg3: i32) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let result1 = T::access_path(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                        arg3 as u32,
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            *ptr2.add(4).cast::<i32>() = _rt::as_i32(e);
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code3,
                                message: message3,
                            } = e;
                            match nfs_error_code3 {
                                Some(e) => {
                                    *ptr2.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr2.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr2.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec4 = (message3.into_bytes()).into_boxed_slice();
                            let ptr4 = vec4.as_ptr().cast::<u8>();
                            let len4 = vec4.len();
                            ::core::mem::forget(vec4);
                            *ptr2.add(16).cast::<usize>() = len4;
                            *ptr2.add(12).cast::<*mut u8>() = ptr4.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_access_path<
                    T: GuestNfsMount,
                >(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(12).cast::<*mut u8>();
                            let l2 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_close_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                    arg1: i32,
                    arg2: i64,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::close(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u32,
                        arg2 as u64,
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Ok(_) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                        }
                        Err(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code2,
                                message: message2,
                            } = e;
                            match nfs_error_code2 {
                                Some(e) => {
                                    *ptr1.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr1.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr1.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec3 = (message2.into_bytes()).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *ptr1.add(16).cast::<usize>() = len3;
                            *ptr1.add(12).cast::<*mut u8>() = ptr3.cast_mut();
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_close<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(12).cast::<*mut u8>();
                            let l2 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_commit_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: i64,
                    arg4: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let result1 = T::commit(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::Vec::from_raw_parts(arg1.cast(), len0, len0),
                        arg3 as u64,
                        arg4 as u32,
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(_) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code3,
                                message: message3,
                            } = e;
                            match nfs_error_code3 {
                                Some(e) => {
                                    *ptr2.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr2.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr2.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec4 = (message3.into_bytes()).into_boxed_slice();
                            let ptr4 = vec4.as_ptr().cast::<u8>();
                            let len4 = vec4.len();
                            ::core::mem::forget(vec4);
                            *ptr2.add(16).cast::<usize>() = len4;
                            *ptr2.add(12).cast::<*mut u8>() = ptr4.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_commit<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(12).cast::<*mut u8>();
                            let l2 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_commit_path_cabi<
                    T: GuestNfsMount,
                >(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: i64,
                    arg4: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let result1 = T::commit_path(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                        arg3 as u64,
                        arg4 as u32,
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(_) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code3,
                                message: message3,
                            } = e;
                            match nfs_error_code3 {
                                Some(e) => {
                                    *ptr2.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr2.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr2.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec4 = (message3.into_bytes()).into_boxed_slice();
                            let ptr4 = vec4.as_ptr().cast::<u8>();
                            let len4 = vec4.len();
                            ::core::mem::forget(vec4);
                            *ptr2.add(16).cast::<usize>() = len4;
                            *ptr2.add(12).cast::<*mut u8>() = ptr4.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_commit_path<
                    T: GuestNfsMount,
                >(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(12).cast::<*mut u8>();
                            let l2 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_create_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: *mut u8,
                    arg4: usize,
                    arg5: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let len1 = arg4;
                    let bytes1 = _rt::Vec::from_raw_parts(arg3.cast(), len1, len1);
                    let result2 = T::create(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::Vec::from_raw_parts(arg1.cast(), len0, len0),
                        _rt::string_lift(bytes1),
                        arg5 as u32,
                    );
                    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result2 {
                        Ok(e) => {
                            *ptr3.add(0).cast::<u8>() = (0i32) as u8;
                            let ObjRes { obj: obj4, attr: attr4 } = e;
                            let vec5 = (obj4).into_boxed_slice();
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            ::core::mem::forget(vec5);
                            *ptr3.add(12).cast::<usize>() = len5;
                            *ptr3.add(8).cast::<*mut u8>() = ptr5.cast_mut();
                            match attr4 {
                                Some(e) => {
                                    *ptr3.add(16).cast::<u8>() = (1i32) as u8;
                                    let Attr {
                                        attr_type: attr_type6,
                                        file_mode: file_mode6,
                                        nlink: nlink6,
                                        uid: uid6,
                                        gid: gid6,
                                        filesize: filesize6,
                                        used: used6,
                                        spec_data: spec_data6,
                                        fsid: fsid6,
                                        fileid: fileid6,
                                        atime: atime6,
                                        mtime: mtime6,
                                        ctime: ctime6,
                                    } = e;
                                    *ptr3.add(24).cast::<i32>() = _rt::as_i32(attr_type6);
                                    *ptr3.add(28).cast::<i32>() = _rt::as_i32(file_mode6);
                                    *ptr3.add(32).cast::<i32>() = _rt::as_i32(nlink6);
                                    *ptr3.add(36).cast::<i32>() = _rt::as_i32(uid6);
                                    *ptr3.add(40).cast::<i32>() = _rt::as_i32(gid6);
                                    *ptr3.add(48).cast::<i64>() = _rt::as_i64(filesize6);
                                    *ptr3.add(56).cast::<i64>() = _rt::as_i64(used6);
                                    let (t7_0, t7_1) = spec_data6;
                                    *ptr3.add(64).cast::<i32>() = _rt::as_i32(t7_0);
                                    *ptr3.add(68).cast::<i32>() = _rt::as_i32(t7_1);
                                    *ptr3.add(72).cast::<i64>() = _rt::as_i64(fsid6);
                                    *ptr3.add(80).cast::<i64>() = _rt::as_i64(fileid6);
                                    let Time { seconds: seconds8, nseconds: nseconds8 } = atime6;
                                    *ptr3.add(88).cast::<i32>() = _rt::as_i32(seconds8);
                                    *ptr3.add(92).cast::<i32>() = _rt::as_i32(nseconds8);
                                    let Time { seconds: seconds9, nseconds: nseconds9 } = mtime6;
                                    *ptr3.add(96).cast::<i32>() = _rt::as_i32(seconds9);
                                    *ptr3.add(100).cast::<i32>() = _rt::as_i32(nseconds9);
                                    let Time { seconds: seconds10, nseconds: nseconds10 } = ctime6;
                                    *ptr3.add(104).cast::<i32>() = _rt::as_i32(seconds10);
                                    *ptr3.add(108).cast::<i32>() = _rt::as_i32(nseconds10);
                                }
                                None => {
                                    *ptr3.add(16).cast::<u8>() = (0i32) as u8;
                                }
                            };
                        }
                        Err(e) => {
                            *ptr3.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code11,
                                message: message11,
                            } = e;
                            match nfs_error_code11 {
                                Some(e) => {
                                    *ptr3.add(8).cast::<u8>() = (1i32) as u8;
                                    *ptr3.add(12).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr3.add(8).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec12 = (message11.into_bytes()).into_boxed_slice();
                            let ptr12 = vec12.as_ptr().cast::<u8>();
                            let len12 = vec12.len();
                            ::core::mem::forget(vec12);
                            *ptr3.add(20).cast::<usize>() = len12;
                            *ptr3.add(16).cast::<*mut u8>() = ptr12.cast_mut();
                        }
                    };
                    ptr3
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_create<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(8).cast::<*mut u8>();
                            let l2 = *arg0.add(12).cast::<usize>();
                            let base3 = l1;
                            let len3 = l2;
                            _rt::cabi_dealloc(base3, len3 * 1, 1);
                        }
                        _ => {
                            let l4 = *arg0.add(16).cast::<*mut u8>();
                            let l5 = *arg0.add(20).cast::<usize>();
                            _rt::cabi_dealloc(l4, l5, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_create_path_cabi<
                    T: GuestNfsMount,
                >(arg0: *mut u8, arg1: *mut u8, arg2: usize, arg3: i32) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let result1 = T::create_path(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                        arg3 as u32,
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            let ObjRes { obj: obj3, attr: attr3 } = e;
                            let vec4 = (obj3).into_boxed_slice();
                            let ptr4 = vec4.as_ptr().cast::<u8>();
                            let len4 = vec4.len();
                            ::core::mem::forget(vec4);
                            *ptr2.add(12).cast::<usize>() = len4;
                            *ptr2.add(8).cast::<*mut u8>() = ptr4.cast_mut();
                            match attr3 {
                                Some(e) => {
                                    *ptr2.add(16).cast::<u8>() = (1i32) as u8;
                                    let Attr {
                                        attr_type: attr_type5,
                                        file_mode: file_mode5,
                                        nlink: nlink5,
                                        uid: uid5,
                                        gid: gid5,
                                        filesize: filesize5,
                                        used: used5,
                                        spec_data: spec_data5,
                                        fsid: fsid5,
                                        fileid: fileid5,
                                        atime: atime5,
                                        mtime: mtime5,
                                        ctime: ctime5,
                                    } = e;
                                    *ptr2.add(24).cast::<i32>() = _rt::as_i32(attr_type5);
                                    *ptr2.add(28).cast::<i32>() = _rt::as_i32(file_mode5);
                                    *ptr2.add(32).cast::<i32>() = _rt::as_i32(nlink5);
                                    *ptr2.add(36).cast::<i32>() = _rt::as_i32(uid5);
                                    *ptr2.add(40).cast::<i32>() = _rt::as_i32(gid5);
                                    *ptr2.add(48).cast::<i64>() = _rt::as_i64(filesize5);
                                    *ptr2.add(56).cast::<i64>() = _rt::as_i64(used5);
                                    let (t6_0, t6_1) = spec_data5;
                                    *ptr2.add(64).cast::<i32>() = _rt::as_i32(t6_0);
                                    *ptr2.add(68).cast::<i32>() = _rt::as_i32(t6_1);
                                    *ptr2.add(72).cast::<i64>() = _rt::as_i64(fsid5);
                                    *ptr2.add(80).cast::<i64>() = _rt::as_i64(fileid5);
                                    let Time { seconds: seconds7, nseconds: nseconds7 } = atime5;
                                    *ptr2.add(88).cast::<i32>() = _rt::as_i32(seconds7);
                                    *ptr2.add(92).cast::<i32>() = _rt::as_i32(nseconds7);
                                    let Time { seconds: seconds8, nseconds: nseconds8 } = mtime5;
                                    *ptr2.add(96).cast::<i32>() = _rt::as_i32(seconds8);
                                    *ptr2.add(100).cast::<i32>() = _rt::as_i32(nseconds8);
                                    let Time { seconds: seconds9, nseconds: nseconds9 } = ctime5;
                                    *ptr2.add(104).cast::<i32>() = _rt::as_i32(seconds9);
                                    *ptr2.add(108).cast::<i32>() = _rt::as_i32(nseconds9);
                                }
                                None => {
                                    *ptr2.add(16).cast::<u8>() = (0i32) as u8;
                                }
                            };
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code10,
                                message: message10,
                            } = e;
                            match nfs_error_code10 {
                                Some(e) => {
                                    *ptr2.add(8).cast::<u8>() = (1i32) as u8;
                                    *ptr2.add(12).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr2.add(8).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec11 = (message10.into_bytes()).into_boxed_slice();
                            let ptr11 = vec11.as_ptr().cast::<u8>();
                            let len11 = vec11.len();
                            ::core::mem::forget(vec11);
                            *ptr2.add(20).cast::<usize>() = len11;
                            *ptr2.add(16).cast::<*mut u8>() = ptr11.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_create_path<
                    T: GuestNfsMount,
                >(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(8).cast::<*mut u8>();
                            let l2 = *arg0.add(12).cast::<usize>();
                            let base3 = l1;
                            let len3 = l2;
                            _rt::cabi_dealloc(base3, len3 * 1, 1);
                        }
                        _ => {
                            let l4 = *arg0.add(16).cast::<*mut u8>();
                            let l5 = *arg0.add(20).cast::<usize>();
                            _rt::cabi_dealloc(l4, l5, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_delegpurge_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                    arg1: i64,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::delegpurge(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u64,
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Ok(_) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                        }
                        Err(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code2,
                                message: message2,
                            } = e;
                            match nfs_error_code2 {
                                Some(e) => {
                                    *ptr1.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr1.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr1.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec3 = (message2.into_bytes()).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *ptr1.add(16).cast::<usize>() = len3;
                            *ptr1.add(12).cast::<*mut u8>() = ptr3.cast_mut();
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_delegpurge<
                    T: GuestNfsMount,
                >(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(12).cast::<*mut u8>();
                            let l2 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_delegreturn_cabi<
                    T: GuestNfsMount,
                >(arg0: *mut u8, arg1: i64) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::delegreturn(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        arg1 as u64,
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Ok(_) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                        }
                        Err(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code2,
                                message: message2,
                            } = e;
                            match nfs_error_code2 {
                                Some(e) => {
                                    *ptr1.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr1.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr1.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec3 = (message2.into_bytes()).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *ptr1.add(16).cast::<usize>() = len3;
                            *ptr1.add(12).cast::<*mut u8>() = ptr3.cast_mut();
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_delegreturn<
                    T: GuestNfsMount,
                >(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(12).cast::<*mut u8>();
                            let l2 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_getattr_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let result1 = T::getattr(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::Vec::from_raw_parts(arg1.cast(), len0, len0),
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            let Attr {
                                attr_type: attr_type3,
                                file_mode: file_mode3,
                                nlink: nlink3,
                                uid: uid3,
                                gid: gid3,
                                filesize: filesize3,
                                used: used3,
                                spec_data: spec_data3,
                                fsid: fsid3,
                                fileid: fileid3,
                                atime: atime3,
                                mtime: mtime3,
                                ctime: ctime3,
                            } = e;
                            *ptr2.add(8).cast::<i32>() = _rt::as_i32(attr_type3);
                            *ptr2.add(12).cast::<i32>() = _rt::as_i32(file_mode3);
                            *ptr2.add(16).cast::<i32>() = _rt::as_i32(nlink3);
                            *ptr2.add(20).cast::<i32>() = _rt::as_i32(uid3);
                            *ptr2.add(24).cast::<i32>() = _rt::as_i32(gid3);
                            *ptr2.add(32).cast::<i64>() = _rt::as_i64(filesize3);
                            *ptr2.add(40).cast::<i64>() = _rt::as_i64(used3);
                            let (t4_0, t4_1) = spec_data3;
                            *ptr2.add(48).cast::<i32>() = _rt::as_i32(t4_0);
                            *ptr2.add(52).cast::<i32>() = _rt::as_i32(t4_1);
                            *ptr2.add(56).cast::<i64>() = _rt::as_i64(fsid3);
                            *ptr2.add(64).cast::<i64>() = _rt::as_i64(fileid3);
                            let Time { seconds: seconds5, nseconds: nseconds5 } = atime3;
                            *ptr2.add(72).cast::<i32>() = _rt::as_i32(seconds5);
                            *ptr2.add(76).cast::<i32>() = _rt::as_i32(nseconds5);
                            let Time { seconds: seconds6, nseconds: nseconds6 } = mtime3;
                            *ptr2.add(80).cast::<i32>() = _rt::as_i32(seconds6);
                            *ptr2.add(84).cast::<i32>() = _rt::as_i32(nseconds6);
                            let Time { seconds: seconds7, nseconds: nseconds7 } = ctime3;
                            *ptr2.add(88).cast::<i32>() = _rt::as_i32(seconds7);
                            *ptr2.add(92).cast::<i32>() = _rt::as_i32(nseconds7);
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code8,
                                message: message8,
                            } = e;
                            match nfs_error_code8 {
                                Some(e) => {
                                    *ptr2.add(8).cast::<u8>() = (1i32) as u8;
                                    *ptr2.add(12).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr2.add(8).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec9 = (message8.into_bytes()).into_boxed_slice();
                            let ptr9 = vec9.as_ptr().cast::<u8>();
                            let len9 = vec9.len();
                            ::core::mem::forget(vec9);
                            *ptr2.add(20).cast::<usize>() = len9;
                            *ptr2.add(16).cast::<*mut u8>() = ptr9.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_getattr<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(16).cast::<*mut u8>();
                            let l2 = *arg0.add(20).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_getattr_path_cabi<
                    T: GuestNfsMount,
                >(arg0: *mut u8, arg1: *mut u8, arg2: usize) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let result1 = T::getattr_path(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            let Attr {
                                attr_type: attr_type3,
                                file_mode: file_mode3,
                                nlink: nlink3,
                                uid: uid3,
                                gid: gid3,
                                filesize: filesize3,
                                used: used3,
                                spec_data: spec_data3,
                                fsid: fsid3,
                                fileid: fileid3,
                                atime: atime3,
                                mtime: mtime3,
                                ctime: ctime3,
                            } = e;
                            *ptr2.add(8).cast::<i32>() = _rt::as_i32(attr_type3);
                            *ptr2.add(12).cast::<i32>() = _rt::as_i32(file_mode3);
                            *ptr2.add(16).cast::<i32>() = _rt::as_i32(nlink3);
                            *ptr2.add(20).cast::<i32>() = _rt::as_i32(uid3);
                            *ptr2.add(24).cast::<i32>() = _rt::as_i32(gid3);
                            *ptr2.add(32).cast::<i64>() = _rt::as_i64(filesize3);
                            *ptr2.add(40).cast::<i64>() = _rt::as_i64(used3);
                            let (t4_0, t4_1) = spec_data3;
                            *ptr2.add(48).cast::<i32>() = _rt::as_i32(t4_0);
                            *ptr2.add(52).cast::<i32>() = _rt::as_i32(t4_1);
                            *ptr2.add(56).cast::<i64>() = _rt::as_i64(fsid3);
                            *ptr2.add(64).cast::<i64>() = _rt::as_i64(fileid3);
                            let Time { seconds: seconds5, nseconds: nseconds5 } = atime3;
                            *ptr2.add(72).cast::<i32>() = _rt::as_i32(seconds5);
                            *ptr2.add(76).cast::<i32>() = _rt::as_i32(nseconds5);
                            let Time { seconds: seconds6, nseconds: nseconds6 } = mtime3;
                            *ptr2.add(80).cast::<i32>() = _rt::as_i32(seconds6);
                            *ptr2.add(84).cast::<i32>() = _rt::as_i32(nseconds6);
                            let Time { seconds: seconds7, nseconds: nseconds7 } = ctime3;
                            *ptr2.add(88).cast::<i32>() = _rt::as_i32(seconds7);
                            *ptr2.add(92).cast::<i32>() = _rt::as_i32(nseconds7);
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code8,
                                message: message8,
                            } = e;
                            match nfs_error_code8 {
                                Some(e) => {
                                    *ptr2.add(8).cast::<u8>() = (1i32) as u8;
                                    *ptr2.add(12).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr2.add(8).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec9 = (message8.into_bytes()).into_boxed_slice();
                            let ptr9 = vec9.as_ptr().cast::<u8>();
                            let len9 = vec9.len();
                            ::core::mem::forget(vec9);
                            *ptr2.add(20).cast::<usize>() = len9;
                            *ptr2.add(16).cast::<*mut u8>() = ptr9.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_getattr_path<
                    T: GuestNfsMount,
                >(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(16).cast::<*mut u8>();
                            let l2 = *arg0.add(20).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_setattr_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let l0 = *arg0.add(0).cast::<i32>();
                    let l1 = *arg0.add(4).cast::<*mut u8>();
                    let l2 = *arg0.add(8).cast::<usize>();
                    let len3 = l2;
                    let l4 = i32::from(*arg0.add(12).cast::<u8>());
                    let l7 = i32::from(*arg0.add(24).cast::<u8>());
                    let l9 = i32::from(*arg0.add(32).cast::<u8>());
                    let l11 = i32::from(*arg0.add(40).cast::<u8>());
                    let l13 = i32::from(*arg0.add(48).cast::<u8>());
                    let l15 = i32::from(*arg0.add(64).cast::<u8>());
                    let l18 = i32::from(*arg0.add(76).cast::<u8>());
                    let result21 = T::setattr(
                        NfsMountBorrow::lift(l0 as u32 as usize).get(),
                        _rt::Vec::from_raw_parts(l1.cast(), len3, len3),
                        match l4 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l5 = *arg0.add(16).cast::<i32>();
                                    let l6 = *arg0.add(20).cast::<i32>();
                                    Time {
                                        seconds: l5 as u32,
                                        nseconds: l6 as u32,
                                    }
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                        match l7 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l8 = *arg0.add(28).cast::<i32>();
                                    l8 as u32
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                        match l9 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l10 = *arg0.add(36).cast::<i32>();
                                    l10 as u32
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                        match l11 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l12 = *arg0.add(44).cast::<i32>();
                                    l12 as u32
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                        match l13 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l14 = *arg0.add(56).cast::<i64>();
                                    l14 as u64
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                        match l15 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l16 = *arg0.add(68).cast::<i32>();
                                    let l17 = *arg0.add(72).cast::<i32>();
                                    Time {
                                        seconds: l16 as u32,
                                        nseconds: l17 as u32,
                                    }
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                        match l18 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l19 = *arg0.add(80).cast::<i32>();
                                    let l20 = *arg0.add(84).cast::<i32>();
                                    Time {
                                        seconds: l19 as u32,
                                        nseconds: l20 as u32,
                                    }
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                    );
                    _rt::cabi_dealloc(arg0, 88, 8);
                    let ptr22 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result21 {
                        Ok(_) => {
                            *ptr22.add(0).cast::<u8>() = (0i32) as u8;
                        }
                        Err(e) => {
                            *ptr22.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code23,
                                message: message23,
                            } = e;
                            match nfs_error_code23 {
                                Some(e) => {
                                    *ptr22.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr22.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr22.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec24 = (message23.into_bytes()).into_boxed_slice();
                            let ptr24 = vec24.as_ptr().cast::<u8>();
                            let len24 = vec24.len();
                            ::core::mem::forget(vec24);
                            *ptr22.add(16).cast::<usize>() = len24;
                            *ptr22.add(12).cast::<*mut u8>() = ptr24.cast_mut();
                        }
                    };
                    ptr22
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_setattr<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(12).cast::<*mut u8>();
                            let l2 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_setattr_path_cabi<
                    T: GuestNfsMount,
                >(arg0: *mut u8) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let l0 = *arg0.add(0).cast::<i32>();
                    let l1 = *arg0.add(4).cast::<*mut u8>();
                    let l2 = *arg0.add(8).cast::<usize>();
                    let len3 = l2;
                    let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                    let l4 = i32::from(*arg0.add(12).cast::<u8>());
                    let l5 = i32::from(*arg0.add(16).cast::<u8>());
                    let l7 = i32::from(*arg0.add(24).cast::<u8>());
                    let l9 = i32::from(*arg0.add(32).cast::<u8>());
                    let l11 = i32::from(*arg0.add(40).cast::<u8>());
                    let l13 = i32::from(*arg0.add(56).cast::<u8>());
                    let l16 = i32::from(*arg0.add(68).cast::<u8>());
                    let result19 = T::setattr_path(
                        NfsMountBorrow::lift(l0 as u32 as usize).get(),
                        _rt::string_lift(bytes3),
                        _rt::bool_lift(l4 as u8),
                        match l5 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l6 = *arg0.add(20).cast::<i32>();
                                    l6 as u32
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                        match l7 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l8 = *arg0.add(28).cast::<i32>();
                                    l8 as u32
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                        match l9 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l10 = *arg0.add(36).cast::<i32>();
                                    l10 as u32
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                        match l11 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l12 = *arg0.add(48).cast::<i64>();
                                    l12 as u64
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                        match l13 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l14 = *arg0.add(60).cast::<i32>();
                                    let l15 = *arg0.add(64).cast::<i32>();
                                    Time {
                                        seconds: l14 as u32,
                                        nseconds: l15 as u32,
                                    }
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                        match l16 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l17 = *arg0.add(72).cast::<i32>();
                                    let l18 = *arg0.add(76).cast::<i32>();
                                    Time {
                                        seconds: l17 as u32,
                                        nseconds: l18 as u32,
                                    }
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                    );
                    _rt::cabi_dealloc(arg0, 80, 8);
                    let ptr20 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result19 {
                        Ok(_) => {
                            *ptr20.add(0).cast::<u8>() = (0i32) as u8;
                        }
                        Err(e) => {
                            *ptr20.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code21,
                                message: message21,
                            } = e;
                            match nfs_error_code21 {
                                Some(e) => {
                                    *ptr20.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr20.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr20.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec22 = (message21.into_bytes()).into_boxed_slice();
                            let ptr22 = vec22.as_ptr().cast::<u8>();
                            let len22 = vec22.len();
                            ::core::mem::forget(vec22);
                            *ptr20.add(16).cast::<usize>() = len22;
                            *ptr20.add(12).cast::<*mut u8>() = ptr22.cast_mut();
                        }
                    };
                    ptr20
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_setattr_path<
                    T: GuestNfsMount,
                >(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(12).cast::<*mut u8>();
                            let l2 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_getfh_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::getfh(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Ok(_) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                        }
                        Err(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code2,
                                message: message2,
                            } = e;
                            match nfs_error_code2 {
                                Some(e) => {
                                    *ptr1.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr1.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr1.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec3 = (message2.into_bytes()).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *ptr1.add(16).cast::<usize>() = len3;
                            *ptr1.add(12).cast::<*mut u8>() = ptr3.cast_mut();
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_getfh<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(12).cast::<*mut u8>();
                            let l2 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_link_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: *mut u8,
                    arg4: usize,
                    arg5: *mut u8,
                    arg6: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let len1 = arg4;
                    let len2 = arg6;
                    let bytes2 = _rt::Vec::from_raw_parts(arg5.cast(), len2, len2);
                    let result3 = T::link(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::Vec::from_raw_parts(arg1.cast(), len0, len0),
                        _rt::Vec::from_raw_parts(arg3.cast(), len1, len1),
                        _rt::string_lift(bytes2),
                    );
                    let ptr4 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result3 {
                        Ok(e) => {
                            *ptr4.add(0).cast::<u8>() = (0i32) as u8;
                            let Attr {
                                attr_type: attr_type5,
                                file_mode: file_mode5,
                                nlink: nlink5,
                                uid: uid5,
                                gid: gid5,
                                filesize: filesize5,
                                used: used5,
                                spec_data: spec_data5,
                                fsid: fsid5,
                                fileid: fileid5,
                                atime: atime5,
                                mtime: mtime5,
                                ctime: ctime5,
                            } = e;
                            *ptr4.add(8).cast::<i32>() = _rt::as_i32(attr_type5);
                            *ptr4.add(12).cast::<i32>() = _rt::as_i32(file_mode5);
                            *ptr4.add(16).cast::<i32>() = _rt::as_i32(nlink5);
                            *ptr4.add(20).cast::<i32>() = _rt::as_i32(uid5);
                            *ptr4.add(24).cast::<i32>() = _rt::as_i32(gid5);
                            *ptr4.add(32).cast::<i64>() = _rt::as_i64(filesize5);
                            *ptr4.add(40).cast::<i64>() = _rt::as_i64(used5);
                            let (t6_0, t6_1) = spec_data5;
                            *ptr4.add(48).cast::<i32>() = _rt::as_i32(t6_0);
                            *ptr4.add(52).cast::<i32>() = _rt::as_i32(t6_1);
                            *ptr4.add(56).cast::<i64>() = _rt::as_i64(fsid5);
                            *ptr4.add(64).cast::<i64>() = _rt::as_i64(fileid5);
                            let Time { seconds: seconds7, nseconds: nseconds7 } = atime5;
                            *ptr4.add(72).cast::<i32>() = _rt::as_i32(seconds7);
                            *ptr4.add(76).cast::<i32>() = _rt::as_i32(nseconds7);
                            let Time { seconds: seconds8, nseconds: nseconds8 } = mtime5;
                            *ptr4.add(80).cast::<i32>() = _rt::as_i32(seconds8);
                            *ptr4.add(84).cast::<i32>() = _rt::as_i32(nseconds8);
                            let Time { seconds: seconds9, nseconds: nseconds9 } = ctime5;
                            *ptr4.add(88).cast::<i32>() = _rt::as_i32(seconds9);
                            *ptr4.add(92).cast::<i32>() = _rt::as_i32(nseconds9);
                        }
                        Err(e) => {
                            *ptr4.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code10,
                                message: message10,
                            } = e;
                            match nfs_error_code10 {
                                Some(e) => {
                                    *ptr4.add(8).cast::<u8>() = (1i32) as u8;
                                    *ptr4.add(12).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr4.add(8).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec11 = (message10.into_bytes()).into_boxed_slice();
                            let ptr11 = vec11.as_ptr().cast::<u8>();
                            let len11 = vec11.len();
                            ::core::mem::forget(vec11);
                            *ptr4.add(20).cast::<usize>() = len11;
                            *ptr4.add(16).cast::<*mut u8>() = ptr11.cast_mut();
                        }
                    };
                    ptr4
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_link<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(16).cast::<*mut u8>();
                            let l2 = *arg0.add(20).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_link_path_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: *mut u8,
                    arg4: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let len1 = arg4;
                    let bytes1 = _rt::Vec::from_raw_parts(arg3.cast(), len1, len1);
                    let result2 = T::link_path(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                        _rt::string_lift(bytes1),
                    );
                    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result2 {
                        Ok(e) => {
                            *ptr3.add(0).cast::<u8>() = (0i32) as u8;
                            let Attr {
                                attr_type: attr_type4,
                                file_mode: file_mode4,
                                nlink: nlink4,
                                uid: uid4,
                                gid: gid4,
                                filesize: filesize4,
                                used: used4,
                                spec_data: spec_data4,
                                fsid: fsid4,
                                fileid: fileid4,
                                atime: atime4,
                                mtime: mtime4,
                                ctime: ctime4,
                            } = e;
                            *ptr3.add(8).cast::<i32>() = _rt::as_i32(attr_type4);
                            *ptr3.add(12).cast::<i32>() = _rt::as_i32(file_mode4);
                            *ptr3.add(16).cast::<i32>() = _rt::as_i32(nlink4);
                            *ptr3.add(20).cast::<i32>() = _rt::as_i32(uid4);
                            *ptr3.add(24).cast::<i32>() = _rt::as_i32(gid4);
                            *ptr3.add(32).cast::<i64>() = _rt::as_i64(filesize4);
                            *ptr3.add(40).cast::<i64>() = _rt::as_i64(used4);
                            let (t5_0, t5_1) = spec_data4;
                            *ptr3.add(48).cast::<i32>() = _rt::as_i32(t5_0);
                            *ptr3.add(52).cast::<i32>() = _rt::as_i32(t5_1);
                            *ptr3.add(56).cast::<i64>() = _rt::as_i64(fsid4);
                            *ptr3.add(64).cast::<i64>() = _rt::as_i64(fileid4);
                            let Time { seconds: seconds6, nseconds: nseconds6 } = atime4;
                            *ptr3.add(72).cast::<i32>() = _rt::as_i32(seconds6);
                            *ptr3.add(76).cast::<i32>() = _rt::as_i32(nseconds6);
                            let Time { seconds: seconds7, nseconds: nseconds7 } = mtime4;
                            *ptr3.add(80).cast::<i32>() = _rt::as_i32(seconds7);
                            *ptr3.add(84).cast::<i32>() = _rt::as_i32(nseconds7);
                            let Time { seconds: seconds8, nseconds: nseconds8 } = ctime4;
                            *ptr3.add(88).cast::<i32>() = _rt::as_i32(seconds8);
                            *ptr3.add(92).cast::<i32>() = _rt::as_i32(nseconds8);
                        }
                        Err(e) => {
                            *ptr3.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code9,
                                message: message9,
                            } = e;
                            match nfs_error_code9 {
                                Some(e) => {
                                    *ptr3.add(8).cast::<u8>() = (1i32) as u8;
                                    *ptr3.add(12).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr3.add(8).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec10 = (message9.into_bytes()).into_boxed_slice();
                            let ptr10 = vec10.as_ptr().cast::<u8>();
                            let len10 = vec10.len();
                            ::core::mem::forget(vec10);
                            *ptr3.add(20).cast::<usize>() = len10;
                            *ptr3.add(16).cast::<*mut u8>() = ptr10.cast_mut();
                        }
                    };
                    ptr3
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_link_path<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(16).cast::<*mut u8>();
                            let l2 = *arg0.add(20).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_symlink_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: *mut u8,
                    arg4: usize,
                    arg5: *mut u8,
                    arg6: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let len1 = arg4;
                    let len2 = arg6;
                    let bytes2 = _rt::Vec::from_raw_parts(arg5.cast(), len2, len2);
                    let result3 = T::symlink(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                        _rt::Vec::from_raw_parts(arg3.cast(), len1, len1),
                        _rt::string_lift(bytes2),
                    );
                    let ptr4 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result3 {
                        Ok(e) => {
                            *ptr4.add(0).cast::<u8>() = (0i32) as u8;
                            let ObjRes { obj: obj5, attr: attr5 } = e;
                            let vec6 = (obj5).into_boxed_slice();
                            let ptr6 = vec6.as_ptr().cast::<u8>();
                            let len6 = vec6.len();
                            ::core::mem::forget(vec6);
                            *ptr4.add(12).cast::<usize>() = len6;
                            *ptr4.add(8).cast::<*mut u8>() = ptr6.cast_mut();
                            match attr5 {
                                Some(e) => {
                                    *ptr4.add(16).cast::<u8>() = (1i32) as u8;
                                    let Attr {
                                        attr_type: attr_type7,
                                        file_mode: file_mode7,
                                        nlink: nlink7,
                                        uid: uid7,
                                        gid: gid7,
                                        filesize: filesize7,
                                        used: used7,
                                        spec_data: spec_data7,
                                        fsid: fsid7,
                                        fileid: fileid7,
                                        atime: atime7,
                                        mtime: mtime7,
                                        ctime: ctime7,
                                    } = e;
                                    *ptr4.add(24).cast::<i32>() = _rt::as_i32(attr_type7);
                                    *ptr4.add(28).cast::<i32>() = _rt::as_i32(file_mode7);
                                    *ptr4.add(32).cast::<i32>() = _rt::as_i32(nlink7);
                                    *ptr4.add(36).cast::<i32>() = _rt::as_i32(uid7);
                                    *ptr4.add(40).cast::<i32>() = _rt::as_i32(gid7);
                                    *ptr4.add(48).cast::<i64>() = _rt::as_i64(filesize7);
                                    *ptr4.add(56).cast::<i64>() = _rt::as_i64(used7);
                                    let (t8_0, t8_1) = spec_data7;
                                    *ptr4.add(64).cast::<i32>() = _rt::as_i32(t8_0);
                                    *ptr4.add(68).cast::<i32>() = _rt::as_i32(t8_1);
                                    *ptr4.add(72).cast::<i64>() = _rt::as_i64(fsid7);
                                    *ptr4.add(80).cast::<i64>() = _rt::as_i64(fileid7);
                                    let Time { seconds: seconds9, nseconds: nseconds9 } = atime7;
                                    *ptr4.add(88).cast::<i32>() = _rt::as_i32(seconds9);
                                    *ptr4.add(92).cast::<i32>() = _rt::as_i32(nseconds9);
                                    let Time { seconds: seconds10, nseconds: nseconds10 } = mtime7;
                                    *ptr4.add(96).cast::<i32>() = _rt::as_i32(seconds10);
                                    *ptr4.add(100).cast::<i32>() = _rt::as_i32(nseconds10);
                                    let Time { seconds: seconds11, nseconds: nseconds11 } = ctime7;
                                    *ptr4.add(104).cast::<i32>() = _rt::as_i32(seconds11);
                                    *ptr4.add(108).cast::<i32>() = _rt::as_i32(nseconds11);
                                }
                                None => {
                                    *ptr4.add(16).cast::<u8>() = (0i32) as u8;
                                }
                            };
                        }
                        Err(e) => {
                            *ptr4.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code12,
                                message: message12,
                            } = e;
                            match nfs_error_code12 {
                                Some(e) => {
                                    *ptr4.add(8).cast::<u8>() = (1i32) as u8;
                                    *ptr4.add(12).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr4.add(8).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec13 = (message12.into_bytes()).into_boxed_slice();
                            let ptr13 = vec13.as_ptr().cast::<u8>();
                            let len13 = vec13.len();
                            ::core::mem::forget(vec13);
                            *ptr4.add(20).cast::<usize>() = len13;
                            *ptr4.add(16).cast::<*mut u8>() = ptr13.cast_mut();
                        }
                    };
                    ptr4
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_symlink<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(8).cast::<*mut u8>();
                            let l2 = *arg0.add(12).cast::<usize>();
                            let base3 = l1;
                            let len3 = l2;
                            _rt::cabi_dealloc(base3, len3 * 1, 1);
                        }
                        _ => {
                            let l4 = *arg0.add(16).cast::<*mut u8>();
                            let l5 = *arg0.add(20).cast::<usize>();
                            _rt::cabi_dealloc(l4, l5, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_symlink_path_cabi<
                    T: GuestNfsMount,
                >(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: *mut u8,
                    arg4: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let len1 = arg4;
                    let bytes1 = _rt::Vec::from_raw_parts(arg3.cast(), len1, len1);
                    let result2 = T::symlink_path(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                        _rt::string_lift(bytes1),
                    );
                    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result2 {
                        Ok(e) => {
                            *ptr3.add(0).cast::<u8>() = (0i32) as u8;
                            let ObjRes { obj: obj4, attr: attr4 } = e;
                            let vec5 = (obj4).into_boxed_slice();
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            ::core::mem::forget(vec5);
                            *ptr3.add(12).cast::<usize>() = len5;
                            *ptr3.add(8).cast::<*mut u8>() = ptr5.cast_mut();
                            match attr4 {
                                Some(e) => {
                                    *ptr3.add(16).cast::<u8>() = (1i32) as u8;
                                    let Attr {
                                        attr_type: attr_type6,
                                        file_mode: file_mode6,
                                        nlink: nlink6,
                                        uid: uid6,
                                        gid: gid6,
                                        filesize: filesize6,
                                        used: used6,
                                        spec_data: spec_data6,
                                        fsid: fsid6,
                                        fileid: fileid6,
                                        atime: atime6,
                                        mtime: mtime6,
                                        ctime: ctime6,
                                    } = e;
                                    *ptr3.add(24).cast::<i32>() = _rt::as_i32(attr_type6);
                                    *ptr3.add(28).cast::<i32>() = _rt::as_i32(file_mode6);
                                    *ptr3.add(32).cast::<i32>() = _rt::as_i32(nlink6);
                                    *ptr3.add(36).cast::<i32>() = _rt::as_i32(uid6);
                                    *ptr3.add(40).cast::<i32>() = _rt::as_i32(gid6);
                                    *ptr3.add(48).cast::<i64>() = _rt::as_i64(filesize6);
                                    *ptr3.add(56).cast::<i64>() = _rt::as_i64(used6);
                                    let (t7_0, t7_1) = spec_data6;
                                    *ptr3.add(64).cast::<i32>() = _rt::as_i32(t7_0);
                                    *ptr3.add(68).cast::<i32>() = _rt::as_i32(t7_1);
                                    *ptr3.add(72).cast::<i64>() = _rt::as_i64(fsid6);
                                    *ptr3.add(80).cast::<i64>() = _rt::as_i64(fileid6);
                                    let Time { seconds: seconds8, nseconds: nseconds8 } = atime6;
                                    *ptr3.add(88).cast::<i32>() = _rt::as_i32(seconds8);
                                    *ptr3.add(92).cast::<i32>() = _rt::as_i32(nseconds8);
                                    let Time { seconds: seconds9, nseconds: nseconds9 } = mtime6;
                                    *ptr3.add(96).cast::<i32>() = _rt::as_i32(seconds9);
                                    *ptr3.add(100).cast::<i32>() = _rt::as_i32(nseconds9);
                                    let Time { seconds: seconds10, nseconds: nseconds10 } = ctime6;
                                    *ptr3.add(104).cast::<i32>() = _rt::as_i32(seconds10);
                                    *ptr3.add(108).cast::<i32>() = _rt::as_i32(nseconds10);
                                }
                                None => {
                                    *ptr3.add(16).cast::<u8>() = (0i32) as u8;
                                }
                            };
                        }
                        Err(e) => {
                            *ptr3.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code11,
                                message: message11,
                            } = e;
                            match nfs_error_code11 {
                                Some(e) => {
                                    *ptr3.add(8).cast::<u8>() = (1i32) as u8;
                                    *ptr3.add(12).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr3.add(8).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec12 = (message11.into_bytes()).into_boxed_slice();
                            let ptr12 = vec12.as_ptr().cast::<u8>();
                            let len12 = vec12.len();
                            ::core::mem::forget(vec12);
                            *ptr3.add(20).cast::<usize>() = len12;
                            *ptr3.add(16).cast::<*mut u8>() = ptr12.cast_mut();
                        }
                    };
                    ptr3
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_symlink_path<
                    T: GuestNfsMount,
                >(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(8).cast::<*mut u8>();
                            let l2 = *arg0.add(12).cast::<usize>();
                            let base3 = l1;
                            let len3 = l2;
                            _rt::cabi_dealloc(base3, len3 * 1, 1);
                        }
                        _ => {
                            let l4 = *arg0.add(16).cast::<*mut u8>();
                            let l5 = *arg0.add(20).cast::<usize>();
                            _rt::cabi_dealloc(l4, l5, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_readlink_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let result1 = T::readlink(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::Vec::from_raw_parts(arg1.cast(), len0, len0),
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            let vec3 = (e.into_bytes()).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *ptr2.add(8).cast::<usize>() = len3;
                            *ptr2.add(4).cast::<*mut u8>() = ptr3.cast_mut();
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code4,
                                message: message4,
                            } = e;
                            match nfs_error_code4 {
                                Some(e) => {
                                    *ptr2.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr2.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr2.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec5 = (message4.into_bytes()).into_boxed_slice();
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            ::core::mem::forget(vec5);
                            *ptr2.add(16).cast::<usize>() = len5;
                            *ptr2.add(12).cast::<*mut u8>() = ptr5.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_readlink<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                        _ => {
                            let l3 = *arg0.add(12).cast::<*mut u8>();
                            let l4 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l3, l4, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_readlink_path_cabi<
                    T: GuestNfsMount,
                >(arg0: *mut u8, arg1: *mut u8, arg2: usize) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let result1 = T::readlink_path(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            let vec3 = (e.into_bytes()).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *ptr2.add(8).cast::<usize>() = len3;
                            *ptr2.add(4).cast::<*mut u8>() = ptr3.cast_mut();
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code4,
                                message: message4,
                            } = e;
                            match nfs_error_code4 {
                                Some(e) => {
                                    *ptr2.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr2.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr2.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec5 = (message4.into_bytes()).into_boxed_slice();
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            ::core::mem::forget(vec5);
                            *ptr2.add(16).cast::<usize>() = len5;
                            *ptr2.add(12).cast::<*mut u8>() = ptr5.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_readlink_path<
                    T: GuestNfsMount,
                >(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                        _ => {
                            let l3 = *arg0.add(12).cast::<*mut u8>();
                            let l4 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l3, l4, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_lookup_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: *mut u8,
                    arg4: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let len1 = arg4;
                    let bytes1 = _rt::Vec::from_raw_parts(arg3.cast(), len1, len1);
                    let result2 = T::lookup(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::Vec::from_raw_parts(arg1.cast(), len0, len0),
                        _rt::string_lift(bytes1),
                    );
                    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result2 {
                        Ok(e) => {
                            *ptr3.add(0).cast::<u8>() = (0i32) as u8;
                            let ObjRes { obj: obj4, attr: attr4 } = e;
                            let vec5 = (obj4).into_boxed_slice();
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            ::core::mem::forget(vec5);
                            *ptr3.add(12).cast::<usize>() = len5;
                            *ptr3.add(8).cast::<*mut u8>() = ptr5.cast_mut();
                            match attr4 {
                                Some(e) => {
                                    *ptr3.add(16).cast::<u8>() = (1i32) as u8;
                                    let Attr {
                                        attr_type: attr_type6,
                                        file_mode: file_mode6,
                                        nlink: nlink6,
                                        uid: uid6,
                                        gid: gid6,
                                        filesize: filesize6,
                                        used: used6,
                                        spec_data: spec_data6,
                                        fsid: fsid6,
                                        fileid: fileid6,
                                        atime: atime6,
                                        mtime: mtime6,
                                        ctime: ctime6,
                                    } = e;
                                    *ptr3.add(24).cast::<i32>() = _rt::as_i32(attr_type6);
                                    *ptr3.add(28).cast::<i32>() = _rt::as_i32(file_mode6);
                                    *ptr3.add(32).cast::<i32>() = _rt::as_i32(nlink6);
                                    *ptr3.add(36).cast::<i32>() = _rt::as_i32(uid6);
                                    *ptr3.add(40).cast::<i32>() = _rt::as_i32(gid6);
                                    *ptr3.add(48).cast::<i64>() = _rt::as_i64(filesize6);
                                    *ptr3.add(56).cast::<i64>() = _rt::as_i64(used6);
                                    let (t7_0, t7_1) = spec_data6;
                                    *ptr3.add(64).cast::<i32>() = _rt::as_i32(t7_0);
                                    *ptr3.add(68).cast::<i32>() = _rt::as_i32(t7_1);
                                    *ptr3.add(72).cast::<i64>() = _rt::as_i64(fsid6);
                                    *ptr3.add(80).cast::<i64>() = _rt::as_i64(fileid6);
                                    let Time { seconds: seconds8, nseconds: nseconds8 } = atime6;
                                    *ptr3.add(88).cast::<i32>() = _rt::as_i32(seconds8);
                                    *ptr3.add(92).cast::<i32>() = _rt::as_i32(nseconds8);
                                    let Time { seconds: seconds9, nseconds: nseconds9 } = mtime6;
                                    *ptr3.add(96).cast::<i32>() = _rt::as_i32(seconds9);
                                    *ptr3.add(100).cast::<i32>() = _rt::as_i32(nseconds9);
                                    let Time { seconds: seconds10, nseconds: nseconds10 } = ctime6;
                                    *ptr3.add(104).cast::<i32>() = _rt::as_i32(seconds10);
                                    *ptr3.add(108).cast::<i32>() = _rt::as_i32(nseconds10);
                                }
                                None => {
                                    *ptr3.add(16).cast::<u8>() = (0i32) as u8;
                                }
                            };
                        }
                        Err(e) => {
                            *ptr3.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code11,
                                message: message11,
                            } = e;
                            match nfs_error_code11 {
                                Some(e) => {
                                    *ptr3.add(8).cast::<u8>() = (1i32) as u8;
                                    *ptr3.add(12).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr3.add(8).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec12 = (message11.into_bytes()).into_boxed_slice();
                            let ptr12 = vec12.as_ptr().cast::<u8>();
                            let len12 = vec12.len();
                            ::core::mem::forget(vec12);
                            *ptr3.add(20).cast::<usize>() = len12;
                            *ptr3.add(16).cast::<*mut u8>() = ptr12.cast_mut();
                        }
                    };
                    ptr3
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_lookup<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(8).cast::<*mut u8>();
                            let l2 = *arg0.add(12).cast::<usize>();
                            let base3 = l1;
                            let len3 = l2;
                            _rt::cabi_dealloc(base3, len3 * 1, 1);
                        }
                        _ => {
                            let l4 = *arg0.add(16).cast::<*mut u8>();
                            let l5 = *arg0.add(20).cast::<usize>();
                            _rt::cabi_dealloc(l4, l5, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_lookup_path_cabi<
                    T: GuestNfsMount,
                >(arg0: *mut u8, arg1: *mut u8, arg2: usize) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let result1 = T::lookup_path(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            let ObjRes { obj: obj3, attr: attr3 } = e;
                            let vec4 = (obj3).into_boxed_slice();
                            let ptr4 = vec4.as_ptr().cast::<u8>();
                            let len4 = vec4.len();
                            ::core::mem::forget(vec4);
                            *ptr2.add(12).cast::<usize>() = len4;
                            *ptr2.add(8).cast::<*mut u8>() = ptr4.cast_mut();
                            match attr3 {
                                Some(e) => {
                                    *ptr2.add(16).cast::<u8>() = (1i32) as u8;
                                    let Attr {
                                        attr_type: attr_type5,
                                        file_mode: file_mode5,
                                        nlink: nlink5,
                                        uid: uid5,
                                        gid: gid5,
                                        filesize: filesize5,
                                        used: used5,
                                        spec_data: spec_data5,
                                        fsid: fsid5,
                                        fileid: fileid5,
                                        atime: atime5,
                                        mtime: mtime5,
                                        ctime: ctime5,
                                    } = e;
                                    *ptr2.add(24).cast::<i32>() = _rt::as_i32(attr_type5);
                                    *ptr2.add(28).cast::<i32>() = _rt::as_i32(file_mode5);
                                    *ptr2.add(32).cast::<i32>() = _rt::as_i32(nlink5);
                                    *ptr2.add(36).cast::<i32>() = _rt::as_i32(uid5);
                                    *ptr2.add(40).cast::<i32>() = _rt::as_i32(gid5);
                                    *ptr2.add(48).cast::<i64>() = _rt::as_i64(filesize5);
                                    *ptr2.add(56).cast::<i64>() = _rt::as_i64(used5);
                                    let (t6_0, t6_1) = spec_data5;
                                    *ptr2.add(64).cast::<i32>() = _rt::as_i32(t6_0);
                                    *ptr2.add(68).cast::<i32>() = _rt::as_i32(t6_1);
                                    *ptr2.add(72).cast::<i64>() = _rt::as_i64(fsid5);
                                    *ptr2.add(80).cast::<i64>() = _rt::as_i64(fileid5);
                                    let Time { seconds: seconds7, nseconds: nseconds7 } = atime5;
                                    *ptr2.add(88).cast::<i32>() = _rt::as_i32(seconds7);
                                    *ptr2.add(92).cast::<i32>() = _rt::as_i32(nseconds7);
                                    let Time { seconds: seconds8, nseconds: nseconds8 } = mtime5;
                                    *ptr2.add(96).cast::<i32>() = _rt::as_i32(seconds8);
                                    *ptr2.add(100).cast::<i32>() = _rt::as_i32(nseconds8);
                                    let Time { seconds: seconds9, nseconds: nseconds9 } = ctime5;
                                    *ptr2.add(104).cast::<i32>() = _rt::as_i32(seconds9);
                                    *ptr2.add(108).cast::<i32>() = _rt::as_i32(nseconds9);
                                }
                                None => {
                                    *ptr2.add(16).cast::<u8>() = (0i32) as u8;
                                }
                            };
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code10,
                                message: message10,
                            } = e;
                            match nfs_error_code10 {
                                Some(e) => {
                                    *ptr2.add(8).cast::<u8>() = (1i32) as u8;
                                    *ptr2.add(12).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr2.add(8).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec11 = (message10.into_bytes()).into_boxed_slice();
                            let ptr11 = vec11.as_ptr().cast::<u8>();
                            let len11 = vec11.len();
                            ::core::mem::forget(vec11);
                            *ptr2.add(20).cast::<usize>() = len11;
                            *ptr2.add(16).cast::<*mut u8>() = ptr11.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_lookup_path<
                    T: GuestNfsMount,
                >(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(8).cast::<*mut u8>();
                            let l2 = *arg0.add(12).cast::<usize>();
                            let base3 = l1;
                            let len3 = l2;
                            _rt::cabi_dealloc(base3, len3 * 1, 1);
                        }
                        _ => {
                            let l4 = *arg0.add(16).cast::<*mut u8>();
                            let l5 = *arg0.add(20).cast::<usize>();
                            _rt::cabi_dealloc(l4, l5, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_pathconf_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let result1 = T::pathconf(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::Vec::from_raw_parts(arg1.cast(), len0, len0),
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            let PathConf {
                                attr: attr3,
                                linkmax: linkmax3,
                                name_max: name_max3,
                                no_trunc: no_trunc3,
                                chown_restricted: chown_restricted3,
                                case_insensitive: case_insensitive3,
                                case_preserving: case_preserving3,
                            } = e;
                            match attr3 {
                                Some(e) => {
                                    *ptr2.add(8).cast::<u8>() = (1i32) as u8;
                                    let Attr {
                                        attr_type: attr_type4,
                                        file_mode: file_mode4,
                                        nlink: nlink4,
                                        uid: uid4,
                                        gid: gid4,
                                        filesize: filesize4,
                                        used: used4,
                                        spec_data: spec_data4,
                                        fsid: fsid4,
                                        fileid: fileid4,
                                        atime: atime4,
                                        mtime: mtime4,
                                        ctime: ctime4,
                                    } = e;
                                    *ptr2.add(16).cast::<i32>() = _rt::as_i32(attr_type4);
                                    *ptr2.add(20).cast::<i32>() = _rt::as_i32(file_mode4);
                                    *ptr2.add(24).cast::<i32>() = _rt::as_i32(nlink4);
                                    *ptr2.add(28).cast::<i32>() = _rt::as_i32(uid4);
                                    *ptr2.add(32).cast::<i32>() = _rt::as_i32(gid4);
                                    *ptr2.add(40).cast::<i64>() = _rt::as_i64(filesize4);
                                    *ptr2.add(48).cast::<i64>() = _rt::as_i64(used4);
                                    let (t5_0, t5_1) = spec_data4;
                                    *ptr2.add(56).cast::<i32>() = _rt::as_i32(t5_0);
                                    *ptr2.add(60).cast::<i32>() = _rt::as_i32(t5_1);
                                    *ptr2.add(64).cast::<i64>() = _rt::as_i64(fsid4);
                                    *ptr2.add(72).cast::<i64>() = _rt::as_i64(fileid4);
                                    let Time { seconds: seconds6, nseconds: nseconds6 } = atime4;
                                    *ptr2.add(80).cast::<i32>() = _rt::as_i32(seconds6);
                                    *ptr2.add(84).cast::<i32>() = _rt::as_i32(nseconds6);
                                    let Time { seconds: seconds7, nseconds: nseconds7 } = mtime4;
                                    *ptr2.add(88).cast::<i32>() = _rt::as_i32(seconds7);
                                    *ptr2.add(92).cast::<i32>() = _rt::as_i32(nseconds7);
                                    let Time { seconds: seconds8, nseconds: nseconds8 } = ctime4;
                                    *ptr2.add(96).cast::<i32>() = _rt::as_i32(seconds8);
                                    *ptr2.add(100).cast::<i32>() = _rt::as_i32(nseconds8);
                                }
                                None => {
                                    *ptr2.add(8).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            *ptr2.add(104).cast::<i32>() = _rt::as_i32(linkmax3);
                            *ptr2.add(108).cast::<i32>() = _rt::as_i32(name_max3);
                            *ptr2.add(112).cast::<u8>() = (match no_trunc3 {
                                true => 1,
                                false => 0,
                            }) as u8;
                            *ptr2.add(113).cast::<u8>() = (match chown_restricted3 {
                                true => 1,
                                false => 0,
                            }) as u8;
                            *ptr2.add(114).cast::<u8>() = (match case_insensitive3 {
                                true => 1,
                                false => 0,
                            }) as u8;
                            *ptr2.add(115).cast::<u8>() = (match case_preserving3 {
                                true => 1,
                                false => 0,
                            }) as u8;
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code9,
                                message: message9,
                            } = e;
                            match nfs_error_code9 {
                                Some(e) => {
                                    *ptr2.add(8).cast::<u8>() = (1i32) as u8;
                                    *ptr2.add(12).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr2.add(8).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec10 = (message9.into_bytes()).into_boxed_slice();
                            let ptr10 = vec10.as_ptr().cast::<u8>();
                            let len10 = vec10.len();
                            ::core::mem::forget(vec10);
                            *ptr2.add(20).cast::<usize>() = len10;
                            *ptr2.add(16).cast::<*mut u8>() = ptr10.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_pathconf<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(16).cast::<*mut u8>();
                            let l2 = *arg0.add(20).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_pathconf_path_cabi<
                    T: GuestNfsMount,
                >(arg0: *mut u8, arg1: *mut u8, arg2: usize) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let result1 = T::pathconf_path(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            let PathConf {
                                attr: attr3,
                                linkmax: linkmax3,
                                name_max: name_max3,
                                no_trunc: no_trunc3,
                                chown_restricted: chown_restricted3,
                                case_insensitive: case_insensitive3,
                                case_preserving: case_preserving3,
                            } = e;
                            match attr3 {
                                Some(e) => {
                                    *ptr2.add(8).cast::<u8>() = (1i32) as u8;
                                    let Attr {
                                        attr_type: attr_type4,
                                        file_mode: file_mode4,
                                        nlink: nlink4,
                                        uid: uid4,
                                        gid: gid4,
                                        filesize: filesize4,
                                        used: used4,
                                        spec_data: spec_data4,
                                        fsid: fsid4,
                                        fileid: fileid4,
                                        atime: atime4,
                                        mtime: mtime4,
                                        ctime: ctime4,
                                    } = e;
                                    *ptr2.add(16).cast::<i32>() = _rt::as_i32(attr_type4);
                                    *ptr2.add(20).cast::<i32>() = _rt::as_i32(file_mode4);
                                    *ptr2.add(24).cast::<i32>() = _rt::as_i32(nlink4);
                                    *ptr2.add(28).cast::<i32>() = _rt::as_i32(uid4);
                                    *ptr2.add(32).cast::<i32>() = _rt::as_i32(gid4);
                                    *ptr2.add(40).cast::<i64>() = _rt::as_i64(filesize4);
                                    *ptr2.add(48).cast::<i64>() = _rt::as_i64(used4);
                                    let (t5_0, t5_1) = spec_data4;
                                    *ptr2.add(56).cast::<i32>() = _rt::as_i32(t5_0);
                                    *ptr2.add(60).cast::<i32>() = _rt::as_i32(t5_1);
                                    *ptr2.add(64).cast::<i64>() = _rt::as_i64(fsid4);
                                    *ptr2.add(72).cast::<i64>() = _rt::as_i64(fileid4);
                                    let Time { seconds: seconds6, nseconds: nseconds6 } = atime4;
                                    *ptr2.add(80).cast::<i32>() = _rt::as_i32(seconds6);
                                    *ptr2.add(84).cast::<i32>() = _rt::as_i32(nseconds6);
                                    let Time { seconds: seconds7, nseconds: nseconds7 } = mtime4;
                                    *ptr2.add(88).cast::<i32>() = _rt::as_i32(seconds7);
                                    *ptr2.add(92).cast::<i32>() = _rt::as_i32(nseconds7);
                                    let Time { seconds: seconds8, nseconds: nseconds8 } = ctime4;
                                    *ptr2.add(96).cast::<i32>() = _rt::as_i32(seconds8);
                                    *ptr2.add(100).cast::<i32>() = _rt::as_i32(nseconds8);
                                }
                                None => {
                                    *ptr2.add(8).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            *ptr2.add(104).cast::<i32>() = _rt::as_i32(linkmax3);
                            *ptr2.add(108).cast::<i32>() = _rt::as_i32(name_max3);
                            *ptr2.add(112).cast::<u8>() = (match no_trunc3 {
                                true => 1,
                                false => 0,
                            }) as u8;
                            *ptr2.add(113).cast::<u8>() = (match chown_restricted3 {
                                true => 1,
                                false => 0,
                            }) as u8;
                            *ptr2.add(114).cast::<u8>() = (match case_insensitive3 {
                                true => 1,
                                false => 0,
                            }) as u8;
                            *ptr2.add(115).cast::<u8>() = (match case_preserving3 {
                                true => 1,
                                false => 0,
                            }) as u8;
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code9,
                                message: message9,
                            } = e;
                            match nfs_error_code9 {
                                Some(e) => {
                                    *ptr2.add(8).cast::<u8>() = (1i32) as u8;
                                    *ptr2.add(12).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr2.add(8).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec10 = (message9.into_bytes()).into_boxed_slice();
                            let ptr10 = vec10.as_ptr().cast::<u8>();
                            let len10 = vec10.len();
                            ::core::mem::forget(vec10);
                            *ptr2.add(20).cast::<usize>() = len10;
                            *ptr2.add(16).cast::<*mut u8>() = ptr10.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_pathconf_path<
                    T: GuestNfsMount,
                >(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(16).cast::<*mut u8>();
                            let l2 = *arg0.add(20).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_read_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: i64,
                    arg4: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let result1 = T::read(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::Vec::from_raw_parts(arg1.cast(), len0, len0),
                        arg3 as u64,
                        arg4 as u32,
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            let vec3 = (e).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *ptr2.add(8).cast::<usize>() = len3;
                            *ptr2.add(4).cast::<*mut u8>() = ptr3.cast_mut();
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code4,
                                message: message4,
                            } = e;
                            match nfs_error_code4 {
                                Some(e) => {
                                    *ptr2.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr2.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr2.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec5 = (message4.into_bytes()).into_boxed_slice();
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            ::core::mem::forget(vec5);
                            *ptr2.add(16).cast::<usize>() = len5;
                            *ptr2.add(12).cast::<*mut u8>() = ptr5.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_read<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            let base3 = l1;
                            let len3 = l2;
                            _rt::cabi_dealloc(base3, len3 * 1, 1);
                        }
                        _ => {
                            let l4 = *arg0.add(12).cast::<*mut u8>();
                            let l5 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l4, l5, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_read_path_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: i64,
                    arg4: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let result1 = T::read_path(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                        arg3 as u64,
                        arg4 as u32,
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            let vec3 = (e).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *ptr2.add(8).cast::<usize>() = len3;
                            *ptr2.add(4).cast::<*mut u8>() = ptr3.cast_mut();
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code4,
                                message: message4,
                            } = e;
                            match nfs_error_code4 {
                                Some(e) => {
                                    *ptr2.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr2.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr2.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec5 = (message4.into_bytes()).into_boxed_slice();
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            ::core::mem::forget(vec5);
                            *ptr2.add(16).cast::<usize>() = len5;
                            *ptr2.add(12).cast::<*mut u8>() = ptr5.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_read_path<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            let base3 = l1;
                            let len3 = l2;
                            _rt::cabi_dealloc(base3, len3 * 1, 1);
                        }
                        _ => {
                            let l4 = *arg0.add(12).cast::<*mut u8>();
                            let l5 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l4, l5, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_write_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: i64,
                    arg4: *mut u8,
                    arg5: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let len1 = arg5;
                    let result2 = T::write(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::Vec::from_raw_parts(arg1.cast(), len0, len0),
                        arg3 as u64,
                        _rt::Vec::from_raw_parts(arg4.cast(), len1, len1),
                    );
                    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result2 {
                        Ok(e) => {
                            *ptr3.add(0).cast::<u8>() = (0i32) as u8;
                            *ptr3.add(4).cast::<i32>() = _rt::as_i32(e);
                        }
                        Err(e) => {
                            *ptr3.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code4,
                                message: message4,
                            } = e;
                            match nfs_error_code4 {
                                Some(e) => {
                                    *ptr3.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr3.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr3.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec5 = (message4.into_bytes()).into_boxed_slice();
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            ::core::mem::forget(vec5);
                            *ptr3.add(16).cast::<usize>() = len5;
                            *ptr3.add(12).cast::<*mut u8>() = ptr5.cast_mut();
                        }
                    };
                    ptr3
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_write<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(12).cast::<*mut u8>();
                            let l2 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_write_path_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: i64,
                    arg4: *mut u8,
                    arg5: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let len1 = arg5;
                    let result2 = T::write_path(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                        arg3 as u64,
                        _rt::Vec::from_raw_parts(arg4.cast(), len1, len1),
                    );
                    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result2 {
                        Ok(e) => {
                            *ptr3.add(0).cast::<u8>() = (0i32) as u8;
                            *ptr3.add(4).cast::<i32>() = _rt::as_i32(e);
                        }
                        Err(e) => {
                            *ptr3.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code4,
                                message: message4,
                            } = e;
                            match nfs_error_code4 {
                                Some(e) => {
                                    *ptr3.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr3.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr3.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec5 = (message4.into_bytes()).into_boxed_slice();
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            ::core::mem::forget(vec5);
                            *ptr3.add(16).cast::<usize>() = len5;
                            *ptr3.add(12).cast::<*mut u8>() = ptr5.cast_mut();
                        }
                    };
                    ptr3
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_write_path<
                    T: GuestNfsMount,
                >(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(12).cast::<*mut u8>();
                            let l2 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_readdir_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let result1 = T::readdir(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::Vec::from_raw_parts(arg1.cast(), len0, len0),
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            let vec5 = e;
                            let len5 = vec5.len();
                            let layout5 = _rt::alloc::Layout::from_size_align_unchecked(
                                vec5.len() * 16,
                                8,
                            );
                            let result5 = if layout5.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout5).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout5);
                                }
                                ptr
                            } else {
                                { ::core::ptr::null_mut() }
                            };
                            for (i, e) in vec5.into_iter().enumerate() {
                                let base = result5.add(i * 16);
                                {
                                    let ReaddirEntry {
                                        fileid: fileid3,
                                        file_name: file_name3,
                                    } = e;
                                    *base.add(0).cast::<i64>() = _rt::as_i64(fileid3);
                                    let vec4 = (file_name3.into_bytes()).into_boxed_slice();
                                    let ptr4 = vec4.as_ptr().cast::<u8>();
                                    let len4 = vec4.len();
                                    ::core::mem::forget(vec4);
                                    *base.add(12).cast::<usize>() = len4;
                                    *base.add(8).cast::<*mut u8>() = ptr4.cast_mut();
                                }
                            }
                            *ptr2.add(8).cast::<usize>() = len5;
                            *ptr2.add(4).cast::<*mut u8>() = result5;
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code6,
                                message: message6,
                            } = e;
                            match nfs_error_code6 {
                                Some(e) => {
                                    *ptr2.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr2.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr2.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec7 = (message6.into_bytes()).into_boxed_slice();
                            let ptr7 = vec7.as_ptr().cast::<u8>();
                            let len7 = vec7.len();
                            ::core::mem::forget(vec7);
                            *ptr2.add(16).cast::<usize>() = len7;
                            *ptr2.add(12).cast::<*mut u8>() = ptr7.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_readdir<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            let base5 = l1;
                            let len5 = l2;
                            for i in 0..len5 {
                                let base = base5.add(i * 16);
                                {
                                    let l3 = *base.add(8).cast::<*mut u8>();
                                    let l4 = *base.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l3, l4, 1);
                                }
                            }
                            _rt::cabi_dealloc(base5, len5 * 16, 8);
                        }
                        _ => {
                            let l6 = *arg0.add(12).cast::<*mut u8>();
                            let l7 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l6, l7, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_readdir_path_cabi<
                    T: GuestNfsMount,
                >(arg0: *mut u8, arg1: *mut u8, arg2: usize) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let result1 = T::readdir_path(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            let vec5 = e;
                            let len5 = vec5.len();
                            let layout5 = _rt::alloc::Layout::from_size_align_unchecked(
                                vec5.len() * 16,
                                8,
                            );
                            let result5 = if layout5.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout5).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout5);
                                }
                                ptr
                            } else {
                                { ::core::ptr::null_mut() }
                            };
                            for (i, e) in vec5.into_iter().enumerate() {
                                let base = result5.add(i * 16);
                                {
                                    let ReaddirEntry {
                                        fileid: fileid3,
                                        file_name: file_name3,
                                    } = e;
                                    *base.add(0).cast::<i64>() = _rt::as_i64(fileid3);
                                    let vec4 = (file_name3.into_bytes()).into_boxed_slice();
                                    let ptr4 = vec4.as_ptr().cast::<u8>();
                                    let len4 = vec4.len();
                                    ::core::mem::forget(vec4);
                                    *base.add(12).cast::<usize>() = len4;
                                    *base.add(8).cast::<*mut u8>() = ptr4.cast_mut();
                                }
                            }
                            *ptr2.add(8).cast::<usize>() = len5;
                            *ptr2.add(4).cast::<*mut u8>() = result5;
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code6,
                                message: message6,
                            } = e;
                            match nfs_error_code6 {
                                Some(e) => {
                                    *ptr2.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr2.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr2.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec7 = (message6.into_bytes()).into_boxed_slice();
                            let ptr7 = vec7.as_ptr().cast::<u8>();
                            let len7 = vec7.len();
                            ::core::mem::forget(vec7);
                            *ptr2.add(16).cast::<usize>() = len7;
                            *ptr2.add(12).cast::<*mut u8>() = ptr7.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_readdir_path<
                    T: GuestNfsMount,
                >(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            let base5 = l1;
                            let len5 = l2;
                            for i in 0..len5 {
                                let base = base5.add(i * 16);
                                {
                                    let l3 = *base.add(8).cast::<*mut u8>();
                                    let l4 = *base.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l3, l4, 1);
                                }
                            }
                            _rt::cabi_dealloc(base5, len5 * 16, 8);
                        }
                        _ => {
                            let l6 = *arg0.add(12).cast::<*mut u8>();
                            let l7 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l6, l7, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_readdirplus_cabi<
                    T: GuestNfsMount,
                >(arg0: *mut u8, arg1: *mut u8, arg2: usize) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let result1 = T::readdirplus(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::Vec::from_raw_parts(arg1.cast(), len0, len0),
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            let vec11 = e;
                            let len11 = vec11.len();
                            let layout11 = _rt::alloc::Layout::from_size_align_unchecked(
                                vec11.len() * 120,
                                8,
                            );
                            let result11 = if layout11.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout11).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout11);
                                }
                                ptr
                            } else {
                                { ::core::ptr::null_mut() }
                            };
                            for (i, e) in vec11.into_iter().enumerate() {
                                let base = result11.add(i * 120);
                                {
                                    let ReaddirplusEntry {
                                        fileid: fileid3,
                                        file_name: file_name3,
                                        attr: attr3,
                                        handle: handle3,
                                    } = e;
                                    *base.add(0).cast::<i64>() = _rt::as_i64(fileid3);
                                    let vec4 = (file_name3.into_bytes()).into_boxed_slice();
                                    let ptr4 = vec4.as_ptr().cast::<u8>();
                                    let len4 = vec4.len();
                                    ::core::mem::forget(vec4);
                                    *base.add(12).cast::<usize>() = len4;
                                    *base.add(8).cast::<*mut u8>() = ptr4.cast_mut();
                                    match attr3 {
                                        Some(e) => {
                                            *base.add(16).cast::<u8>() = (1i32) as u8;
                                            let Attr {
                                                attr_type: attr_type5,
                                                file_mode: file_mode5,
                                                nlink: nlink5,
                                                uid: uid5,
                                                gid: gid5,
                                                filesize: filesize5,
                                                used: used5,
                                                spec_data: spec_data5,
                                                fsid: fsid5,
                                                fileid: fileid5,
                                                atime: atime5,
                                                mtime: mtime5,
                                                ctime: ctime5,
                                            } = e;
                                            *base.add(24).cast::<i32>() = _rt::as_i32(attr_type5);
                                            *base.add(28).cast::<i32>() = _rt::as_i32(file_mode5);
                                            *base.add(32).cast::<i32>() = _rt::as_i32(nlink5);
                                            *base.add(36).cast::<i32>() = _rt::as_i32(uid5);
                                            *base.add(40).cast::<i32>() = _rt::as_i32(gid5);
                                            *base.add(48).cast::<i64>() = _rt::as_i64(filesize5);
                                            *base.add(56).cast::<i64>() = _rt::as_i64(used5);
                                            let (t6_0, t6_1) = spec_data5;
                                            *base.add(64).cast::<i32>() = _rt::as_i32(t6_0);
                                            *base.add(68).cast::<i32>() = _rt::as_i32(t6_1);
                                            *base.add(72).cast::<i64>() = _rt::as_i64(fsid5);
                                            *base.add(80).cast::<i64>() = _rt::as_i64(fileid5);
                                            let Time { seconds: seconds7, nseconds: nseconds7 } = atime5;
                                            *base.add(88).cast::<i32>() = _rt::as_i32(seconds7);
                                            *base.add(92).cast::<i32>() = _rt::as_i32(nseconds7);
                                            let Time { seconds: seconds8, nseconds: nseconds8 } = mtime5;
                                            *base.add(96).cast::<i32>() = _rt::as_i32(seconds8);
                                            *base.add(100).cast::<i32>() = _rt::as_i32(nseconds8);
                                            let Time { seconds: seconds9, nseconds: nseconds9 } = ctime5;
                                            *base.add(104).cast::<i32>() = _rt::as_i32(seconds9);
                                            *base.add(108).cast::<i32>() = _rt::as_i32(nseconds9);
                                        }
                                        None => {
                                            *base.add(16).cast::<u8>() = (0i32) as u8;
                                        }
                                    };
                                    let vec10 = (handle3).into_boxed_slice();
                                    let ptr10 = vec10.as_ptr().cast::<u8>();
                                    let len10 = vec10.len();
                                    ::core::mem::forget(vec10);
                                    *base.add(116).cast::<usize>() = len10;
                                    *base.add(112).cast::<*mut u8>() = ptr10.cast_mut();
                                }
                            }
                            *ptr2.add(8).cast::<usize>() = len11;
                            *ptr2.add(4).cast::<*mut u8>() = result11;
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code12,
                                message: message12,
                            } = e;
                            match nfs_error_code12 {
                                Some(e) => {
                                    *ptr2.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr2.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr2.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec13 = (message12.into_bytes()).into_boxed_slice();
                            let ptr13 = vec13.as_ptr().cast::<u8>();
                            let len13 = vec13.len();
                            ::core::mem::forget(vec13);
                            *ptr2.add(16).cast::<usize>() = len13;
                            *ptr2.add(12).cast::<*mut u8>() = ptr13.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_readdirplus<
                    T: GuestNfsMount,
                >(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            let base8 = l1;
                            let len8 = l2;
                            for i in 0..len8 {
                                let base = base8.add(i * 120);
                                {
                                    let l3 = *base.add(8).cast::<*mut u8>();
                                    let l4 = *base.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l3, l4, 1);
                                    let l5 = *base.add(112).cast::<*mut u8>();
                                    let l6 = *base.add(116).cast::<usize>();
                                    let base7 = l5;
                                    let len7 = l6;
                                    _rt::cabi_dealloc(base7, len7 * 1, 1);
                                }
                            }
                            _rt::cabi_dealloc(base8, len8 * 120, 8);
                        }
                        _ => {
                            let l9 = *arg0.add(12).cast::<*mut u8>();
                            let l10 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l9, l10, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_readdirplus_path_cabi<
                    T: GuestNfsMount,
                >(arg0: *mut u8, arg1: *mut u8, arg2: usize) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let result1 = T::readdirplus_path(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            let vec11 = e;
                            let len11 = vec11.len();
                            let layout11 = _rt::alloc::Layout::from_size_align_unchecked(
                                vec11.len() * 120,
                                8,
                            );
                            let result11 = if layout11.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout11).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout11);
                                }
                                ptr
                            } else {
                                { ::core::ptr::null_mut() }
                            };
                            for (i, e) in vec11.into_iter().enumerate() {
                                let base = result11.add(i * 120);
                                {
                                    let ReaddirplusEntry {
                                        fileid: fileid3,
                                        file_name: file_name3,
                                        attr: attr3,
                                        handle: handle3,
                                    } = e;
                                    *base.add(0).cast::<i64>() = _rt::as_i64(fileid3);
                                    let vec4 = (file_name3.into_bytes()).into_boxed_slice();
                                    let ptr4 = vec4.as_ptr().cast::<u8>();
                                    let len4 = vec4.len();
                                    ::core::mem::forget(vec4);
                                    *base.add(12).cast::<usize>() = len4;
                                    *base.add(8).cast::<*mut u8>() = ptr4.cast_mut();
                                    match attr3 {
                                        Some(e) => {
                                            *base.add(16).cast::<u8>() = (1i32) as u8;
                                            let Attr {
                                                attr_type: attr_type5,
                                                file_mode: file_mode5,
                                                nlink: nlink5,
                                                uid: uid5,
                                                gid: gid5,
                                                filesize: filesize5,
                                                used: used5,
                                                spec_data: spec_data5,
                                                fsid: fsid5,
                                                fileid: fileid5,
                                                atime: atime5,
                                                mtime: mtime5,
                                                ctime: ctime5,
                                            } = e;
                                            *base.add(24).cast::<i32>() = _rt::as_i32(attr_type5);
                                            *base.add(28).cast::<i32>() = _rt::as_i32(file_mode5);
                                            *base.add(32).cast::<i32>() = _rt::as_i32(nlink5);
                                            *base.add(36).cast::<i32>() = _rt::as_i32(uid5);
                                            *base.add(40).cast::<i32>() = _rt::as_i32(gid5);
                                            *base.add(48).cast::<i64>() = _rt::as_i64(filesize5);
                                            *base.add(56).cast::<i64>() = _rt::as_i64(used5);
                                            let (t6_0, t6_1) = spec_data5;
                                            *base.add(64).cast::<i32>() = _rt::as_i32(t6_0);
                                            *base.add(68).cast::<i32>() = _rt::as_i32(t6_1);
                                            *base.add(72).cast::<i64>() = _rt::as_i64(fsid5);
                                            *base.add(80).cast::<i64>() = _rt::as_i64(fileid5);
                                            let Time { seconds: seconds7, nseconds: nseconds7 } = atime5;
                                            *base.add(88).cast::<i32>() = _rt::as_i32(seconds7);
                                            *base.add(92).cast::<i32>() = _rt::as_i32(nseconds7);
                                            let Time { seconds: seconds8, nseconds: nseconds8 } = mtime5;
                                            *base.add(96).cast::<i32>() = _rt::as_i32(seconds8);
                                            *base.add(100).cast::<i32>() = _rt::as_i32(nseconds8);
                                            let Time { seconds: seconds9, nseconds: nseconds9 } = ctime5;
                                            *base.add(104).cast::<i32>() = _rt::as_i32(seconds9);
                                            *base.add(108).cast::<i32>() = _rt::as_i32(nseconds9);
                                        }
                                        None => {
                                            *base.add(16).cast::<u8>() = (0i32) as u8;
                                        }
                                    };
                                    let vec10 = (handle3).into_boxed_slice();
                                    let ptr10 = vec10.as_ptr().cast::<u8>();
                                    let len10 = vec10.len();
                                    ::core::mem::forget(vec10);
                                    *base.add(116).cast::<usize>() = len10;
                                    *base.add(112).cast::<*mut u8>() = ptr10.cast_mut();
                                }
                            }
                            *ptr2.add(8).cast::<usize>() = len11;
                            *ptr2.add(4).cast::<*mut u8>() = result11;
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code12,
                                message: message12,
                            } = e;
                            match nfs_error_code12 {
                                Some(e) => {
                                    *ptr2.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr2.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr2.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec13 = (message12.into_bytes()).into_boxed_slice();
                            let ptr13 = vec13.as_ptr().cast::<u8>();
                            let len13 = vec13.len();
                            ::core::mem::forget(vec13);
                            *ptr2.add(16).cast::<usize>() = len13;
                            *ptr2.add(12).cast::<*mut u8>() = ptr13.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_readdirplus_path<
                    T: GuestNfsMount,
                >(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            let base8 = l1;
                            let len8 = l2;
                            for i in 0..len8 {
                                let base = base8.add(i * 120);
                                {
                                    let l3 = *base.add(8).cast::<*mut u8>();
                                    let l4 = *base.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l3, l4, 1);
                                    let l5 = *base.add(112).cast::<*mut u8>();
                                    let l6 = *base.add(116).cast::<usize>();
                                    let base7 = l5;
                                    let len7 = l6;
                                    _rt::cabi_dealloc(base7, len7 * 1, 1);
                                }
                            }
                            _rt::cabi_dealloc(base8, len8 * 120, 8);
                        }
                        _ => {
                            let l9 = *arg0.add(12).cast::<*mut u8>();
                            let l10 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l9, l10, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_mkdir_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: *mut u8,
                    arg4: usize,
                    arg5: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let len1 = arg4;
                    let bytes1 = _rt::Vec::from_raw_parts(arg3.cast(), len1, len1);
                    let result2 = T::mkdir(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::Vec::from_raw_parts(arg1.cast(), len0, len0),
                        _rt::string_lift(bytes1),
                        arg5 as u32,
                    );
                    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result2 {
                        Ok(e) => {
                            *ptr3.add(0).cast::<u8>() = (0i32) as u8;
                            let ObjRes { obj: obj4, attr: attr4 } = e;
                            let vec5 = (obj4).into_boxed_slice();
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            ::core::mem::forget(vec5);
                            *ptr3.add(12).cast::<usize>() = len5;
                            *ptr3.add(8).cast::<*mut u8>() = ptr5.cast_mut();
                            match attr4 {
                                Some(e) => {
                                    *ptr3.add(16).cast::<u8>() = (1i32) as u8;
                                    let Attr {
                                        attr_type: attr_type6,
                                        file_mode: file_mode6,
                                        nlink: nlink6,
                                        uid: uid6,
                                        gid: gid6,
                                        filesize: filesize6,
                                        used: used6,
                                        spec_data: spec_data6,
                                        fsid: fsid6,
                                        fileid: fileid6,
                                        atime: atime6,
                                        mtime: mtime6,
                                        ctime: ctime6,
                                    } = e;
                                    *ptr3.add(24).cast::<i32>() = _rt::as_i32(attr_type6);
                                    *ptr3.add(28).cast::<i32>() = _rt::as_i32(file_mode6);
                                    *ptr3.add(32).cast::<i32>() = _rt::as_i32(nlink6);
                                    *ptr3.add(36).cast::<i32>() = _rt::as_i32(uid6);
                                    *ptr3.add(40).cast::<i32>() = _rt::as_i32(gid6);
                                    *ptr3.add(48).cast::<i64>() = _rt::as_i64(filesize6);
                                    *ptr3.add(56).cast::<i64>() = _rt::as_i64(used6);
                                    let (t7_0, t7_1) = spec_data6;
                                    *ptr3.add(64).cast::<i32>() = _rt::as_i32(t7_0);
                                    *ptr3.add(68).cast::<i32>() = _rt::as_i32(t7_1);
                                    *ptr3.add(72).cast::<i64>() = _rt::as_i64(fsid6);
                                    *ptr3.add(80).cast::<i64>() = _rt::as_i64(fileid6);
                                    let Time { seconds: seconds8, nseconds: nseconds8 } = atime6;
                                    *ptr3.add(88).cast::<i32>() = _rt::as_i32(seconds8);
                                    *ptr3.add(92).cast::<i32>() = _rt::as_i32(nseconds8);
                                    let Time { seconds: seconds9, nseconds: nseconds9 } = mtime6;
                                    *ptr3.add(96).cast::<i32>() = _rt::as_i32(seconds9);
                                    *ptr3.add(100).cast::<i32>() = _rt::as_i32(nseconds9);
                                    let Time { seconds: seconds10, nseconds: nseconds10 } = ctime6;
                                    *ptr3.add(104).cast::<i32>() = _rt::as_i32(seconds10);
                                    *ptr3.add(108).cast::<i32>() = _rt::as_i32(nseconds10);
                                }
                                None => {
                                    *ptr3.add(16).cast::<u8>() = (0i32) as u8;
                                }
                            };
                        }
                        Err(e) => {
                            *ptr3.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code11,
                                message: message11,
                            } = e;
                            match nfs_error_code11 {
                                Some(e) => {
                                    *ptr3.add(8).cast::<u8>() = (1i32) as u8;
                                    *ptr3.add(12).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr3.add(8).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec12 = (message11.into_bytes()).into_boxed_slice();
                            let ptr12 = vec12.as_ptr().cast::<u8>();
                            let len12 = vec12.len();
                            ::core::mem::forget(vec12);
                            *ptr3.add(20).cast::<usize>() = len12;
                            *ptr3.add(16).cast::<*mut u8>() = ptr12.cast_mut();
                        }
                    };
                    ptr3
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_mkdir<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(8).cast::<*mut u8>();
                            let l2 = *arg0.add(12).cast::<usize>();
                            let base3 = l1;
                            let len3 = l2;
                            _rt::cabi_dealloc(base3, len3 * 1, 1);
                        }
                        _ => {
                            let l4 = *arg0.add(16).cast::<*mut u8>();
                            let l5 = *arg0.add(20).cast::<usize>();
                            _rt::cabi_dealloc(l4, l5, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_mkdir_path_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let result1 = T::mkdir_path(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                        arg3 as u32,
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            let ObjRes { obj: obj3, attr: attr3 } = e;
                            let vec4 = (obj3).into_boxed_slice();
                            let ptr4 = vec4.as_ptr().cast::<u8>();
                            let len4 = vec4.len();
                            ::core::mem::forget(vec4);
                            *ptr2.add(12).cast::<usize>() = len4;
                            *ptr2.add(8).cast::<*mut u8>() = ptr4.cast_mut();
                            match attr3 {
                                Some(e) => {
                                    *ptr2.add(16).cast::<u8>() = (1i32) as u8;
                                    let Attr {
                                        attr_type: attr_type5,
                                        file_mode: file_mode5,
                                        nlink: nlink5,
                                        uid: uid5,
                                        gid: gid5,
                                        filesize: filesize5,
                                        used: used5,
                                        spec_data: spec_data5,
                                        fsid: fsid5,
                                        fileid: fileid5,
                                        atime: atime5,
                                        mtime: mtime5,
                                        ctime: ctime5,
                                    } = e;
                                    *ptr2.add(24).cast::<i32>() = _rt::as_i32(attr_type5);
                                    *ptr2.add(28).cast::<i32>() = _rt::as_i32(file_mode5);
                                    *ptr2.add(32).cast::<i32>() = _rt::as_i32(nlink5);
                                    *ptr2.add(36).cast::<i32>() = _rt::as_i32(uid5);
                                    *ptr2.add(40).cast::<i32>() = _rt::as_i32(gid5);
                                    *ptr2.add(48).cast::<i64>() = _rt::as_i64(filesize5);
                                    *ptr2.add(56).cast::<i64>() = _rt::as_i64(used5);
                                    let (t6_0, t6_1) = spec_data5;
                                    *ptr2.add(64).cast::<i32>() = _rt::as_i32(t6_0);
                                    *ptr2.add(68).cast::<i32>() = _rt::as_i32(t6_1);
                                    *ptr2.add(72).cast::<i64>() = _rt::as_i64(fsid5);
                                    *ptr2.add(80).cast::<i64>() = _rt::as_i64(fileid5);
                                    let Time { seconds: seconds7, nseconds: nseconds7 } = atime5;
                                    *ptr2.add(88).cast::<i32>() = _rt::as_i32(seconds7);
                                    *ptr2.add(92).cast::<i32>() = _rt::as_i32(nseconds7);
                                    let Time { seconds: seconds8, nseconds: nseconds8 } = mtime5;
                                    *ptr2.add(96).cast::<i32>() = _rt::as_i32(seconds8);
                                    *ptr2.add(100).cast::<i32>() = _rt::as_i32(nseconds8);
                                    let Time { seconds: seconds9, nseconds: nseconds9 } = ctime5;
                                    *ptr2.add(104).cast::<i32>() = _rt::as_i32(seconds9);
                                    *ptr2.add(108).cast::<i32>() = _rt::as_i32(nseconds9);
                                }
                                None => {
                                    *ptr2.add(16).cast::<u8>() = (0i32) as u8;
                                }
                            };
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code10,
                                message: message10,
                            } = e;
                            match nfs_error_code10 {
                                Some(e) => {
                                    *ptr2.add(8).cast::<u8>() = (1i32) as u8;
                                    *ptr2.add(12).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr2.add(8).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec11 = (message10.into_bytes()).into_boxed_slice();
                            let ptr11 = vec11.as_ptr().cast::<u8>();
                            let len11 = vec11.len();
                            ::core::mem::forget(vec11);
                            *ptr2.add(20).cast::<usize>() = len11;
                            *ptr2.add(16).cast::<*mut u8>() = ptr11.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_mkdir_path<
                    T: GuestNfsMount,
                >(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(8).cast::<*mut u8>();
                            let l2 = *arg0.add(12).cast::<usize>();
                            let base3 = l1;
                            let len3 = l2;
                            _rt::cabi_dealloc(base3, len3 * 1, 1);
                        }
                        _ => {
                            let l4 = *arg0.add(16).cast::<*mut u8>();
                            let l5 = *arg0.add(20).cast::<usize>();
                            _rt::cabi_dealloc(l4, l5, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_remove_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: *mut u8,
                    arg4: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let len1 = arg4;
                    let bytes1 = _rt::Vec::from_raw_parts(arg3.cast(), len1, len1);
                    let result2 = T::remove(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::Vec::from_raw_parts(arg1.cast(), len0, len0),
                        _rt::string_lift(bytes1),
                    );
                    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result2 {
                        Ok(_) => {
                            *ptr3.add(0).cast::<u8>() = (0i32) as u8;
                        }
                        Err(e) => {
                            *ptr3.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code4,
                                message: message4,
                            } = e;
                            match nfs_error_code4 {
                                Some(e) => {
                                    *ptr3.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr3.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr3.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec5 = (message4.into_bytes()).into_boxed_slice();
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            ::core::mem::forget(vec5);
                            *ptr3.add(16).cast::<usize>() = len5;
                            *ptr3.add(12).cast::<*mut u8>() = ptr5.cast_mut();
                        }
                    };
                    ptr3
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_remove<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(12).cast::<*mut u8>();
                            let l2 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_remove_path_cabi<
                    T: GuestNfsMount,
                >(arg0: *mut u8, arg1: *mut u8, arg2: usize) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let result1 = T::remove_path(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(_) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code3,
                                message: message3,
                            } = e;
                            match nfs_error_code3 {
                                Some(e) => {
                                    *ptr2.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr2.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr2.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec4 = (message3.into_bytes()).into_boxed_slice();
                            let ptr4 = vec4.as_ptr().cast::<u8>();
                            let len4 = vec4.len();
                            ::core::mem::forget(vec4);
                            *ptr2.add(16).cast::<usize>() = len4;
                            *ptr2.add(12).cast::<*mut u8>() = ptr4.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_remove_path<
                    T: GuestNfsMount,
                >(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(12).cast::<*mut u8>();
                            let l2 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_rmdir_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: *mut u8,
                    arg4: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let len1 = arg4;
                    let bytes1 = _rt::Vec::from_raw_parts(arg3.cast(), len1, len1);
                    let result2 = T::rmdir(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::Vec::from_raw_parts(arg1.cast(), len0, len0),
                        _rt::string_lift(bytes1),
                    );
                    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result2 {
                        Ok(_) => {
                            *ptr3.add(0).cast::<u8>() = (0i32) as u8;
                        }
                        Err(e) => {
                            *ptr3.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code4,
                                message: message4,
                            } = e;
                            match nfs_error_code4 {
                                Some(e) => {
                                    *ptr3.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr3.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr3.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec5 = (message4.into_bytes()).into_boxed_slice();
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            ::core::mem::forget(vec5);
                            *ptr3.add(16).cast::<usize>() = len5;
                            *ptr3.add(12).cast::<*mut u8>() = ptr5.cast_mut();
                        }
                    };
                    ptr3
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_rmdir<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(12).cast::<*mut u8>();
                            let l2 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_rmdir_path_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let result1 = T::rmdir_path(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(_) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code3,
                                message: message3,
                            } = e;
                            match nfs_error_code3 {
                                Some(e) => {
                                    *ptr2.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr2.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr2.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec4 = (message3.into_bytes()).into_boxed_slice();
                            let ptr4 = vec4.as_ptr().cast::<u8>();
                            let len4 = vec4.len();
                            ::core::mem::forget(vec4);
                            *ptr2.add(16).cast::<usize>() = len4;
                            *ptr2.add(12).cast::<*mut u8>() = ptr4.cast_mut();
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_rmdir_path<
                    T: GuestNfsMount,
                >(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(12).cast::<*mut u8>();
                            let l2 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_rename_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: *mut u8,
                    arg4: usize,
                    arg5: *mut u8,
                    arg6: usize,
                    arg7: *mut u8,
                    arg8: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let len1 = arg4;
                    let bytes1 = _rt::Vec::from_raw_parts(arg3.cast(), len1, len1);
                    let len2 = arg6;
                    let len3 = arg8;
                    let bytes3 = _rt::Vec::from_raw_parts(arg7.cast(), len3, len3);
                    let result4 = T::rename(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::Vec::from_raw_parts(arg1.cast(), len0, len0),
                        _rt::string_lift(bytes1),
                        _rt::Vec::from_raw_parts(arg5.cast(), len2, len2),
                        _rt::string_lift(bytes3),
                    );
                    let ptr5 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result4 {
                        Ok(_) => {
                            *ptr5.add(0).cast::<u8>() = (0i32) as u8;
                        }
                        Err(e) => {
                            *ptr5.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code6,
                                message: message6,
                            } = e;
                            match nfs_error_code6 {
                                Some(e) => {
                                    *ptr5.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr5.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr5.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec7 = (message6.into_bytes()).into_boxed_slice();
                            let ptr7 = vec7.as_ptr().cast::<u8>();
                            let len7 = vec7.len();
                            ::core::mem::forget(vec7);
                            *ptr5.add(16).cast::<usize>() = len7;
                            *ptr5.add(12).cast::<*mut u8>() = ptr7.cast_mut();
                        }
                    };
                    ptr5
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_rename<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(12).cast::<*mut u8>();
                            let l2 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_rename_path_cabi<
                    T: GuestNfsMount,
                >(
                    arg0: *mut u8,
                    arg1: *mut u8,
                    arg2: usize,
                    arg3: *mut u8,
                    arg4: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg2;
                    let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);
                    let len1 = arg4;
                    let bytes1 = _rt::Vec::from_raw_parts(arg3.cast(), len1, len1);
                    let result2 = T::rename_path(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                        _rt::string_lift(bytes0),
                        _rt::string_lift(bytes1),
                    );
                    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result2 {
                        Ok(_) => {
                            *ptr3.add(0).cast::<u8>() = (0i32) as u8;
                        }
                        Err(e) => {
                            *ptr3.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code4,
                                message: message4,
                            } = e;
                            match nfs_error_code4 {
                                Some(e) => {
                                    *ptr3.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr3.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr3.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec5 = (message4.into_bytes()).into_boxed_slice();
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            ::core::mem::forget(vec5);
                            *ptr3.add(16).cast::<usize>() = len5;
                            *ptr3.add(12).cast::<*mut u8>() = ptr5.cast_mut();
                        }
                    };
                    ptr3
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_rename_path<
                    T: GuestNfsMount,
                >(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(12).cast::<*mut u8>();
                            let l2 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_umount_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::umount(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Ok(_) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                        }
                        Err(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code2,
                                message: message2,
                            } = e;
                            match nfs_error_code2 {
                                Some(e) => {
                                    *ptr1.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr1.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr1.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec3 = (message2.into_bytes()).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *ptr1.add(16).cast::<usize>() = len3;
                            *ptr1.add(12).cast::<*mut u8>() = ptr3.cast_mut();
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_umount<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(12).cast::<*mut u8>();
                            let l2 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_method_nfs_mount_version_cabi<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::version(
                        NfsMountBorrow::lift(arg0 as u32 as usize).get(),
                    );
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        Ok(e) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                            *ptr1.add(4).cast::<u8>() = (e.clone() as i32) as u8;
                        }
                        Err(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let Error {
                                nfs_error_code: nfs_error_code2,
                                message: message2,
                            } = e;
                            match nfs_error_code2 {
                                Some(e) => {
                                    *ptr1.add(4).cast::<u8>() = (1i32) as u8;
                                    *ptr1.add(8).cast::<i32>() = _rt::as_i32(e);
                                }
                                None => {
                                    *ptr1.add(4).cast::<u8>() = (0i32) as u8;
                                }
                            };
                            let vec3 = (message2.into_bytes()).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *ptr1.add(16).cast::<usize>() = len3;
                            *ptr1.add(12).cast::<*mut u8>() = ptr3.cast_mut();
                        }
                    };
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_method_nfs_mount_version<T: GuestNfsMount>(
                    arg0: *mut u8,
                ) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {}
                        _ => {
                            let l1 = *arg0.add(12).cast::<*mut u8>();
                            let l2 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                pub trait Guest {
                    type NfsMount: GuestNfsMount;
                    /// Parse URL and mount
                    ///
                    /// Parses the specified URL and attempts to mount the relevant NFS export.
                    /// Example usage in the following pseud-code:
                    ///
                    /// ```text
                    /// // query param version=4.1,4,3 in URL should try each version in turn until mount procedure is successful
                    /// // see https://github.com/NetAppLabs/nfs-rs#url-format for supported URL format
                    /// let mount = nfs_rs.parse_url_and_mount("nfs://localhost/some/export?version=4.1,4,3");
                    /// let version = mount.version(); // check which NFS version we succeeded in mounting
                    /// mount.umount();
                    /// ```
                    fn parse_url_and_mount(url: _rt::String) -> Result<NfsMount, Error>;
                }
                pub trait GuestNfsMount: 'static {
                    #[doc(hidden)]
                    unsafe fn _resource_new(val: *mut u8) -> u32
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = val;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]component:nfs-rs/nfs")]
                            extern "C" {
                                #[link_name = "[resource-new]nfs-mount"]
                                fn new(_: *mut u8) -> u32;
                            }
                            new(val)
                        }
                    }
                    #[doc(hidden)]
                    fn _resource_rep(handle: u32) -> *mut u8
                    where
                        Self: Sized,
                    {
                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let _ = handle;
                            unreachable!();
                        }
                        #[cfg(target_arch = "wasm32")]
                        {
                            #[link(wasm_import_module = "[export]component:nfs-rs/nfs")]
                            extern "C" {
                                #[link_name = "[resource-rep]nfs-mount"]
                                fn rep(_: u32) -> *mut u8;
                            }
                            unsafe { rep(handle) }
                        }
                    }
                    /// NULL procedure
                    ///
                    /// Procedure NULL does not do any work. It is made available to allow server response testing and timing.
                    /// Example usage in the following pseud-code:
                    ///
                    /// ```text
                    /// let mount = nfs_rs.parse_url_and_mount("nfs://localhost/some/export");
                    /// let version = mount.null(); // check which NFS version we succeeded in mounting
                    /// mount.umount();
                    /// ```
                    fn null_op(&self) -> Result<(), Error>;
                    /// ACCESS procedure
                    ///
                    /// Procedure ACCESS determines the access rights that a user, as identified by the credentials in the request,
                    /// has with respect to a file system object.
                    fn access(&self, fh: Fh, mode: u32) -> Result<u32, Error>;
                    /// ACCESS procedure for a path
                    ///
                    /// Same as `nfs-mount::access` but instead of taking in a file handle, takes in a path for which file handle
                    /// is obtained by performing one or move LOOKUP procedures.
                    fn access_path(
                        &self,
                        path: _rt::String,
                        mode: u32,
                    ) -> Result<u32, Error>;
                    /// CLOSE procedure
                    ///
                    /// Procedure CLOSE releases share reservations for the regular or named attribute file as specified by the
                    /// current filehandle.
                    fn close(&self, seqid: u32, stateid: u64) -> Result<(), Error>;
                    /// COMMIT procedure
                    ///
                    /// Procedure COMMIT forces or flushes data to stable storage that was previously written with a WRITE
                    /// procedure call with the stable field set to UNSTABLE.
                    fn commit(
                        &self,
                        fh: Fh,
                        offset: u64,
                        count: u32,
                    ) -> Result<(), Error>;
                    /// COMMIT procedure for a path
                    ///
                    /// Same as `nfs-mount::commit` but instead of taking in a file handle, takes in a path for which file handle
                    /// is obtained by performing one or more LOOKUP procedures.
                    fn commit_path(
                        &self,
                        path: _rt::String,
                        offset: u64,
                        count: u32,
                    ) -> Result<(), Error>;
                    /// CREATE procedure
                    ///
                    /// Procedure CREATE creates a regular file.
                    fn create(
                        &self,
                        dir_fh: Fh,
                        filename: _rt::String,
                        mode: u32,
                    ) -> Result<ObjRes, Error>;
                    /// CREATE procedure for a path
                    ///
                    /// Same as `nfs-mount::create` but instead of taking in directory file handle and filename, takes in a path
                    /// for which directory file handle is obtained by performing one or more LOOKUP procedures.
                    fn create_path(
                        &self,
                        path: _rt::String,
                        mode: u32,
                    ) -> Result<ObjRes, Error>;
                    /// DELEGPURGE procedure
                    ///
                    /// Procedure DELEGPURGE purges all of the delegations awaiting recovery for a given client.
                    fn delegpurge(&self, clientid: u64) -> Result<(), Error>;
                    /// DELEGRETURN procedure
                    ///
                    /// Procedure DELEGRETURN returns the delegation represented by the current filehandle and stateid.
                    fn delegreturn(&self, stateid: u64) -> Result<(), Error>;
                    /// GETATTR procedure
                    ///
                    /// Procedure GETATTR retrieves the attributes for a specified file system object.
                    fn getattr(&self, fh: Fh) -> Result<Attr, Error>;
                    /// GETATTR procedure for a path
                    ///
                    /// Same as `nfs-mount::getattr` but instead of taking in a file handle, takes in a path for which file handle
                    /// is obtained by performing one or more LOOKUP procedures.
                    fn getattr_path(&self, path: _rt::String) -> Result<Attr, Error>;
                    /// SETATTR procedure
                    ///
                    /// Procedure SETATTR changes one or more of the attributes of a file system object on the server.
                    fn setattr(
                        &self,
                        fh: Fh,
                        guard_ctime: Option<Time>,
                        mode: Option<u32>,
                        uid: Option<u32>,
                        gid: Option<u32>,
                        size: Option<u64>,
                        atime: Option<Time>,
                        mtime: Option<Time>,
                    ) -> Result<(), Error>;
                    /// SETATTR procedure for a path
                    ///
                    /// Same as `nfs-mount::setattr` but instead of taking in a file handle, takes in a path for which file handle
                    /// is obtained by performing one or more LOOKUP procedures.  Also, instead of taking in optional guard ctime,
                    /// takes in a boolean which determines whether to specify guard in SETATTR procedure or not, using ctime from
                    /// LOOKUP.
                    fn setattr_path(
                        &self,
                        path: _rt::String,
                        specify_guard: bool,
                        mode: Option<u32>,
                        uid: Option<u32>,
                        gid: Option<u32>,
                        size: Option<u64>,
                        atime: Option<Time>,
                        mtime: Option<Time>,
                    ) -> Result<(), Error>;
                    /// GETFH procedure
                    ///
                    /// Procedure GETFH returns the current filehandle value.
                    fn getfh(&self) -> Result<(), Error>;
                    /// LINK procedure
                    ///
                    /// Procedure LINK creates a hard link.
                    fn link(
                        &self,
                        src_fh: Fh,
                        dst_dir_fh: Fh,
                        dst_filename: _rt::String,
                    ) -> Result<Attr, Error>;
                    /// LINK procedure for a path
                    ///
                    /// Same as `nfs-mount::link` but instead of taking in a source file handle, destination directory file handle,
                    /// and destination filename, takes in a source path for which file handle is obtained by performing one or
                    /// more LOOKUP procedures and destination path for which directory file handle is obtained by performing one
                    /// or more LOOKUP procedures.
                    fn link_path(
                        &self,
                        src_path: _rt::String,
                        dst_path: _rt::String,
                    ) -> Result<Attr, Error>;
                    /// SYMLINK procedure
                    ///
                    /// Procedure SYMLINK creates a new symbolic link.
                    fn symlink(
                        &self,
                        src_path: _rt::String,
                        dst_dir_fh: Fh,
                        dst_filename: _rt::String,
                    ) -> Result<ObjRes, Error>;
                    /// SYMLINK procedure for a path
                    ///
                    /// Same as `nfs-mount::symlink` but instead of taking in a destination directory file handle and destination
                    /// filename, takes in a  destination path for which directory file handle is obtained by performing one or
                    /// more LOOKUP procedures.
                    fn symlink_path(
                        &self,
                        src_path: _rt::String,
                        dst_path: _rt::String,
                    ) -> Result<ObjRes, Error>;
                    /// READLINK procedure
                    ///
                    /// Procedure READLINK reads the data associated with a symbolic link.
                    fn readlink(&self, fh: Fh) -> Result<_rt::String, Error>;
                    /// READLINK procedure for a path
                    ///
                    /// Same as `nfs-mount::readlink` but instead of taking in a file handle, takes in a path for which file handle
                    /// is obtained by performing one or more LOOKUP procedures.
                    fn readlink_path(
                        &self,
                        path: _rt::String,
                    ) -> Result<_rt::String, Error>;
                    /// LOOKUP procedure
                    ///
                    /// Procedure LOOKUP searches a directory for a specific name and returns the file handle and attributes for
                    /// the corresponding file system object.
                    fn lookup(
                        &self,
                        dir_fh: Fh,
                        filename: _rt::String,
                    ) -> Result<ObjRes, Error>;
                    /// LOOKUP procedure for a path
                    ///
                    /// Same as `nfs-mount::lookup` but instead of taking in a directory file handle and filename, takes in a path
                    /// for which directory file handle is obtained by performing one or more LOOKUP procedures for each directory
                    /// in the path, in turn.
                    fn lookup_path(&self, path: _rt::String) -> Result<ObjRes, Error>;
                    /// PATHCONF procedure
                    ///
                    /// Procedure PATHCONF retrieves the pathconf information for a file or directory.
                    fn pathconf(&self, fh: Fh) -> Result<PathConf, Error>;
                    /// PATHCONF procedure for a path
                    ///
                    /// Same as `nfs-mount::pathconf` but instead of taking in a file handle, takes in a path for which file handle
                    /// is obtained by performing one or more LOOKUP procedures.
                    fn pathconf_path(
                        &self,
                        path: _rt::String,
                    ) -> Result<PathConf, Error>;
                    /// READ procedure
                    ///
                    /// Procedure READ reads data from a file.
                    fn read(
                        &self,
                        fh: Fh,
                        offset: u64,
                        count: u32,
                    ) -> Result<Bytes, Error>;
                    /// READ procedure for a path
                    ///
                    /// Same as `nfs-mount::read` but instead of taking in a file handle, takes in a path for which file handle is
                    /// obtained by performing one or more LOOKUP procedures.
                    fn read_path(
                        &self,
                        path: _rt::String,
                        offset: u64,
                        count: u32,
                    ) -> Result<Bytes, Error>;
                    /// WRITE procedure
                    ///
                    /// Procedure WRITE writes data to a file.
                    fn write(
                        &self,
                        fh: Fh,
                        offset: u64,
                        data: Bytes,
                    ) -> Result<u32, Error>;
                    /// WRITE procedure for a path
                    ///
                    /// Same as `nfs-mount::write` but instead of taking in a file handle, takes in a path for which file handle is
                    /// obtained by performing one or more LOOKUP procedures.
                    fn write_path(
                        &self,
                        path: _rt::String,
                        offset: u64,
                        data: Bytes,
                    ) -> Result<u32, Error>;
                    /// READDIR procedure
                    ///
                    /// Procedure READDIR retrieves a variable number of entries, in sequence, from a directory and returns the
                    /// name and file identifier for each, with information to allow the client to request additional directory
                    /// entries in a subsequent READDIR request.
                    fn readdir(
                        &self,
                        dir_fh: Fh,
                    ) -> Result<_rt::Vec<ReaddirEntry>, Error>;
                    /// READDIR procedure for a path
                    ///
                    /// Same as `nfs-mount::readdir` but instead of taking in a directory file handle, takes in a path for which
                    /// directory file handle is obtained by performing one or more LOOKUP procedures.
                    fn readdir_path(
                        &self,
                        dir_path: _rt::String,
                    ) -> Result<_rt::Vec<ReaddirEntry>, Error>;
                    /// READDIRPLUS procedure
                    ///
                    /// Procedure READDIRPLUS retrieves a variable number of entries from a file system directory and returns
                    /// complete information about each along with information to allow the client to request additional directory
                    /// entries in a subsequent READDIRPLUS.  READDIRPLUS differs from READDIR only in the amount of information
                    /// returned for each entry.  In READDIR, each entry returns the filename and the fileid.  In READDIRPLUS, each
                    /// entry returns the name, the fileid, attributes (including the fileid), and file handle.
                    fn readdirplus(
                        &self,
                        dir_fh: Fh,
                    ) -> Result<_rt::Vec<ReaddirplusEntry>, Error>;
                    /// READDIRPLUS procedure for a path
                    ///
                    /// Same as `nfs-mount::readdirplus` but instead of taking in a directory file handle, takes in a path for
                    /// which directory file handle is obtained by performing one or more LOOKUP procedures.
                    fn readdirplus_path(
                        &self,
                        dir_path: _rt::String,
                    ) -> Result<_rt::Vec<ReaddirplusEntry>, Error>;
                    /// MKDIR procedure
                    ///
                    /// Procedure MKDIR creates a new subdirectory.
                    fn mkdir(
                        &self,
                        dir_fh: Fh,
                        dirname: _rt::String,
                        mode: u32,
                    ) -> Result<ObjRes, Error>;
                    /// MKDIR procedure for a path
                    ///
                    /// Same as `nfs-mount::mkdir` but instead of taking in directory file handle and dirname, takes in a path for
                    /// which directory file handle is obtained by performing one or more LOOKUP procedures.
                    fn mkdir_path(
                        &self,
                        path: _rt::String,
                        mode: u32,
                    ) -> Result<ObjRes, Error>;
                    /// REMOVE procedure
                    ///
                    /// Procedure REMOVE removes (deletes) an entry from a directory.
                    fn remove(
                        &self,
                        dir_fh: Fh,
                        filename: _rt::String,
                    ) -> Result<(), Error>;
                    /// REMOVE procedure for a path
                    ///
                    /// Same as `nfs-mount::remove` but instead of taking in a directory file handle and filename, takes in a path
                    /// for which directory file handle is obtained by performing one or more LOOKUP procedures.
                    fn remove_path(&self, path: _rt::String) -> Result<(), Error>;
                    /// RMDIR procedure
                    ///
                    /// Procedure RMDIR removes (deletes) a subdirectory from a directory.
                    fn rmdir(
                        &self,
                        dir_fh: Fh,
                        dirname: _rt::String,
                    ) -> Result<(), Error>;
                    /// RMDIR procedure for a path
                    ///
                    /// Same as `nfs-mount::rmdir` but instead of taking in a directory file handle and directory name, takes in a
                    /// path for which directory file handle is obtained by performing one or more LOOKUP procedures.
                    fn rmdir_path(&self, path: _rt::String) -> Result<(), Error>;
                    /// RENAME procedure
                    ///
                    /// Procedure RENAME renames an entry.
                    fn rename(
                        &self,
                        from_dir_fh: Fh,
                        from_filename: _rt::String,
                        to_dir_fh: Fh,
                        to_filename: _rt::String,
                    ) -> Result<(), Error>;
                    /// RENAME procedure for a path
                    ///
                    /// Same as `nfs-mount::rename` but instead of taking in a from directory file handle, from filename, to
                    /// directory file handle, and to filename, takes in a from path for which directory file handle is obtained by
                    /// performing one or more LOOKUP procedures and to path for which directory file handle is obtained by
                    /// performing one or more LOOKUP procedures.
                    fn rename_path(
                        &self,
                        from_path: _rt::String,
                        to_path: _rt::String,
                    ) -> Result<(), Error>;
                    /// UMOUNT procedure
                    ///
                    /// Procedure UMOUNT unmounts the mount itself.
                    fn umount(&self) -> Result<(), Error>;
                    /// Return `nfs-mount`'s NFS version
                    fn version(&self) -> Result<NfsVersion, Error>;
                }
                #[doc(hidden)]
                macro_rules! __export_component_nfs_rs_nfs_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "component:nfs-rs/nfs#parse-url-and-mount"] unsafe extern "C" fn
                        export_parse_url_and_mount(arg0 : * mut u8, arg1 : usize,) -> *
                        mut u8 { $($path_to_types)*::
                        _export_parse_url_and_mount_cabi::<$ty > (arg0, arg1) }
                        #[export_name =
                        "cabi_post_component:nfs-rs/nfs#parse-url-and-mount"] unsafe
                        extern "C" fn _post_return_parse_url_and_mount(arg0 : * mut u8,)
                        { $($path_to_types)*:: __post_return_parse_url_and_mount::<$ty >
                        (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.null-op"] unsafe extern
                        "C" fn export_method_nfs_mount_null_op(arg0 : * mut u8,) -> * mut
                        u8 { $($path_to_types)*::
                        _export_method_nfs_mount_null_op_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.null-op"]
                        unsafe extern "C" fn _post_return_method_nfs_mount_null_op(arg0 :
                        * mut u8,) { $($path_to_types)*::
                        __post_return_method_nfs_mount_null_op::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.access"] unsafe extern
                        "C" fn export_method_nfs_mount_access(arg0 : * mut u8, arg1 : *
                        mut u8, arg2 : usize, arg3 : i32,) -> * mut u8 {
                        $($path_to_types)*:: _export_method_nfs_mount_access_cabi::<<$ty
                        as $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2,
                        arg3) } #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.access"] unsafe
                        extern "C" fn _post_return_method_nfs_mount_access(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_nfs_mount_access::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.access-path"] unsafe
                        extern "C" fn export_method_nfs_mount_access_path(arg0 : * mut
                        u8, arg1 : * mut u8, arg2 : usize, arg3 : i32,) -> * mut u8 {
                        $($path_to_types)*::
                        _export_method_nfs_mount_access_path_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2, arg3)
                        } #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.access-path"]
                        unsafe extern "C" fn
                        _post_return_method_nfs_mount_access_path(arg0 : * mut u8,) {
                        $($path_to_types)*::
                        __post_return_method_nfs_mount_access_path::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.close"] unsafe extern "C"
                        fn export_method_nfs_mount_close(arg0 : * mut u8, arg1 : i32,
                        arg2 : i64,) -> * mut u8 { $($path_to_types)*::
                        _export_method_nfs_mount_close_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2) }
                        #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.close"] unsafe
                        extern "C" fn _post_return_method_nfs_mount_close(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_nfs_mount_close::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.commit"] unsafe extern
                        "C" fn export_method_nfs_mount_commit(arg0 : * mut u8, arg1 : *
                        mut u8, arg2 : usize, arg3 : i64, arg4 : i32,) -> * mut u8 {
                        $($path_to_types)*:: _export_method_nfs_mount_commit_cabi::<<$ty
                        as $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2,
                        arg3, arg4) } #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.commit"] unsafe
                        extern "C" fn _post_return_method_nfs_mount_commit(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_nfs_mount_commit::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.commit-path"] unsafe
                        extern "C" fn export_method_nfs_mount_commit_path(arg0 : * mut
                        u8, arg1 : * mut u8, arg2 : usize, arg3 : i64, arg4 : i32,) -> *
                        mut u8 { $($path_to_types)*::
                        _export_method_nfs_mount_commit_path_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2, arg3,
                        arg4) } #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.commit-path"]
                        unsafe extern "C" fn
                        _post_return_method_nfs_mount_commit_path(arg0 : * mut u8,) {
                        $($path_to_types)*::
                        __post_return_method_nfs_mount_commit_path::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.create"] unsafe extern
                        "C" fn export_method_nfs_mount_create(arg0 : * mut u8, arg1 : *
                        mut u8, arg2 : usize, arg3 : * mut u8, arg4 : usize, arg5 : i32,)
                        -> * mut u8 { $($path_to_types)*::
                        _export_method_nfs_mount_create_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2, arg3,
                        arg4, arg5) } #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.create"] unsafe
                        extern "C" fn _post_return_method_nfs_mount_create(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_nfs_mount_create::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.create-path"] unsafe
                        extern "C" fn export_method_nfs_mount_create_path(arg0 : * mut
                        u8, arg1 : * mut u8, arg2 : usize, arg3 : i32,) -> * mut u8 {
                        $($path_to_types)*::
                        _export_method_nfs_mount_create_path_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2, arg3)
                        } #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.create-path"]
                        unsafe extern "C" fn
                        _post_return_method_nfs_mount_create_path(arg0 : * mut u8,) {
                        $($path_to_types)*::
                        __post_return_method_nfs_mount_create_path::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.delegpurge"] unsafe
                        extern "C" fn export_method_nfs_mount_delegpurge(arg0 : * mut u8,
                        arg1 : i64,) -> * mut u8 { $($path_to_types)*::
                        _export_method_nfs_mount_delegpurge_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1) }
                        #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.delegpurge"]
                        unsafe extern "C" fn
                        _post_return_method_nfs_mount_delegpurge(arg0 : * mut u8,) {
                        $($path_to_types)*::
                        __post_return_method_nfs_mount_delegpurge::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.delegreturn"] unsafe
                        extern "C" fn export_method_nfs_mount_delegreturn(arg0 : * mut
                        u8, arg1 : i64,) -> * mut u8 { $($path_to_types)*::
                        _export_method_nfs_mount_delegreturn_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1) }
                        #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.delegreturn"]
                        unsafe extern "C" fn
                        _post_return_method_nfs_mount_delegreturn(arg0 : * mut u8,) {
                        $($path_to_types)*::
                        __post_return_method_nfs_mount_delegreturn::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.getattr"] unsafe extern
                        "C" fn export_method_nfs_mount_getattr(arg0 : * mut u8, arg1 : *
                        mut u8, arg2 : usize,) -> * mut u8 { $($path_to_types)*::
                        _export_method_nfs_mount_getattr_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2) }
                        #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.getattr"]
                        unsafe extern "C" fn _post_return_method_nfs_mount_getattr(arg0 :
                        * mut u8,) { $($path_to_types)*::
                        __post_return_method_nfs_mount_getattr::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.getattr-path"] unsafe
                        extern "C" fn export_method_nfs_mount_getattr_path(arg0 : * mut
                        u8, arg1 : * mut u8, arg2 : usize,) -> * mut u8 {
                        $($path_to_types)*::
                        _export_method_nfs_mount_getattr_path_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2) }
                        #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.getattr-path"]
                        unsafe extern "C" fn
                        _post_return_method_nfs_mount_getattr_path(arg0 : * mut u8,) {
                        $($path_to_types)*::
                        __post_return_method_nfs_mount_getattr_path::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.setattr"] unsafe extern
                        "C" fn export_method_nfs_mount_setattr(arg0 : * mut u8,) -> * mut
                        u8 { $($path_to_types)*::
                        _export_method_nfs_mount_setattr_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.setattr"]
                        unsafe extern "C" fn _post_return_method_nfs_mount_setattr(arg0 :
                        * mut u8,) { $($path_to_types)*::
                        __post_return_method_nfs_mount_setattr::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.setattr-path"] unsafe
                        extern "C" fn export_method_nfs_mount_setattr_path(arg0 : * mut
                        u8,) -> * mut u8 { $($path_to_types)*::
                        _export_method_nfs_mount_setattr_path_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.setattr-path"]
                        unsafe extern "C" fn
                        _post_return_method_nfs_mount_setattr_path(arg0 : * mut u8,) {
                        $($path_to_types)*::
                        __post_return_method_nfs_mount_setattr_path::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.getfh"] unsafe extern "C"
                        fn export_method_nfs_mount_getfh(arg0 : * mut u8,) -> * mut u8 {
                        $($path_to_types)*:: _export_method_nfs_mount_getfh_cabi::<<$ty
                        as $($path_to_types)*:: Guest >::NfsMount > (arg0) }
                        #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.getfh"] unsafe
                        extern "C" fn _post_return_method_nfs_mount_getfh(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_nfs_mount_getfh::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.link"] unsafe extern "C"
                        fn export_method_nfs_mount_link(arg0 : * mut u8, arg1 : * mut u8,
                        arg2 : usize, arg3 : * mut u8, arg4 : usize, arg5 : * mut u8,
                        arg6 : usize,) -> * mut u8 { $($path_to_types)*::
                        _export_method_nfs_mount_link_cabi::<<$ty as $($path_to_types)*::
                        Guest >::NfsMount > (arg0, arg1, arg2, arg3, arg4, arg5, arg6) }
                        #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.link"] unsafe
                        extern "C" fn _post_return_method_nfs_mount_link(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_nfs_mount_link::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.link-path"] unsafe extern
                        "C" fn export_method_nfs_mount_link_path(arg0 : * mut u8, arg1 :
                        * mut u8, arg2 : usize, arg3 : * mut u8, arg4 : usize,) -> * mut
                        u8 { $($path_to_types)*::
                        _export_method_nfs_mount_link_path_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2, arg3,
                        arg4) } #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.link-path"]
                        unsafe extern "C" fn _post_return_method_nfs_mount_link_path(arg0
                        : * mut u8,) { $($path_to_types)*::
                        __post_return_method_nfs_mount_link_path::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.symlink"] unsafe extern
                        "C" fn export_method_nfs_mount_symlink(arg0 : * mut u8, arg1 : *
                        mut u8, arg2 : usize, arg3 : * mut u8, arg4 : usize, arg5 : * mut
                        u8, arg6 : usize,) -> * mut u8 { $($path_to_types)*::
                        _export_method_nfs_mount_symlink_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2, arg3,
                        arg4, arg5, arg6) } #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.symlink"]
                        unsafe extern "C" fn _post_return_method_nfs_mount_symlink(arg0 :
                        * mut u8,) { $($path_to_types)*::
                        __post_return_method_nfs_mount_symlink::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.symlink-path"] unsafe
                        extern "C" fn export_method_nfs_mount_symlink_path(arg0 : * mut
                        u8, arg1 : * mut u8, arg2 : usize, arg3 : * mut u8, arg4 :
                        usize,) -> * mut u8 { $($path_to_types)*::
                        _export_method_nfs_mount_symlink_path_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2, arg3,
                        arg4) } #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.symlink-path"]
                        unsafe extern "C" fn
                        _post_return_method_nfs_mount_symlink_path(arg0 : * mut u8,) {
                        $($path_to_types)*::
                        __post_return_method_nfs_mount_symlink_path::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.readlink"] unsafe extern
                        "C" fn export_method_nfs_mount_readlink(arg0 : * mut u8, arg1 : *
                        mut u8, arg2 : usize,) -> * mut u8 { $($path_to_types)*::
                        _export_method_nfs_mount_readlink_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2) }
                        #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.readlink"]
                        unsafe extern "C" fn _post_return_method_nfs_mount_readlink(arg0
                        : * mut u8,) { $($path_to_types)*::
                        __post_return_method_nfs_mount_readlink::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.readlink-path"] unsafe
                        extern "C" fn export_method_nfs_mount_readlink_path(arg0 : * mut
                        u8, arg1 : * mut u8, arg2 : usize,) -> * mut u8 {
                        $($path_to_types)*::
                        _export_method_nfs_mount_readlink_path_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2) }
                        #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.readlink-path"]
                        unsafe extern "C" fn
                        _post_return_method_nfs_mount_readlink_path(arg0 : * mut u8,) {
                        $($path_to_types)*::
                        __post_return_method_nfs_mount_readlink_path::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.lookup"] unsafe extern
                        "C" fn export_method_nfs_mount_lookup(arg0 : * mut u8, arg1 : *
                        mut u8, arg2 : usize, arg3 : * mut u8, arg4 : usize,) -> * mut u8
                        { $($path_to_types)*::
                        _export_method_nfs_mount_lookup_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2, arg3,
                        arg4) } #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.lookup"] unsafe
                        extern "C" fn _post_return_method_nfs_mount_lookup(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_nfs_mount_lookup::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.lookup-path"] unsafe
                        extern "C" fn export_method_nfs_mount_lookup_path(arg0 : * mut
                        u8, arg1 : * mut u8, arg2 : usize,) -> * mut u8 {
                        $($path_to_types)*::
                        _export_method_nfs_mount_lookup_path_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2) }
                        #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.lookup-path"]
                        unsafe extern "C" fn
                        _post_return_method_nfs_mount_lookup_path(arg0 : * mut u8,) {
                        $($path_to_types)*::
                        __post_return_method_nfs_mount_lookup_path::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.pathconf"] unsafe extern
                        "C" fn export_method_nfs_mount_pathconf(arg0 : * mut u8, arg1 : *
                        mut u8, arg2 : usize,) -> * mut u8 { $($path_to_types)*::
                        _export_method_nfs_mount_pathconf_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2) }
                        #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.pathconf"]
                        unsafe extern "C" fn _post_return_method_nfs_mount_pathconf(arg0
                        : * mut u8,) { $($path_to_types)*::
                        __post_return_method_nfs_mount_pathconf::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.pathconf-path"] unsafe
                        extern "C" fn export_method_nfs_mount_pathconf_path(arg0 : * mut
                        u8, arg1 : * mut u8, arg2 : usize,) -> * mut u8 {
                        $($path_to_types)*::
                        _export_method_nfs_mount_pathconf_path_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2) }
                        #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.pathconf-path"]
                        unsafe extern "C" fn
                        _post_return_method_nfs_mount_pathconf_path(arg0 : * mut u8,) {
                        $($path_to_types)*::
                        __post_return_method_nfs_mount_pathconf_path::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.read"] unsafe extern "C"
                        fn export_method_nfs_mount_read(arg0 : * mut u8, arg1 : * mut u8,
                        arg2 : usize, arg3 : i64, arg4 : i32,) -> * mut u8 {
                        $($path_to_types)*:: _export_method_nfs_mount_read_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2, arg3,
                        arg4) } #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.read"] unsafe
                        extern "C" fn _post_return_method_nfs_mount_read(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_nfs_mount_read::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.read-path"] unsafe extern
                        "C" fn export_method_nfs_mount_read_path(arg0 : * mut u8, arg1 :
                        * mut u8, arg2 : usize, arg3 : i64, arg4 : i32,) -> * mut u8 {
                        $($path_to_types)*::
                        _export_method_nfs_mount_read_path_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2, arg3,
                        arg4) } #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.read-path"]
                        unsafe extern "C" fn _post_return_method_nfs_mount_read_path(arg0
                        : * mut u8,) { $($path_to_types)*::
                        __post_return_method_nfs_mount_read_path::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.write"] unsafe extern "C"
                        fn export_method_nfs_mount_write(arg0 : * mut u8, arg1 : * mut
                        u8, arg2 : usize, arg3 : i64, arg4 : * mut u8, arg5 : usize,) ->
                        * mut u8 { $($path_to_types)*::
                        _export_method_nfs_mount_write_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2, arg3,
                        arg4, arg5) } #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.write"] unsafe
                        extern "C" fn _post_return_method_nfs_mount_write(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_nfs_mount_write::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.write-path"] unsafe
                        extern "C" fn export_method_nfs_mount_write_path(arg0 : * mut u8,
                        arg1 : * mut u8, arg2 : usize, arg3 : i64, arg4 : * mut u8, arg5
                        : usize,) -> * mut u8 { $($path_to_types)*::
                        _export_method_nfs_mount_write_path_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2, arg3,
                        arg4, arg5) } #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.write-path"]
                        unsafe extern "C" fn
                        _post_return_method_nfs_mount_write_path(arg0 : * mut u8,) {
                        $($path_to_types)*::
                        __post_return_method_nfs_mount_write_path::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.readdir"] unsafe extern
                        "C" fn export_method_nfs_mount_readdir(arg0 : * mut u8, arg1 : *
                        mut u8, arg2 : usize,) -> * mut u8 { $($path_to_types)*::
                        _export_method_nfs_mount_readdir_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2) }
                        #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.readdir"]
                        unsafe extern "C" fn _post_return_method_nfs_mount_readdir(arg0 :
                        * mut u8,) { $($path_to_types)*::
                        __post_return_method_nfs_mount_readdir::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.readdir-path"] unsafe
                        extern "C" fn export_method_nfs_mount_readdir_path(arg0 : * mut
                        u8, arg1 : * mut u8, arg2 : usize,) -> * mut u8 {
                        $($path_to_types)*::
                        _export_method_nfs_mount_readdir_path_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2) }
                        #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.readdir-path"]
                        unsafe extern "C" fn
                        _post_return_method_nfs_mount_readdir_path(arg0 : * mut u8,) {
                        $($path_to_types)*::
                        __post_return_method_nfs_mount_readdir_path::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.readdirplus"] unsafe
                        extern "C" fn export_method_nfs_mount_readdirplus(arg0 : * mut
                        u8, arg1 : * mut u8, arg2 : usize,) -> * mut u8 {
                        $($path_to_types)*::
                        _export_method_nfs_mount_readdirplus_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2) }
                        #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.readdirplus"]
                        unsafe extern "C" fn
                        _post_return_method_nfs_mount_readdirplus(arg0 : * mut u8,) {
                        $($path_to_types)*::
                        __post_return_method_nfs_mount_readdirplus::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.readdirplus-path"] unsafe
                        extern "C" fn export_method_nfs_mount_readdirplus_path(arg0 : *
                        mut u8, arg1 : * mut u8, arg2 : usize,) -> * mut u8 {
                        $($path_to_types)*::
                        _export_method_nfs_mount_readdirplus_path_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2) }
                        #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.readdirplus-path"]
                        unsafe extern "C" fn
                        _post_return_method_nfs_mount_readdirplus_path(arg0 : * mut u8,)
                        { $($path_to_types)*::
                        __post_return_method_nfs_mount_readdirplus_path::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.mkdir"] unsafe extern "C"
                        fn export_method_nfs_mount_mkdir(arg0 : * mut u8, arg1 : * mut
                        u8, arg2 : usize, arg3 : * mut u8, arg4 : usize, arg5 : i32,) ->
                        * mut u8 { $($path_to_types)*::
                        _export_method_nfs_mount_mkdir_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2, arg3,
                        arg4, arg5) } #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.mkdir"] unsafe
                        extern "C" fn _post_return_method_nfs_mount_mkdir(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_nfs_mount_mkdir::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.mkdir-path"] unsafe
                        extern "C" fn export_method_nfs_mount_mkdir_path(arg0 : * mut u8,
                        arg1 : * mut u8, arg2 : usize, arg3 : i32,) -> * mut u8 {
                        $($path_to_types)*::
                        _export_method_nfs_mount_mkdir_path_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2, arg3)
                        } #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.mkdir-path"]
                        unsafe extern "C" fn
                        _post_return_method_nfs_mount_mkdir_path(arg0 : * mut u8,) {
                        $($path_to_types)*::
                        __post_return_method_nfs_mount_mkdir_path::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.remove"] unsafe extern
                        "C" fn export_method_nfs_mount_remove(arg0 : * mut u8, arg1 : *
                        mut u8, arg2 : usize, arg3 : * mut u8, arg4 : usize,) -> * mut u8
                        { $($path_to_types)*::
                        _export_method_nfs_mount_remove_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2, arg3,
                        arg4) } #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.remove"] unsafe
                        extern "C" fn _post_return_method_nfs_mount_remove(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_nfs_mount_remove::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.remove-path"] unsafe
                        extern "C" fn export_method_nfs_mount_remove_path(arg0 : * mut
                        u8, arg1 : * mut u8, arg2 : usize,) -> * mut u8 {
                        $($path_to_types)*::
                        _export_method_nfs_mount_remove_path_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2) }
                        #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.remove-path"]
                        unsafe extern "C" fn
                        _post_return_method_nfs_mount_remove_path(arg0 : * mut u8,) {
                        $($path_to_types)*::
                        __post_return_method_nfs_mount_remove_path::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.rmdir"] unsafe extern "C"
                        fn export_method_nfs_mount_rmdir(arg0 : * mut u8, arg1 : * mut
                        u8, arg2 : usize, arg3 : * mut u8, arg4 : usize,) -> * mut u8 {
                        $($path_to_types)*:: _export_method_nfs_mount_rmdir_cabi::<<$ty
                        as $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2,
                        arg3, arg4) } #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.rmdir"] unsafe
                        extern "C" fn _post_return_method_nfs_mount_rmdir(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_nfs_mount_rmdir::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.rmdir-path"] unsafe
                        extern "C" fn export_method_nfs_mount_rmdir_path(arg0 : * mut u8,
                        arg1 : * mut u8, arg2 : usize,) -> * mut u8 {
                        $($path_to_types)*::
                        _export_method_nfs_mount_rmdir_path_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2) }
                        #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.rmdir-path"]
                        unsafe extern "C" fn
                        _post_return_method_nfs_mount_rmdir_path(arg0 : * mut u8,) {
                        $($path_to_types)*::
                        __post_return_method_nfs_mount_rmdir_path::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.rename"] unsafe extern
                        "C" fn export_method_nfs_mount_rename(arg0 : * mut u8, arg1 : *
                        mut u8, arg2 : usize, arg3 : * mut u8, arg4 : usize, arg5 : * mut
                        u8, arg6 : usize, arg7 : * mut u8, arg8 : usize,) -> * mut u8 {
                        $($path_to_types)*:: _export_method_nfs_mount_rename_cabi::<<$ty
                        as $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2,
                        arg3, arg4, arg5, arg6, arg7, arg8) } #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.rename"] unsafe
                        extern "C" fn _post_return_method_nfs_mount_rename(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_nfs_mount_rename::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.rename-path"] unsafe
                        extern "C" fn export_method_nfs_mount_rename_path(arg0 : * mut
                        u8, arg1 : * mut u8, arg2 : usize, arg3 : * mut u8, arg4 :
                        usize,) -> * mut u8 { $($path_to_types)*::
                        _export_method_nfs_mount_rename_path_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0, arg1, arg2, arg3,
                        arg4) } #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.rename-path"]
                        unsafe extern "C" fn
                        _post_return_method_nfs_mount_rename_path(arg0 : * mut u8,) {
                        $($path_to_types)*::
                        __post_return_method_nfs_mount_rename_path::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.umount"] unsafe extern
                        "C" fn export_method_nfs_mount_umount(arg0 : * mut u8,) -> * mut
                        u8 { $($path_to_types)*::
                        _export_method_nfs_mount_umount_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.umount"] unsafe
                        extern "C" fn _post_return_method_nfs_mount_umount(arg0 : * mut
                        u8,) { $($path_to_types)*::
                        __post_return_method_nfs_mount_umount::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "component:nfs-rs/nfs#[method]nfs-mount.version"] unsafe extern
                        "C" fn export_method_nfs_mount_version(arg0 : * mut u8,) -> * mut
                        u8 { $($path_to_types)*::
                        _export_method_nfs_mount_version_cabi::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } #[export_name =
                        "cabi_post_component:nfs-rs/nfs#[method]nfs-mount.version"]
                        unsafe extern "C" fn _post_return_method_nfs_mount_version(arg0 :
                        * mut u8,) { $($path_to_types)*::
                        __post_return_method_nfs_mount_version::<<$ty as
                        $($path_to_types)*:: Guest >::NfsMount > (arg0) } const _ : () =
                        { #[doc(hidden)] #[export_name =
                        "component:nfs-rs/nfs#[dtor]nfs-mount"] #[allow(non_snake_case)]
                        unsafe extern "C" fn dtor(rep : * mut u8) { $($path_to_types)*::
                        NfsMount::dtor::< <$ty as $($path_to_types)*:: Guest >::NfsMount
                        > (rep) } }; };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_component_nfs_rs_nfs_cabi;
                #[repr(align(8))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 120]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 120],
                );
            }
        }
    }
}
mod _rt {
    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }
    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }
    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }
        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }
        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }
    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource").field("handle", &self.handle).finish()
        }
    }
    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    u32::MAX => {}
                    other => T::drop(other),
                }
            }
        }
    }
    pub unsafe fn bool_lift(val: u8) -> bool {
        if cfg!(debug_assertions) {
            match val {
                0 => false,
                1 => true,
                _ => panic!("invalid bool discriminant"),
            }
        } else {
            val != 0
        }
    }
    pub use alloc_crate::vec::Vec;
    pub use alloc_crate::alloc;
    pub use alloc_crate::string::String;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub fn as_i64<T: AsI64>(t: T) -> i64 {
        t.as_i64()
    }
    pub trait AsI64 {
        fn as_i64(self) -> i64;
    }
    impl<'a, T: Copy + AsI64> AsI64 for &'a T {
        fn as_i64(self) -> i64 {
            (*self).as_i64()
        }
    }
    impl AsI64 for i64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    impl AsI64 for u64 {
        #[inline]
        fn as_i64(self) -> i64 {
            self as i64
        }
    }
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    pub use alloc_crate::boxed::Box;
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    extern crate alloc as alloc_crate;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_nfs_rs_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::component::nfs_rs::nfs::__export_component_nfs_rs_nfs_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::component::nfs_rs::nfs);
    };
}
#[doc(inline)]
pub(crate) use __export_nfs_rs_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.30.0:nfs-rs:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 7838] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xa1<\x01A\x02\x01A\x1f\
\x01B\x0a\x04\0\x08pollable\x03\x01\x01h\0\x01@\x01\x04self\x01\0\x7f\x04\0\x16[\
method]pollable.ready\x01\x02\x01@\x01\x04self\x01\x01\0\x04\0\x16[method]pollab\
le.block\x01\x03\x01p\x01\x01py\x01@\x01\x02in\x04\0\x05\x04\0\x04poll\x01\x06\x03\
\x01\x12wasi:io/poll@0.2.0\x05\0\x01B\x04\x04\0\x05error\x03\x01\x01h\0\x01@\x01\
\x04self\x01\0s\x04\0\x1d[method]error.to-debug-string\x01\x02\x03\x01\x13wasi:i\
o/error@0.2.0\x05\x01\x02\x03\0\x01\x05error\x02\x03\0\0\x08pollable\x01B(\x02\x03\
\x02\x01\x02\x04\0\x05error\x03\0\0\x02\x03\x02\x01\x03\x04\0\x08pollable\x03\0\x02\
\x01i\x01\x01q\x02\x15last-operation-failed\x01\x04\0\x06closed\0\0\x04\0\x0cstr\
eam-error\x03\0\x05\x04\0\x0cinput-stream\x03\x01\x04\0\x0doutput-stream\x03\x01\
\x01h\x07\x01p}\x01j\x01\x0a\x01\x06\x01@\x02\x04self\x09\x03lenw\0\x0b\x04\0\x19\
[method]input-stream.read\x01\x0c\x04\0\"[method]input-stream.blocking-read\x01\x0c\
\x01j\x01w\x01\x06\x01@\x02\x04self\x09\x03lenw\0\x0d\x04\0\x19[method]input-str\
eam.skip\x01\x0e\x04\0\"[method]input-stream.blocking-skip\x01\x0e\x01i\x03\x01@\
\x01\x04self\x09\0\x0f\x04\0\x1e[method]input-stream.subscribe\x01\x10\x01h\x08\x01\
@\x01\x04self\x11\0\x0d\x04\0![method]output-stream.check-write\x01\x12\x01j\0\x01\
\x06\x01@\x02\x04self\x11\x08contents\x0a\0\x13\x04\0\x1b[method]output-stream.w\
rite\x01\x14\x04\0.[method]output-stream.blocking-write-and-flush\x01\x14\x01@\x01\
\x04self\x11\0\x13\x04\0\x1b[method]output-stream.flush\x01\x15\x04\0$[method]ou\
tput-stream.blocking-flush\x01\x15\x01@\x01\x04self\x11\0\x0f\x04\0\x1f[method]o\
utput-stream.subscribe\x01\x16\x01@\x02\x04self\x11\x03lenw\0\x13\x04\0\"[method\
]output-stream.write-zeroes\x01\x17\x04\05[method]output-stream.blocking-write-z\
eroes-and-flush\x01\x17\x01@\x03\x04self\x11\x03src\x09\x03lenw\0\x0d\x04\0\x1c[\
method]output-stream.splice\x01\x18\x04\0%[method]output-stream.blocking-splice\x01\
\x18\x03\x01\x15wasi:io/streams@0.2.0\x05\x04\x01B\x11\x04\0\x07network\x03\x01\x01\
m\x15\x07unknown\x0daccess-denied\x0dnot-supported\x10invalid-argument\x0dout-of\
-memory\x07timeout\x14concurrency-conflict\x0fnot-in-progress\x0bwould-block\x0d\
invalid-state\x10new-socket-limit\x14address-not-bindable\x0eaddress-in-use\x12r\
emote-unreachable\x12connection-refused\x10connection-reset\x12connection-aborte\
d\x12datagram-too-large\x11name-unresolvable\x1atemporary-resolver-failure\x1ape\
rmanent-resolver-failure\x04\0\x0aerror-code\x03\0\x01\x01m\x02\x04ipv4\x04ipv6\x04\
\0\x11ip-address-family\x03\0\x03\x01o\x04}}}}\x04\0\x0cipv4-address\x03\0\x05\x01\
o\x08{{{{{{{{\x04\0\x0cipv6-address\x03\0\x07\x01q\x02\x04ipv4\x01\x06\0\x04ipv6\
\x01\x08\0\x04\0\x0aip-address\x03\0\x09\x01r\x02\x04port{\x07address\x06\x04\0\x13\
ipv4-socket-address\x03\0\x0b\x01r\x04\x04port{\x09flow-infoy\x07address\x08\x08\
scope-idy\x04\0\x13ipv6-socket-address\x03\0\x0d\x01q\x02\x04ipv4\x01\x0c\0\x04i\
pv6\x01\x0e\0\x04\0\x11ip-socket-address\x03\0\x0f\x03\x01\x1awasi:sockets/netwo\
rk@0.2.0\x05\x05\x02\x03\0\x03\x07network\x01B\x05\x02\x03\x02\x01\x06\x04\0\x07\
network\x03\0\0\x01i\x01\x01@\0\0\x02\x04\0\x10instance-network\x01\x03\x03\x01#\
wasi:sockets/instance-network@0.2.0\x05\x07\x02\x03\0\x03\x0aerror-code\x02\x03\0\
\x03\x0aip-address\x01B\x16\x02\x03\x02\x01\x03\x04\0\x08pollable\x03\0\0\x02\x03\
\x02\x01\x06\x04\0\x07network\x03\0\x02\x02\x03\x02\x01\x08\x04\0\x0aerror-code\x03\
\0\x04\x02\x03\x02\x01\x09\x04\0\x0aip-address\x03\0\x06\x04\0\x16resolve-addres\
s-stream\x03\x01\x01h\x08\x01k\x07\x01j\x01\x0a\x01\x05\x01@\x01\x04self\x09\0\x0b\
\x04\03[method]resolve-address-stream.resolve-next-address\x01\x0c\x01i\x01\x01@\
\x01\x04self\x09\0\x0d\x04\0([method]resolve-address-stream.subscribe\x01\x0e\x01\
h\x03\x01i\x08\x01j\x01\x10\x01\x05\x01@\x02\x07network\x0f\x04names\0\x11\x04\0\
\x11resolve-addresses\x01\x12\x03\x01!wasi:sockets/ip-name-lookup@0.2.0\x05\x0a\x01\
B\x0f\x02\x03\x02\x01\x03\x04\0\x08pollable\x03\0\0\x01w\x04\0\x07instant\x03\0\x02\
\x01w\x04\0\x08duration\x03\0\x04\x01@\0\0\x03\x04\0\x03now\x01\x06\x01@\0\0\x05\
\x04\0\x0aresolution\x01\x07\x01i\x01\x01@\x01\x04when\x03\0\x08\x04\0\x11subscr\
ibe-instant\x01\x09\x01@\x01\x04when\x05\0\x08\x04\0\x12subscribe-duration\x01\x0a\
\x03\x01!wasi:clocks/monotonic-clock@0.2.0\x05\x0b\x02\x03\0\x02\x0cinput-stream\
\x02\x03\0\x02\x0doutput-stream\x02\x03\0\x06\x08duration\x02\x03\0\x03\x11ip-so\
cket-address\x02\x03\0\x03\x11ip-address-family\x01BT\x02\x03\x02\x01\x0c\x04\0\x0c\
input-stream\x03\0\0\x02\x03\x02\x01\x0d\x04\0\x0doutput-stream\x03\0\x02\x02\x03\
\x02\x01\x03\x04\0\x08pollable\x03\0\x04\x02\x03\x02\x01\x0e\x04\0\x08duration\x03\
\0\x06\x02\x03\x02\x01\x06\x04\0\x07network\x03\0\x08\x02\x03\x02\x01\x08\x04\0\x0a\
error-code\x03\0\x0a\x02\x03\x02\x01\x0f\x04\0\x11ip-socket-address\x03\0\x0c\x02\
\x03\x02\x01\x10\x04\0\x11ip-address-family\x03\0\x0e\x01m\x03\x07receive\x04sen\
d\x04both\x04\0\x0dshutdown-type\x03\0\x10\x04\0\x0atcp-socket\x03\x01\x01h\x12\x01\
h\x09\x01j\0\x01\x0b\x01@\x03\x04self\x13\x07network\x14\x0dlocal-address\x0d\0\x15\
\x04\0\x1d[method]tcp-socket.start-bind\x01\x16\x01@\x01\x04self\x13\0\x15\x04\0\
\x1e[method]tcp-socket.finish-bind\x01\x17\x01@\x03\x04self\x13\x07network\x14\x0e\
remote-address\x0d\0\x15\x04\0\x20[method]tcp-socket.start-connect\x01\x18\x01i\x01\
\x01i\x03\x01o\x02\x19\x1a\x01j\x01\x1b\x01\x0b\x01@\x01\x04self\x13\0\x1c\x04\0\
![method]tcp-socket.finish-connect\x01\x1d\x04\0\x1f[method]tcp-socket.start-lis\
ten\x01\x17\x04\0\x20[method]tcp-socket.finish-listen\x01\x17\x01i\x12\x01o\x03\x1e\
\x19\x1a\x01j\x01\x1f\x01\x0b\x01@\x01\x04self\x13\0\x20\x04\0\x19[method]tcp-so\
cket.accept\x01!\x01j\x01\x0d\x01\x0b\x01@\x01\x04self\x13\0\"\x04\0\x20[method]\
tcp-socket.local-address\x01#\x04\0![method]tcp-socket.remote-address\x01#\x01@\x01\
\x04self\x13\0\x7f\x04\0\x1f[method]tcp-socket.is-listening\x01$\x01@\x01\x04sel\
f\x13\0\x0f\x04\0![method]tcp-socket.address-family\x01%\x01@\x02\x04self\x13\x05\
valuew\0\x15\x04\0*[method]tcp-socket.set-listen-backlog-size\x01&\x01j\x01\x7f\x01\
\x0b\x01@\x01\x04self\x13\0'\x04\0%[method]tcp-socket.keep-alive-enabled\x01(\x01\
@\x02\x04self\x13\x05value\x7f\0\x15\x04\0)[method]tcp-socket.set-keep-alive-ena\
bled\x01)\x01j\x01\x07\x01\x0b\x01@\x01\x04self\x13\0*\x04\0'[method]tcp-socket.\
keep-alive-idle-time\x01+\x01@\x02\x04self\x13\x05value\x07\0\x15\x04\0+[method]\
tcp-socket.set-keep-alive-idle-time\x01,\x04\0&[method]tcp-socket.keep-alive-int\
erval\x01+\x04\0*[method]tcp-socket.set-keep-alive-interval\x01,\x01j\x01y\x01\x0b\
\x01@\x01\x04self\x13\0-\x04\0#[method]tcp-socket.keep-alive-count\x01.\x01@\x02\
\x04self\x13\x05valuey\0\x15\x04\0'[method]tcp-socket.set-keep-alive-count\x01/\x01\
j\x01}\x01\x0b\x01@\x01\x04self\x13\00\x04\0\x1c[method]tcp-socket.hop-limit\x01\
1\x01@\x02\x04self\x13\x05value}\0\x15\x04\0\x20[method]tcp-socket.set-hop-limit\
\x012\x01j\x01w\x01\x0b\x01@\x01\x04self\x13\03\x04\0&[method]tcp-socket.receive\
-buffer-size\x014\x04\0*[method]tcp-socket.set-receive-buffer-size\x01&\x04\0#[m\
ethod]tcp-socket.send-buffer-size\x014\x04\0'[method]tcp-socket.set-send-buffer-\
size\x01&\x01i\x05\x01@\x01\x04self\x13\05\x04\0\x1c[method]tcp-socket.subscribe\
\x016\x01@\x02\x04self\x13\x0dshutdown-type\x11\0\x15\x04\0\x1b[method]tcp-socke\
t.shutdown\x017\x03\x01\x16wasi:sockets/tcp@0.2.0\x05\x11\x02\x03\0\x07\x0atcp-s\
ocket\x01B\x0c\x02\x03\x02\x01\x06\x04\0\x07network\x03\0\0\x02\x03\x02\x01\x08\x04\
\0\x0aerror-code\x03\0\x02\x02\x03\x02\x01\x10\x04\0\x11ip-address-family\x03\0\x04\
\x02\x03\x02\x01\x12\x04\0\x0atcp-socket\x03\0\x06\x01i\x07\x01j\x01\x08\x01\x03\
\x01@\x01\x0eaddress-family\x05\0\x09\x04\0\x11create-tcp-socket\x01\x0a\x03\x01\
$wasi:sockets/tcp-create-socket@0.2.0\x05\x13\x01B~\x01p}\x04\0\x02fh\x03\0\0\x01\
p}\x04\0\x05bytes\x03\0\x02\x01r\x02\x07secondsy\x08nsecondsy\x04\0\x04time\x03\0\
\x04\x01o\x02yy\x01r\x0d\x09attr-typey\x09file-modey\x05nlinky\x03uidy\x03gidy\x08\
filesizew\x04usedw\x09spec-data\x06\x04fsidw\x06fileidw\x05atime\x05\x05mtime\x05\
\x05ctime\x05\x04\0\x04attr\x03\0\x07\x01k\x08\x01r\x02\x03obj\x01\x04attr\x09\x04\
\0\x07obj-res\x03\0\x0a\x01r\x07\x04attr\x09\x07linkmaxy\x08name-maxy\x08no-trun\
c\x7f\x10chown-restricted\x7f\x10case-insensitive\x7f\x0fcase-preserving\x7f\x04\
\0\x09path-conf\x03\0\x0c\x01r\x02\x06fileidw\x09file-names\x04\0\x0dreaddir-ent\
ry\x03\0\x0e\x01r\x04\x06fileidw\x09file-names\x04attr\x09\x06handle\x01\x04\0\x11\
readdirplus-entry\x03\0\x10\x01m\x03\x06nfs-v3\x06nfs-v4\x08nfs-v4p1\x04\0\x0bnf\
s-version\x03\0\x12\x01kz\x01r\x02\x0enfs-error-code\x14\x07messages\x04\0\x05er\
ror\x03\0\x15\x04\0\x09nfs-mount\x03\x01\x01h\x17\x01j\0\x01\x16\x01@\x01\x04sel\
f\x18\0\x19\x04\0\x19[method]nfs-mount.null-op\x01\x1a\x01j\x01y\x01\x16\x01@\x03\
\x04self\x18\x02fh\x01\x04modey\0\x1b\x04\0\x18[method]nfs-mount.access\x01\x1c\x01\
@\x03\x04self\x18\x04paths\x04modey\0\x1b\x04\0\x1d[method]nfs-mount.access-path\
\x01\x1d\x01@\x03\x04self\x18\x05seqidy\x07stateidw\0\x19\x04\0\x17[method]nfs-m\
ount.close\x01\x1e\x01@\x04\x04self\x18\x02fh\x01\x06offsetw\x05county\0\x19\x04\
\0\x18[method]nfs-mount.commit\x01\x1f\x01@\x04\x04self\x18\x04paths\x06offsetw\x05\
county\0\x19\x04\0\x1d[method]nfs-mount.commit-path\x01\x20\x01j\x01\x0b\x01\x16\
\x01@\x04\x04self\x18\x06dir-fh\x01\x08filenames\x04modey\0!\x04\0\x18[method]nf\
s-mount.create\x01\"\x01@\x03\x04self\x18\x04paths\x04modey\0!\x04\0\x1d[method]\
nfs-mount.create-path\x01#\x01@\x02\x04self\x18\x08clientidw\0\x19\x04\0\x1c[met\
hod]nfs-mount.delegpurge\x01$\x01@\x02\x04self\x18\x07stateidw\0\x19\x04\0\x1d[m\
ethod]nfs-mount.delegreturn\x01%\x01j\x01\x08\x01\x16\x01@\x02\x04self\x18\x02fh\
\x01\0&\x04\0\x19[method]nfs-mount.getattr\x01'\x01@\x02\x04self\x18\x04paths\0&\
\x04\0\x1e[method]nfs-mount.getattr-path\x01(\x01k\x05\x01ky\x01kw\x01@\x09\x04s\
elf\x18\x02fh\x01\x0bguard-ctime)\x04mode*\x03uid*\x03gid*\x04size+\x05atime)\x05\
mtime)\0\x19\x04\0\x19[method]nfs-mount.setattr\x01,\x01@\x09\x04self\x18\x04pat\
hs\x0dspecify-guard\x7f\x04mode*\x03uid*\x03gid*\x04size+\x05atime)\x05mtime)\0\x19\
\x04\0\x1e[method]nfs-mount.setattr-path\x01-\x04\0\x17[method]nfs-mount.getfh\x01\
\x1a\x01@\x04\x04self\x18\x06src-fh\x01\x0adst-dir-fh\x01\x0cdst-filenames\0&\x04\
\0\x16[method]nfs-mount.link\x01.\x01@\x03\x04self\x18\x08src-paths\x08dst-paths\
\0&\x04\0\x1b[method]nfs-mount.link-path\x01/\x01@\x04\x04self\x18\x08src-paths\x0a\
dst-dir-fh\x01\x0cdst-filenames\0!\x04\0\x19[method]nfs-mount.symlink\x010\x01@\x03\
\x04self\x18\x08src-paths\x08dst-paths\0!\x04\0\x1e[method]nfs-mount.symlink-pat\
h\x011\x01j\x01s\x01\x16\x01@\x02\x04self\x18\x02fh\x01\02\x04\0\x1a[method]nfs-\
mount.readlink\x013\x01@\x02\x04self\x18\x04paths\02\x04\0\x1f[method]nfs-mount.\
readlink-path\x014\x01@\x03\x04self\x18\x06dir-fh\x01\x08filenames\0!\x04\0\x18[\
method]nfs-mount.lookup\x015\x01@\x02\x04self\x18\x04paths\0!\x04\0\x1d[method]n\
fs-mount.lookup-path\x016\x01j\x01\x0d\x01\x16\x01@\x02\x04self\x18\x02fh\x01\07\
\x04\0\x1a[method]nfs-mount.pathconf\x018\x01@\x02\x04self\x18\x04paths\07\x04\0\
\x1f[method]nfs-mount.pathconf-path\x019\x01j\x01\x03\x01\x16\x01@\x04\x04self\x18\
\x02fh\x01\x06offsetw\x05county\0:\x04\0\x16[method]nfs-mount.read\x01;\x01@\x04\
\x04self\x18\x04paths\x06offsetw\x05county\0:\x04\0\x1b[method]nfs-mount.read-pa\
th\x01<\x01@\x04\x04self\x18\x02fh\x01\x06offsetw\x04data\x03\0\x1b\x04\0\x17[me\
thod]nfs-mount.write\x01=\x01@\x04\x04self\x18\x04paths\x06offsetw\x04data\x03\0\
\x1b\x04\0\x1c[method]nfs-mount.write-path\x01>\x01p\x0f\x01j\x01?\x01\x16\x01@\x02\
\x04self\x18\x06dir-fh\x01\0\xc0\0\x04\0\x19[method]nfs-mount.readdir\x01A\x01@\x02\
\x04self\x18\x08dir-paths\0\xc0\0\x04\0\x1e[method]nfs-mount.readdir-path\x01B\x01\
p\x11\x01j\x01\xc3\0\x01\x16\x01@\x02\x04self\x18\x06dir-fh\x01\0\xc4\0\x04\0\x1d\
[method]nfs-mount.readdirplus\x01E\x01@\x02\x04self\x18\x08dir-paths\0\xc4\0\x04\
\0\"[method]nfs-mount.readdirplus-path\x01F\x01@\x04\x04self\x18\x06dir-fh\x01\x07\
dirnames\x04modey\0!\x04\0\x17[method]nfs-mount.mkdir\x01G\x04\0\x1c[method]nfs-\
mount.mkdir-path\x01#\x01@\x03\x04self\x18\x06dir-fh\x01\x08filenames\0\x19\x04\0\
\x18[method]nfs-mount.remove\x01H\x01@\x02\x04self\x18\x04paths\0\x19\x04\0\x1d[\
method]nfs-mount.remove-path\x01I\x01@\x03\x04self\x18\x06dir-fh\x01\x07dirnames\
\0\x19\x04\0\x17[method]nfs-mount.rmdir\x01J\x04\0\x1c[method]nfs-mount.rmdir-pa\
th\x01I\x01@\x05\x04self\x18\x0bfrom-dir-fh\x01\x0dfrom-filenames\x09to-dir-fh\x01\
\x0bto-filenames\0\x19\x04\0\x18[method]nfs-mount.rename\x01K\x01@\x03\x04self\x18\
\x09from-paths\x07to-paths\0\x19\x04\0\x1d[method]nfs-mount.rename-path\x01L\x04\
\0\x18[method]nfs-mount.umount\x01\x1a\x01j\x01\x13\x01\x16\x01@\x01\x04self\x18\
\0\xcd\0\x04\0\x19[method]nfs-mount.version\x01N\x01i\x17\x01j\x01\xcf\0\x01\x16\
\x01@\x01\x03urls\0\xd0\0\x04\0\x13parse-url-and-mount\x01Q\x04\x01\x14component\
:nfs-rs/nfs\x05\x14\x04\x01\x17component:nfs-rs/nfs-rs\x04\0\x0b\x0c\x01\0\x06nf\
s-rs\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.215.0\
\x10wit-bindgen-rust\x060.30.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
