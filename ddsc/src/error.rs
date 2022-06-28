use std::{error, fmt};

use libddsc_sys as sys;

const ERROR_CODES: [i32; 13] = [
    sys::DDS_RETCODE_ALREADY_DELETED,
    sys::DDS_RETCODE_BAD_PARAMETER,
    sys::DDS_RETCODE_ERROR,
    sys::DDS_RETCODE_ILLEGAL_OPERATION,
    sys::DDS_RETCODE_IMMUTABLE_POLICY,
    sys::DDS_RETCODE_INCONSISTENT_POLICY,
    sys::DDS_RETCODE_NOT_ALLOWED_BY_SECURITY,
    sys::DDS_RETCODE_NOT_ENABLED,
    sys::DDS_RETCODE_NO_DATA,
    sys::DDS_RETCODE_OUT_OF_RESOURCES,
    sys::DDS_RETCODE_PRECONDITION_NOT_MET,
    sys::DDS_RETCODE_TIMEOUT,
    sys::DDS_RETCODE_UNSUPPORTED,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Error {
    code: i32,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cyclone DDS error: code={}", self.code)
    }
}

impl error::Error for Error {}

pub(crate) fn retcode_to_result(code: i32) -> Result<i32, Error> {
    if ERROR_CODES.contains(&code) {
        Err(Error { code })
    } else {
        Ok(code)
    }
}
