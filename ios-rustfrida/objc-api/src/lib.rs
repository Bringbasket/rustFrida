use common::Result;

#[derive(Debug, Clone)]
pub struct ObjcApi;

impl Default for ObjcApi {
    fn default() -> Self {
        Self::new()
    }
}

impl ObjcApi {
    pub fn new() -> Self {
        Self
    }

    pub fn is_available(&self) -> bool {
        cfg!(any(target_os = "ios", target_os = "macos"))
    }

    pub fn class_exists(&self, name: &str) -> bool {
        platform::class_exists(name)
    }

    pub fn enumerate_classes(&self) -> Result<Vec<String>> {
        platform::enumerate_classes()
    }

    pub fn selector(&self, name: &str) -> Result<usize> {
        platform::selector(name)
    }
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
mod platform {
    use std::ffi::{CStr, CString};
    use std::os::raw::{c_char, c_void};

    use common::{Error, Result};

    extern "C" {
        fn objc_getClass(name: *const c_char) -> *mut c_void;
        fn objc_copyClassList(out_count: *mut u32) -> *mut *mut c_void;
        fn class_getName(cls: *const c_void) -> *const c_char;
        fn sel_registerName(name: *const c_char) -> *const c_void;
    }

    pub fn class_exists(name: &str) -> bool {
        let Ok(name) = CString::new(name) else {
            return false;
        };
        unsafe { !objc_getClass(name.as_ptr()).is_null() }
    }

    pub fn enumerate_classes() -> Result<Vec<String>> {
        let mut count = 0u32;
        let list = unsafe { objc_copyClassList(&mut count as *mut u32) };
        if list.is_null() {
            return Ok(Vec::new());
        }

        let slice = unsafe { std::slice::from_raw_parts(list, count as usize) };
        let mut classes = Vec::with_capacity(slice.len());
        for cls in slice {
            let name = unsafe { class_getName(*cls as *const c_void) };
            if name.is_null() {
                continue;
            }
            classes.push(unsafe { CStr::from_ptr(name) }.to_string_lossy().into_owned());
        }
        unsafe { libc::free(list.cast()) };
        classes.sort();
        Ok(classes)
    }

    pub fn selector(name: &str) -> Result<usize> {
        let name = CString::new(name)
            .map_err(|_| Error::InvalidArgument("selector contains interior NUL".into()))?;
        let ptr = unsafe { sel_registerName(name.as_ptr()) };
        Ok(ptr as usize)
    }
}

#[cfg(not(any(target_os = "ios", target_os = "macos")))]
mod platform {
    use common::Result;

    pub fn class_exists(_name: &str) -> bool {
        false
    }

    pub fn enumerate_classes() -> Result<Vec<String>> {
        Err(common::Error::Unsupported(
            "Objective-C runtime is only available on Apple targets".into(),
        ))
    }

    pub fn selector(_name: &str) -> Result<usize> {
        Err(common::Error::Unsupported(
            "Objective-C runtime is only available on Apple targets".into(),
        ))
    }
}
