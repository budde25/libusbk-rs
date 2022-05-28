use core::fmt;
use std::collections::HashSet;
use std::ffi::c_void;
use std::mem;
use std::ptr::NonNull;

use libusbk_sys::{UsbK_Init, KLST_DEVINFO};

use crate::error::try_unsafe;
use crate::DeviceHandle;

const STRING_LEN: usize = 256;

// wrapper type
// TODO not pub
#[repr(transparent)]
pub struct Device(pub(crate) *mut KLST_DEVINFO);

impl Device {
    pub fn open(&self) -> crate::Result<DeviceHandle> {
        let mut handle = mem::MaybeUninit::<*mut c_void>::uninit();

        try_unsafe!(UsbK_Init(handle.as_mut_ptr(), self.0));

        let ptr = unsafe { NonNull::new(handle.assume_init()).unwrap() };
        Ok(DeviceHandle {
            driver_id: self.driver_id(),
            handle: Some(ptr),
            claimed_interface: HashSet::new(),
        })
    }

    pub fn driver_id(&self) -> i32 {
        self.inner().DriverID
    }

    pub fn device_interface_guid(&self) -> &str {
        let data = &self.inner().DeviceInterfaceGUID;
        unsafe {
            std::str::from_utf8(std::mem::transmute::<&[i8; STRING_LEN], &[u8; STRING_LEN]>(
                data,
            ))
            .unwrap()
        }
    }

    pub fn device_id(&self) -> &str {
        let data = &self.inner().DeviceID;
        unsafe {
            std::str::from_utf8(std::mem::transmute::<&[i8; STRING_LEN], &[u8; STRING_LEN]>(
                data,
            ))
            .unwrap()
        }
    }

    pub fn class_guid(&self) -> &str {
        let data = &self.inner().ClassGUID;
        unsafe {
            std::str::from_utf8(std::mem::transmute::<&[i8; STRING_LEN], &[u8; STRING_LEN]>(
                data,
            ))
            .unwrap()
        }
    }

    pub fn manufacturer(&self) -> &str {
        let data = &self.inner().Mfg;
        unsafe {
            std::str::from_utf8(std::mem::transmute::<&[i8; STRING_LEN], &[u8; STRING_LEN]>(
                data,
            ))
            .unwrap()
        }
    }

    pub fn device_descriptor(&self) -> &str {
        let data = &self.inner().DeviceDesc;
        unsafe {
            std::str::from_utf8(std::mem::transmute::<&[i8; STRING_LEN], &[u8; STRING_LEN]>(
                data,
            ))
            .unwrap()
        }
    }

    pub fn service(&self) -> &str {
        let data = &self.inner().Service;
        unsafe {
            std::str::from_utf8(std::mem::transmute::<&[i8; STRING_LEN], &[u8; STRING_LEN]>(
                data,
            ))
            .unwrap()
        }
    }

    pub fn symbolic_link(&self) -> &str {
        let data = &self.inner().SymbolicLink;
        unsafe {
            std::str::from_utf8(std::mem::transmute::<&[i8; STRING_LEN], &[u8; STRING_LEN]>(
                data,
            ))
            .unwrap()
        }
    }

    pub fn device_path(&self) -> &str {
        let data = &self.inner().DevicePath;
        unsafe {
            std::str::from_utf8(std::mem::transmute::<&[i8; STRING_LEN], &[u8; STRING_LEN]>(
                data,
            ))
            .unwrap()
        }
    }

    pub fn lusb0_filter_index(&self) -> i32 {
        self.inner().LUsb0FilterIndex
    }

    pub fn connected(&self) -> bool {
        match self.inner().Connected {
            0 => false,
            _ => true,
        }
    }

    pub fn serial_number(&self) -> &str {
        let data = &self.inner().SerialNumber;
        unsafe {
            std::str::from_utf8(std::mem::transmute::<&[i8; STRING_LEN], &[u8; STRING_LEN]>(
                data,
            ))
            .unwrap()
        }
    }

    fn inner(&self) -> &KLST_DEVINFO {
        unsafe { &*self.0 }
    }
}

impl fmt::Debug for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Device")
            .field("driver_id", &self.driver_id())
            .field("device_interface_guid", &self.device_interface_guid())
            .field("device_id", &self.device_id())
            .field("class_guid", &self.class_guid())
            .field("manufacturer", &self.manufacturer())
            .field("device_descriptor", &self.device_descriptor())
            .field("service", &self.service())
            .field("symbolic_link", &self.symbolic_link())
            .field("device_path", &self.device_path())
            .field("lusb0_filter_index", &self.lusb0_filter_index())
            .field("connected", &self.connected())
            .field("serial_number", &self.serial_number())
            .finish()
    }
}
