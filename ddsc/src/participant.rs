use std::ptr;

use libddsc_sys as sys;

#[derive(Debug)]
pub struct Participant {
    entity: sys::dds_entity_t,
}

impl Drop for Participant {
    fn drop(&mut self) {
        unsafe { sys::dds_delete(self.entity) };
    }
}

impl Participant {
    pub fn new(d: sys::dds_domainid_t) -> Participant {
        let e = unsafe { sys::dds_create_participant(d, ptr::null(), ptr::null()) };
        Participant { entity: e }
    }
}
