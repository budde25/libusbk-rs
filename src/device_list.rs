use std::ffi::c_void;
use std::ptr::NonNull;
use std::sync::Arc;

use libusbk_sys::{LstK_Count, LstK_FindByVidPid, LstK_Free, LstK_Init};

use crate::device::Device;
use crate::error::try_unsafe;

type KListHandle = NonNull<c_void>;

/// A list of detected USB devices.
pub struct DeviceList {
    list: Arc<DeviceListInner>,
}

struct DeviceListInner {
    inner: KListHandle,
}

impl Drop for DeviceListInner {
    fn drop(&mut self) {
        unsafe { LstK_Free(self.inner.as_ptr()) };
    }
}

impl DeviceList {
    pub fn new() -> crate::Result<Self> {
        let mut context = std::mem::MaybeUninit::<*mut c_void>::uninit();

        try_unsafe!(LstK_Init(context.as_mut_ptr(), 0));

        Ok(Self {
            list: unsafe {
                Arc::new(DeviceListInner {
                    inner: NonNull::new_unchecked(context.assume_init()),
                })
            },
        })
    }

    pub fn length(&self) -> crate::Result<u32> {
        let mut count = 0;
        try_unsafe!(LstK_Count(self.list.inner.as_ptr(), &mut count));
        Ok(count)
    }

    pub fn find_with_vid_and_pid(&self, vid: i32, pid: i32) -> crate::Result<Device> {
        let mut device = Device(std::ptr::null_mut());
        try_unsafe!(LstK_FindByVidPid(
            self.list.inner.as_ptr(),
            vid,
            pid,
            &mut device.0
        ));
        Ok(device)
    }
}
