use crate::{dds_domainid_t, dds_duration_t, dds_entity_t, dds_time_t};

pub const DDS_BUILTIN_TOPIC_DCPSPARTICIPANT: dds_entity_t =
    (DDS_MIN_PSEUDO_HANDLE + 1) as dds_entity_t;
pub const DDS_BUILTIN_TOPIC_DCPSTOPIC: dds_entity_t = (DDS_MIN_PSEUDO_HANDLE + 2) as dds_entity_t;
pub const DDS_BUILTIN_TOPIC_DCPSPUBLICATION: dds_entity_t =
    (DDS_MIN_PSEUDO_HANDLE + 3) as dds_entity_t;
pub const DDS_BUILTIN_TOPIC_DCPSSUBSCRIPTION: dds_entity_t =
    (DDS_MIN_PSEUDO_HANDLE + 4) as dds_entity_t;

pub const DDS_MIN_PSEUDO_HANDLE: dds_entity_t = 0x7fff0000 as dds_entity_t;

/** Special handle representing the entity corresponding to the CycloneDDS library itself */
pub const DDS_CYCLONEDDS_HANDLE: dds_entity_t = (DDS_MIN_PSEUDO_HANDLE + 256) as dds_entity_t;

pub const DDS_DOMAIN_DEFAULT: dds_domainid_t = 0xffffffff as dds_domainid_t;

pub const DDS_NEVER: dds_time_t = i64::MAX as dds_time_t;

pub const DDS_INFINITY: dds_duration_t = i64::MAX as dds_duration_t;
pub const DDS_DURATION_INVALID: dds_duration_t = i64::MIN as dds_duration_t;
