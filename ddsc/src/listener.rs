use std::ptr;

use libddsc_sys as sys;

#[derive(Debug)]
pub struct Listener {
    pub(crate) ptr: *mut sys::dds_listener_t,
}

impl Listener {
    pub fn merge(&mut self, other: Self) {
        unsafe { sys::dds_listener_merge(self.ptr, other.ptr) }
    }

    pub fn reset(&mut self) {
        unsafe { sys::dds_listener_reset(self.ptr) }
    }
}

unsafe impl Send for Listener {}

unsafe impl Sync for Listener {}

impl Default for Listener {
    fn default() -> Listener {
        Listener {
            ptr: unsafe { sys::dds_listener_create(ptr::null_mut()) },
        }
    }
}

impl Clone for Listener {
    fn clone(&self) -> Self {
        let dst = Listener {
            ptr: unsafe { sys::dds_listener_create(ptr::null_mut()) },
        };
        unsafe { sys::dds_listener_copy(dst.ptr, self.ptr as *const sys::dds_listener_t) };
        dst
    }
}

impl Drop for Listener {
    fn drop(&mut self) {
        unsafe { sys::dds_listener_delete(self.ptr) };
    }
}
