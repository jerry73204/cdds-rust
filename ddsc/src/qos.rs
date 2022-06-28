use anyhow::bail;
use anyhow::Result;
use libddsc_sys as sys;
use std::{ffi::CString, os::raw::c_char};

use crate::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "with-serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case", tag = "type")
)]
pub enum Reliability {
    BestEffort,
    Reliable { max_blocking_time: Duration },
}

impl Reliability {
    pub fn from_raw_parts(
        kind: sys::dds_reliability_kind,
        t: Option<sys::dds_duration_t>,
    ) -> Result<Self> {
        use sys::dds_reliability_kind as R;

        let history = match (kind, t) {
            (R::DDS_RELIABILITY_RELIABLE, Some(t)) => Self::Reliable {
                max_blocking_time: Duration::from_raw(t),
            },
            (R::DDS_RELIABILITY_BEST_EFFORT, None) => Self::BestEffort,
            _ => bail!("invalid arguments"),
        };

        Ok(history)
    }

    pub fn to_raw_parts(&self) -> (sys::dds_reliability_kind, Option<sys::dds_duration_t>) {
        use sys::dds_reliability_kind as R;

        match *self {
            Self::Reliable {
                max_blocking_time: t,
            } => (R::DDS_RELIABILITY_RELIABLE, Some(t.to_raw())),
            Self::BestEffort => (R::DDS_RELIABILITY_BEST_EFFORT, None),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "with-serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case", tag = "type")
)]
pub enum History {
    KeepLast { n: usize },
    KeepAll,
}

impl History {
    pub fn from_raw(kind: sys::dds_history_kind, n: Option<usize>) -> Result<Self> {
        use sys::dds_history_kind as H;

        let history = match (kind, n) {
            (H::DDS_HISTORY_KEEP_LAST, Some(n)) => Self::KeepLast { n },
            (H::DDS_HISTORY_KEEP_ALL, None) => Self::KeepAll,
            _ => bail!("invalid arguments"),
        };

        Ok(history)
    }

    pub fn to_raw(&self) -> (sys::dds_history_kind, Option<usize>) {
        use sys::dds_history_kind as H;

        match *self {
            History::KeepLast { n } => (H::DDS_HISTORY_KEEP_LAST, Some(n)),
            History::KeepAll => (H::DDS_HISTORY_KEEP_ALL, None),
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "with-serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
pub enum Durability {
    Volatile = sys::dds_durability_kind::DDS_DURABILITY_VOLATILE.0,
    TransientLocal = sys::dds_durability_kind::DDS_DURABILITY_TRANSIENT_LOCAL.0,
    Transient = sys::dds_durability_kind::DDS_DURABILITY_TRANSIENT.0,
    Persistent = sys::dds_durability_kind::DDS_DURABILITY_PERSISTENT.0,
}

#[derive(Debug)]
pub struct QoS {
    pub(crate) ptr: *mut sys::dds_qos_t,
}

unsafe impl Send for QoS {}

unsafe impl Sync for QoS {}

impl QoS {
    pub fn reset(&mut self) {
        unsafe { sys::dds_qos_reset(self.ptr) }
    }

    pub fn history(&mut self, h: History) -> &mut Self {
        let (kind, n) = h.to_raw();
        let n = n.unwrap_or(0) as i32;

        unsafe {
            sys::dds_qset_history(self.ptr, kind, n);
        }

        self
    }

    pub fn durability(&mut self, d: Durability) -> &mut Self {
        let dur = sys::dds_durability_kind(d as u32);

        unsafe {
            sys::dds_qset_durability(self.ptr, dur);
        }

        self
    }

    pub fn reliability(&mut self, r: Reliability) -> &mut Self {
        let (rel, t) = r.to_raw_parts();
        let t = t.unwrap_or(sys::DDS_INFINITY);
        unsafe {
            sys::dds_qset_reliability(self.ptr, rel, t);
        }

        self
    }

    pub fn partitions<S>(&mut self, ps: &[S]) -> &mut Self
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
                self.ptr,
                ps.len() as u32,
                cps.as_mut_ptr() as *mut *const c_char,
            )
        }

        // deallocate Vec<CString>
        cps.into_iter().for_each(|raw| unsafe {
            let _ = CString::from_raw(raw as *mut c_char);
        });

        self
    }
}

impl Default for QoS {
    fn default() -> QoS {
        QoS {
            ptr: unsafe { sys::dds_create_qos() },
        }
    }
}

impl PartialEq for QoS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sys::dds_qos_equal(self.ptr, other.ptr) }
    }
}

impl Eq for QoS {}

impl Clone for QoS {
    fn clone(&self) -> Self {
        let dst = QoS {
            ptr: unsafe { sys::dds_create_qos() },
        };
        unsafe { sys::dds_copy_qos(dst.ptr, self.ptr as *const sys::dds_qos_t) };
        dst
    }
}

impl Drop for QoS {
    fn drop(&mut self) {
        unsafe { sys::dds_qos_delete(self.ptr) };
    }
}
