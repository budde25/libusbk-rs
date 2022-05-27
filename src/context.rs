use std::ffi::c_void;
use std::ptr::{self, NonNull};
use std::sync::Arc;

use libusbk_sys::LstK_Init;

use crate::device_list::DeviceList;
use crate::error::{self, try_unsafe};

type KListHandle = NonNull<c_void>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Context {
    context: Arc<ContextInner>,
}

#[derive(Debug, Eq, PartialEq)]
struct ContextInner {
    inner: KListHandle,
}

impl Drop for ContextInner {
    /// Closes the `libusbk` context.
    fn drop(&mut self) {
        unsafe {}
    }
}

impl Context {
    /// Opens a new `libusbk` context.
    pub fn new() -> crate::Result<Self> {
        let mut context = std::mem::MaybeUninit::<*mut c_void>::uninit();

        try_unsafe!(LstK_Init(context.as_mut_ptr(), 0));

        Ok(Self {
            context: unsafe {
                Arc::new(ContextInner {
                    inner: ptr::NonNull::new_unchecked(context.assume_init()),
                })
            },
        })
    }

    /// Get the raw libusb_context pointer, for advanced use in unsafe code.
    fn as_raw(&self) -> *mut c_void {
        self.context.inner.as_ptr()
    }
}
