use std::fmt;

use libusbk_sys::{LibK_GetVersion, KLIB_VERSION};

/// A structure that describes the version of the underlying `libusbK` library.
#[derive(Clone, Copy)]
pub struct LibraryVersion {
    inner: KLIB_VERSION,
}

impl LibraryVersion {
    pub fn new() -> Self {
        let mut version = KLIB_VERSION::default();
        unsafe { LibK_GetVersion(&mut version) }
        Self { inner: version }
    }

    pub fn major(&self) -> i32 {
        self.inner.Major
    }

    pub fn minor(&self) -> i32 {
        self.inner.Minor
    } 

    pub fn micro(&self) -> i32 {
        self.inner.Micro
    } 

    pub fn nano(&self) -> i32 {
        self.inner.Nano
    }
}

impl fmt::Debug for LibraryVersion {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut debug = fmt.debug_struct("LibraryVersion");

        debug.field("major", &self.major());
        debug.field("minor", &self.minor());
        debug.field("micro", &self.micro());
        debug.field("nano", &self.nano());

        debug.finish()
    }
}

impl Default for LibraryVersion {
   fn default() -> Self {
        Self::new()
    }
}

pub fn version() -> LibraryVersion {
    LibraryVersion::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_version() {
        let v = version();
        assert_ne!(v.major(), 0)
    }
}