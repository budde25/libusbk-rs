pub use libusbk_sys as ffi;

pub use crate::device::Device;
pub use crate::device_handle::{DeviceHandle, DriverId};
pub use crate::device_list::DeviceList;
pub use crate::error::{Error, Result};
pub use crate::hotplug::{has_hotplug, Hotplug, HotplugBuilder};
pub use crate::version::{version, LibraryVersion};

//mod context;
mod device;
mod device_handle;
mod device_list;
mod error;
mod hotplug;
mod version;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_list() {
        let dl = DeviceList::new().unwrap();
        let _count = dl.length().unwrap();
        // let device = dl.find_with_vid_and_pid(0x0955, 0x7321).unwrap();
        // dbg!(device);
    }
}
