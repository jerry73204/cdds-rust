use libddsc_sys as sys;
use std::{ffi::CString, os::raw::c_char};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum History {
    KeepLast { n: u32 },
    KeepAll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Durability {
    Volatile,
    TransientLocal,
    Transient,
    Persistent,
}

#[derive(Debug)]
pub struct QoS {
    qos: *mut sys::dds_qos_t,
}

unsafe impl Send for QoS {}

unsafe impl Sync for QoS {}

impl QoS {
    pub fn reset(&mut self) {
        unsafe { sys::dds_qos_reset(self.qos) }
    }

    pub fn history(&mut self, h: &History) {
        match h {
            History::KeepLast { n } => unsafe {
                sys::dds_qset_history(
                    self.qos,
                    sys::dds_history_kind_DDS_HISTORY_KEEP_LAST,
                    *n as i32,
                )
            },
            History::KeepAll => unsafe {
                sys::dds_qset_history(self.qos, sys::dds_history_kind_DDS_HISTORY_KEEP_ALL, 0)
            },
        }
    }

    pub fn partitions(&mut self, ps: &[String]) {
        // let mut xs : [*const c_char; ps.len()] = [ std::ptr::null(); ps.len()];
        // let p = CString::new(ps[0]).unwrap().as_ptr();
        let mut cps: Vec<*const c_char> = ps
            .iter()
            .map(|s| CString::new(String::from(s)).unwrap().into_raw() as *const c_char)
            .collect();
        unsafe {
            sys::dds_qset_partition(
                self.qos,
                ps.len() as u32,
                cps.as_mut_ptr() as *mut *const c_char,
            )
        }
    }
}

impl Default for QoS {
    fn default() -> QoS {
        QoS {
            qos: unsafe { sys::dds_create_qos() },
        }
    }
}

impl PartialEq for QoS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sys::dds_qos_equal(self.qos, other.qos) }
    }
}

impl Eq for QoS {}

impl Clone for QoS {
    fn clone(&self) -> Self {
        let dst = QoS {
            qos: unsafe { sys::dds_create_qos() },
        };
        unsafe { sys::dds_copy_qos(dst.qos, self.qos as *const sys::dds_qos_t) };
        dst
    }
}

impl Drop for QoS {
    fn drop(&mut self) {
        unsafe { sys::dds_qos_delete(self.qos) };
    }
}
