use std::{
    ops::{Deref, DerefMut},
    ptr
};
use x11::{
    xlib::{Display, XCloseDisplay, XOpenDisplay, XrmDatabase, XrmValue, XResourceManagerString, XrmGetResource, XrmGetStringDatabase, XrmDestroyDatabase},
};
use std::ffi::{CString, CStr};
use libc;
use scopeguard::defer;

#[derive(Copy, Clone, Debug)]
pub struct XDisplayError;

/// A wrapper around x11::xlib::Display.
pub struct XDisplay(*mut Display);
impl XDisplay {
    pub fn new() -> Result<Self, XDisplayError> {
        unsafe {
            let ptr = XOpenDisplay(ptr::null());
            if ptr.is_null() {
                Err(XDisplayError)
            } else {
                Ok(XDisplay(ptr))
            }
        }
    }
}
impl Deref for XDisplay {
    type Target = *mut Display;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Drop for XDisplay {
    fn drop(&mut self) {
        unsafe {
            XCloseDisplay(self.0);
        }
    }
}

/// Return the value corresponding to the class-name pair from the XrmDatabase if found.
/// This function provides low-level, **unsafe** access to XrmDatabase.
///
/// See [query_xrdb] for safe, single-element queries.
pub unsafe fn get_xrm_resource<'a>(db: XrmDatabase, class: &'a str, name: &'a str) -> Option<&'a str> {
    let mut value = XrmValue {
        size: 0,
        addr: std::ptr::null_mut(),
    };

    let mut value_type: *mut libc::c_char = std::ptr::null_mut();
    let name_c_str = CString::new(name).unwrap();
    let c_str = CString::new(class).unwrap();
    if XrmGetResource(
        db,
        name_c_str.as_ptr(),
        c_str.as_ptr(),
        &mut value_type,
        &mut value
    ) != 0 && !value.addr.is_null() {
        let value_addr: &CStr = CStr::from_ptr(value.addr);
        value_addr.to_str().ok()
    } else {
        None
    }
}

/// Return the value corresponding to the class-name pair from the XrmDatabase if found.
/// This function provides high-level access to the database via the given display.
pub fn query_xrdb<'a>(display: *mut Display, class: &'a str, name: &'a str) -> Option<&'a str> {
    unsafe {
        let rms = XResourceManagerString(display);
        if !rms.is_null() {
            let db = XrmGetStringDatabase(rms);
            if !db.is_null() {
                defer!({
                    XrmDestroyDatabase(db);
                });
                return get_xrm_resource(db, class, name);
            }
        }
    }
    None
}
