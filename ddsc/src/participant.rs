use libddsc_sys as sys;
use std::ptr;

use crate::{error::retcode_to_result, Error, QoS};

#[derive(Debug)]
pub struct Participant {
    pub(crate) entity: sys::dds_entity_t,
}

impl Drop for Participant {
    fn drop(&mut self) {
        unsafe { sys::dds_delete(self.entity) };
    }
}

impl Participant {
    pub fn new(domain: Option<usize>, qos: Option<&QoS>) -> Result<Self, Error> {
        let domain = domain
            .map(|d| d as sys::dds_domainid_t)
            .unwrap_or(sys::DDS_DOMAIN_DEFAULT);
        let qos_ptr = qos.map(|q| q.ptr as *const _).unwrap_or(ptr::null());
        let listener_ptr = ptr::null();
        let retcode = unsafe { sys::dds_create_participant(domain, qos_ptr, listener_ptr) };
        let id = retcode_to_result(retcode)?;

        Ok(Participant { entity: id })
    }
}
