use libusbk_sys::{UsbK_ReleaseInterface, KUSB_DRIVER_API};
use std::collections::HashSet;
use std::ffi::c_void;
use std::fmt::Display;
use std::ptr::NonNull;

use crate::error::try_unsafe;
use crate::Result;

type UsbkHandle = NonNull<c_void>;
type Interface = (u8, bool);

unsafe impl Send for DeviceHandle {}

#[derive(Debug)]
pub struct DeviceHandle {
    pub(crate) dev: KUSB_DRIVER_API,
    pub(crate) driver_id: i32,
    // TODO use bitmap
    pub(crate) claimed_interface: HashSet<Interface>,
    // TODO not pub
    pub(crate) handle: Option<UsbkHandle>,
}

impl DeviceHandle {
    pub fn claim_interface(&mut self, num_or_index: u8, is_index: bool) -> Result<()> {
        try_unsafe!(self.dev.ClaimInterface.unwrap()(
            self.handle.unwrap().as_ptr(),
            num_or_index,
            is_index.into()
        ));
        self.claimed_interface.insert((num_or_index, is_index));
        Ok(())
    }

    pub fn driver_id(&self) -> DriverId {
        DriverId::from(self.driver_id)
    }

    pub fn read_pipe(&mut self, pipe_id: u8, buffer: &mut [u8]) -> crate::Result<u32> {
        let mut transferred: u32 = 0;
        try_unsafe!(self.dev.ReadPipe.unwrap()(
            self.handle.unwrap().as_ptr(),
            pipe_id,
            buffer.as_mut_ptr(),
            buffer.len() as u32,
            &mut transferred,
            std::ptr::null_mut(),
        ));
        return Ok(transferred);
    }

    pub fn write_pipe(&mut self, pipe_id: u8, buffer: &[u8]) -> Result<u32> {
        let mut transferred: u32 = 0;
        let ptr = buffer.as_ptr();
        try_unsafe!(self.dev.WritePipe.unwrap()(
            self.handle.unwrap().as_ptr(),
            pipe_id,
            ptr as *mut u8,
            buffer.len() as u32,
            &mut transferred,
            std::ptr::null_mut()
        ));
        return Ok(transferred);
    }

    pub fn raw_handle(&self) -> NonNull<c_void> {
        self.handle.unwrap()
    }
}

impl Drop for DeviceHandle {
    fn drop(&mut self) {
        if let Some(handle) = self.handle {
            for i in &self.claimed_interface {
                unsafe { UsbK_ReleaseInterface(handle.as_ptr(), i.0, i.1 as i32) };
            }
            unsafe { self.dev.Free.unwrap()(handle.as_ptr()) };
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DriverId {
    LibUsbK = 0,
    LibUsb0 = 1,
    WinUsb = 2,
    LibUsb0Filter = 3,
    Count = 4,
}

impl From<i32> for DriverId {
    fn from(num: i32) -> Self {
        use DriverId::*;
        match num {
            0 => LibUsbK,
            1 => LibUsb0,
            2 => WinUsb,
            3 => LibUsb0Filter,
            4 => Count,
            _ => panic!("impossible"),
        }
    }
}

impl Display for DriverId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DriverId::LibUsbK => write!(f, "libusbK"),
            DriverId::LibUsb0 => write!(f, "libusb0"),
            DriverId::WinUsb => write!(f, "winusb"),
            DriverId::LibUsb0Filter => write!(f, "libusb0 filter"),
            DriverId::Count => write!(f, "count"),
        }
    }
}
