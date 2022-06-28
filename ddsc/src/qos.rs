use libddsc_sys as sys;
use std::{ffi::CString, os::raw::c_char};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    features = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case", tag = "type")
)]
pub enum History {
    KeepLast { n: u32 },
    KeepAll,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    features = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum Durability {
    Volatile = sys::dds_durability_kind_DDS_DURABILITY_VOLATILE,
    TransientLocal = sys::dds_durability_kind_DDS_DURABILITY_TRANSIENT_LOCAL,
    Transient = sys::dds_durability_kind_DDS_DURABILITY_TRANSIENT,
    Persistent = sys::dds_durability_kind_DDS_DURABILITY_PERSISTENT,
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

    pub fn history(&mut self, h: History) {
        let (kind, n) = match h {
            History::KeepLast { n } => (sys::dds_history_kind_DDS_HISTORY_KEEP_LAST, n as i32),
            History::KeepAll => (sys::dds_history_kind_DDS_HISTORY_KEEP_ALL, 0),
        };

        unsafe {
            sys::dds_qset_history(self.qos, kind, n);
        }
    }

    pub fn durability(&mut self, d: Durability) {
        unsafe {
            sys::dds_qset_durability(self.qos, d as u32);
        }
    }

    pub fn partitions<S>(&mut self, ps: &[S])
    where
        S: AsRef<str>,
    {
        let mut cps: Vec<*const c_char> = ps
            .iter()
            .map(|s| {
                let s = s.as_ref();
                CString::new(s)
                    .unwrap_or_else(|_| {
                        panic!("unable to convert the partition name '{}' to a C string", s)
                    })
                    .into_raw() as *const c_char
            })
            .collect();
        unsafe {
            sys::dds_qset_partition(
                self.qos,
                ps.len() as u32,
                cps.as_mut_ptr() as *mut *const c_char,
            )
        }

        // deallocate Vec<CString>
        cps.into_iter().for_each(|raw| unsafe {
            let _ = CString::from_raw(raw as *mut c_char);
        });
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
