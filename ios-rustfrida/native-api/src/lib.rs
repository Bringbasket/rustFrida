use common::Result;

#[derive(Debug, Clone)]
pub struct ImageInfo {
    pub name: String,
    pub slide: isize,
}

#[derive(Debug, Clone)]
pub struct InjectionTarget {
    pub pid: i32,
    pub dylib_path: String,
    pub entry_symbol: String,
}

#[derive(Debug, Default, Clone)]
pub struct MachInjector;

impl MachInjector {
    pub fn inject(&self, target: &InjectionTarget) -> Result<()> {
        platform::inject(target)
    }
}

pub fn enumerate_images() -> Result<Vec<ImageInfo>> {
    platform::enumerate_images()
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
mod platform {
    use std::ffi::CStr;
    use std::os::raw::c_char;

    use common::Result;

    use crate::{ImageInfo, InjectionTarget};

    extern "C" {
        fn _dyld_image_count() -> u32;
        fn _dyld_get_image_name(index: u32) -> *const c_char;
        fn _dyld_get_image_vmaddr_slide(index: u32) -> isize;
    }

    pub fn enumerate_images() -> Result<Vec<ImageInfo>> {
        let count = unsafe { _dyld_image_count() };
        let mut images = Vec::with_capacity(count as usize);
        for index in 0..count {
            let name_ptr = unsafe { _dyld_get_image_name(index) };
            if name_ptr.is_null() {
                continue;
            }
            let name = unsafe { CStr::from_ptr(name_ptr) }.to_string_lossy().into_owned();
            let slide = unsafe { _dyld_get_image_vmaddr_slide(index) };
            images.push(ImageInfo { name, slide });
        }
        Ok(images)
    }

    pub fn inject(target: &InjectionTarget) -> Result<()> {
        Err(common::Error::Unsupported(format!(
            "remote Mach injection for pid {} is not implemented yet; finish task_for_pid + mach_vm_write + remote pthread bootstrap for {} ({})",
            target.pid, target.entry_symbol, target.dylib_path
        )))
    }
}

#[cfg(not(any(target_os = "ios", target_os = "macos")))]
mod platform {
    use common::Result;

    use crate::{ImageInfo, InjectionTarget};

    pub fn enumerate_images() -> Result<Vec<ImageInfo>> {
        Err(common::Error::Unsupported(
            "dyld image enumeration is only available on Apple targets".into(),
        ))
    }

    pub fn inject(_target: &InjectionTarget) -> Result<()> {
        Err(common::Error::Unsupported(
            "Mach injection is only available on Apple targets".into(),
        ))
    }
}
