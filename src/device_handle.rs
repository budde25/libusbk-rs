use libusbk_sys::{
    UsbK_ClaimInterface, UsbK_Free, UsbK_ReadPipe, UsbK_ReleaseInterface, UsbK_WritePipe,
};
use std::ffi::c_void;
use std::ptr::NonNull;

use crate::error::try_unsafe;

type UsbkHandle = NonNull<c_void>;
type Interface = (u8, bool);

#[derive(Debug)]
pub struct DeviceHandle {
    // TODO use bitmap
    pub claimed_interface: std::collections::HashSet<Interface>,
    // TODO not pub
    pub(crate) handle: Option<UsbkHandle>,
}

impl DeviceHandle {
    pub fn claim_interface(&mut self, num_or_index: u8, is_index: bool) -> crate::Result<()> {
        try_unsafe!(UsbK_ClaimInterface(
            self.handle.unwrap().as_ptr(),
            num_or_index,
            is_index.into()
        ));
        self.claimed_interface.insert((num_or_index, is_index));
        Ok(())
    }

    pub fn read_pipe(&mut self, pipe_id: u8, buffer: &mut [u8]) -> crate::Result<u32> {
        let mut transferred: u32 = 0;
        try_unsafe!(UsbK_ReadPipe(
            self.handle.unwrap().as_ptr(),
            pipe_id,
            buffer.as_mut_ptr(),
            buffer.len() as u32,
            &mut transferred,
            std::ptr::null_mut(),
        ));
        return Ok(transferred);
    }

    pub fn write_pipe(&mut self, pipe_id: u8, buffer: &[u8]) -> crate::Result<u32> {
        let mut transferred: u32 = 0;
        let ptr = buffer.as_ptr();
        try_unsafe!(UsbK_WritePipe(
            self.handle.unwrap().as_ptr(),
            pipe_id,
            ptr as *mut u8,
            buffer.len() as u32,
            &mut transferred,
            std::ptr::null_mut()
        ));
        return Ok(transferred);
    }
}

impl Drop for DeviceHandle {
    fn drop(&mut self) {
        if let Some(handle) = self.handle {
            for i in &self.claimed_interface {
                unsafe { UsbK_ReleaseInterface(handle.as_ptr(), i.0, i.1 as i32) };
            }
            unsafe { UsbK_Free(handle.as_ptr()) };
        }
    }
}
