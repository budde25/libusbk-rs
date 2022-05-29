use std::ffi::c_void;
use std::sync::Mutex;

use libusbk_sys::{
    HotK_Free, HotK_Init, KHOT_HANDLE, KHOT_PARAMS, KLST_DEVINFO_HANDLE, KLST_SYNC_FLAG,
};
use once_cell::sync::OnceCell;

use crate::device::{self, Device};
use crate::error::{try_unsafe, Result};

static DATA: OnceCell<Mutex<Option<Data>>> = OnceCell::new();

unsafe impl Send for Data {}

struct Data {
    callback: Box<dyn Hotplug>,
    vid: Option<i32>,
    pid: Option<i32>,
}

pub trait Hotplug: Send {
    fn device_arrived(&mut self, device: Device);
    fn device_left(&mut self, device: Device);
}

#[derive(Debug, Copy, Clone)]
pub struct HotplugBuilder {
    vendor_id: Option<i32>,
    product_id: Option<i32>,
    params: KHOT_PARAMS,
}

impl HotplugBuilder {
    pub fn new() -> Self {
        HotplugBuilder::default()
    }

    pub fn vendor_id(&mut self, vendor_id: i32) -> &mut Self {
        self.vendor_id = Some(vendor_id);
        self
    }

    pub fn product_id(&mut self, product_id: i32) -> &mut Self {
        self.product_id = Some(product_id);
        self
    }

    pub fn register(self, callback: Box<dyn Hotplug>) -> crate::Result<Registration> {
        let v = DATA.get_or_init(|| Mutex::new(None));
        let mut item = v.lock().unwrap();
        *item = Some(Data {
            callback,
            vid: self.vendor_id,
            pid: self.product_id,
        });
        Ok(Registration {
            params: self.params,
            handle: 0 as *mut c_void,
        })
    }
}

impl Default for HotplugBuilder {
    fn default() -> Self {
        Self {
            vendor_id: None,
            product_id: None,
            params: libusbk_sys::_KHOT_PARAMS::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NotificationType {
    Arrival,
    Removal,
}

impl From<KLST_SYNC_FLAG> for NotificationType {
    fn from(flag: KLST_SYNC_FLAG) -> Self {
        const ADDED: i32 = 2;

        match flag {
            ADDED => NotificationType::Arrival,
            _ => NotificationType::Removal,
        }
    }
}

pub struct Registration {
    params: KHOT_PARAMS,
    handle: KHOT_HANDLE,
}

impl Registration {
    unsafe extern "C" fn on_hotplug(
        _handle: KHOT_HANDLE,
        device_info: KLST_DEVINFO_HANDLE,
        sync_flag: KLST_SYNC_FLAG,
    ) {
        let mut lock = DATA.get().unwrap().lock().unwrap();
        let data_s = lock.as_mut().unwrap();
        let callback = data_s.callback.as_mut();

        let vid = data_s.vid;
        let pid = data_s.pid;

        let device = device::Device(device_info);

        if let Some(vid) = vid {
            if vid != device.vendor_id() as i32 {
                return;
            }
        }

        if let Some(pid) = pid {
            if pid != device.product_id() as i32 {
                return;
            }
        }

        match sync_flag.into() {
            NotificationType::Arrival => callback.device_arrived(device),
            NotificationType::Removal => callback.device_left(device),
        }
    }

    pub fn init(&mut self) -> Result<()> {
        self.params.OnHotPlug = Some(Self::on_hotplug);

        try_unsafe!(HotK_Init(&mut self.handle, &mut self.params));
        Ok(())
    }
}

impl Drop for Registration {
    fn drop(&mut self) {
        unsafe { HotK_Free(self.handle) };
    }
}

pub fn has_hotplug() -> bool {
    true
}
