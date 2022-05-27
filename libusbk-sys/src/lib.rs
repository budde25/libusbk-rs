#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_version() {
        let mut version = KLIB_VERSION::default();

        unsafe { LibK_GetVersion(&mut version) }
        assert_ne!(version.Major, 0)
    }

    #[test]
    fn get_usb() {
        let mut handle: KLIB_HANDLE = std::ptr::null_mut();
        let flags = 0;

        let ret = unsafe { LstK_Init(&mut handle, flags) };
        assert_ne!(ret, 0);

        let mut device_info: KLST_DEVINFO_HANDLE = std::ptr::null_mut();
        let _ = unsafe { LstK_FindByVidPid(handle, 0x0955, 0x7321, &mut device_info) };
        let ret = unsafe { LstK_Free(handle) };
        assert_ne!(ret, 0);

        if device_info != std::ptr::null_mut() {
            dbg!(unsafe { *device_info });
        }
    }

    #[test]
    fn get_usb_count() {
        let mut handle: KLIB_HANDLE = std::ptr::null_mut();
        let flags = 0;

        let ret = unsafe { LstK_Init(&mut handle, flags) };
        assert_ne!(ret, 0);

        let mut count = 0;
        let ret = unsafe { LstK_Count(handle, &mut count) };

        assert_ne!(ret, 0);

        let ret = unsafe { LstK_Free(handle) };
        assert_ne!(ret, 0);

        dbg!(count);
    }
}
