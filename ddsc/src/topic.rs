use libddsc_sys as sys;
use std::{ffi::CString, ptr};

use crate::{retcode_to_result, Error, Participant, QoS};

#[derive(Debug)]
pub struct Topic {
    entity: sys::dds_entity_t,
}

impl Drop for Topic {
    fn drop(&mut self) {
        unsafe { sys::dds_delete(self.entity) };
    }
}

impl Topic {
    pub fn new(
        participant: &Participant,
        descriptor: *const sys::dds_topic_descriptor_t,
        name: &str,
        qos: Option<&QoS>,
    ) -> Result<Self, Error> {
        let qos_ptr = qos.map(|q| q.ptr as *const _).unwrap_or(ptr::null());
        let listener_ptr = ptr::null();
        let name_ptr = CString::new(name).unwrap().into_raw();

        let retcode = unsafe {
            sys::dds_create_topic(
                participant.entity,
                descriptor,
                name_ptr,
                qos_ptr,
                listener_ptr,
            )
        };
        let id = retcode_to_result(retcode)?;

        let _ = unsafe { CString::from_raw(name_ptr) };

        Ok(Self { entity: id })
    }
}
