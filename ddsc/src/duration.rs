use libddsc_sys as sys;
use std::{cmp::Ordering, cmp::Ordering::*, fmt, time};

const INFINITE_TEXT: &str = "infinite";
const INVALID_TEXT: &str = "invalid";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Duration {
    Invalid,
    Infinite,
    Finite(chrono::Duration),
}

impl Duration {
    pub fn from_raw(dur: sys::dds_duration_t) -> Self {
        if dur == sys::DDS_DURATION_INVALID {
            Self::Invalid
        } else if dur == sys::DDS_INFINITY {
            Self::Infinite
        } else {
            let dur = chrono::Duration::nanoseconds(dur as i64);
            Self::Finite(dur)
        }
    }

    pub fn to_raw(&self) -> sys::dds_duration_t {
        match self {
            Duration::Invalid => sys::DDS_DURATION_INVALID,
            Duration::Infinite => sys::DDS_INFINITY,
            Duration::Finite(dur) => dur.num_nanoseconds().unwrap() as sys::dds_duration_t,
        }
    }

    pub fn infinite() -> Self {
        Self::Infinite
    }

    pub fn invalid() -> Self {
        Self::Invalid
    }

    pub fn from_nanos(nanos: i64) -> Self {
        chrono::Duration::nanoseconds(nanos).into()
    }

    pub fn from_micros(micros: i64) -> Self {
        chrono::Duration::microseconds(micros).into()
    }

    pub fn from_millis(millis: i64) -> Self {
        chrono::Duration::milliseconds(millis).into()
    }

    pub fn from_secs(secs: i64) -> Self {
        chrono::Duration::seconds(secs).into()
    }

    pub fn from_minutes(mins: i64) -> Self {
        chrono::Duration::minutes(mins).into()
    }

    pub fn from_hours(hrs: i64) -> Self {
        chrono::Duration::hours(hrs).into()
    }

    /// Returns `true` if the duration is [`Finite`].
    ///
    /// [`Finite`]: Duration::Finite
    #[must_use]
    pub fn is_finite(&self) -> bool {
        matches!(self, Self::Finite(..))
    }

    /// Returns `true` if the duration is [`Infinite`].
    ///
    /// [`Infinite`]: Duration::Infinite
    #[must_use]
    pub fn is_infinite(&self) -> bool {
        matches!(self, Self::Infinite)
    }

    /// Returns `true` if the duration is [`Invalid`].
    ///
    /// [`Invalid`]: Duration::Invalid
    #[must_use]
    pub fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid)
    }
}

impl PartialOrd for Duration {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Duration as D;

        Some(match (self, other) {
            (D::Invalid, _) | (_, D::Invalid) => return None,
            (D::Finite(lhs), D::Finite(rhs)) => lhs.partial_cmp(rhs)?,
            (D::Finite(_), D::Infinite) => Less,
            (D::Infinite, D::Finite(_)) => Greater,
            (D::Infinite, D::Infinite) => Equal,
        })
    }
}

impl From<time::Duration> for Duration {
    fn from(v: time::Duration) -> Self {
        Self::Finite(chrono::Duration::from_std(v).unwrap())
    }
}

impl From<chrono::Duration> for Duration {
    fn from(v: chrono::Duration) -> Self {
        Self::Finite(v)
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Duration::Invalid => write!(f, "{}", INVALID_TEXT),
            Duration::Infinite => write!(f, "{}", INFINITE_TEXT),
            Duration::Finite(dur) => match dur.to_std() {
                Ok(dur) => {
                    write!(f, "{}", humantime::format_duration(dur))
                }
                Err(_) => {
                    let pos_dur = (-dur).to_std().unwrap();
                    write!(f, "-{}", humantime::format_duration(pos_dur))
                }
            },
        }
    }
}

#[cfg(feature = "with-serde")]
mod with_serde {
    use std::borrow::Cow;

    use super::*;
    use serde::de::Error as _;
    use serde::Deserialize;
    use serde::Deserializer;
    use serde::Serialize;
    use serde::Serializer;

    impl Serialize for Duration {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let text: Cow<'_, str> = match *self {
                Duration::Invalid => INVALID_TEXT.into(),
                Self::Finite(dur) => {
                    if let Ok(dur) = dur.to_std() {
                        humantime::format_duration(dur).to_string().into()
                    } else {
                        todo!();
                    }
                }
                Self::Infinite => INFINITE_TEXT.into(),
            };
            text.serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for Duration {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let create_error = |text, err| {
                D::Error::custom(format!("unable to parse duration '{}': {:#}", text, err))
            };

            let text = String::deserialize(deserializer)?;

            let dur = if text == INFINITE_TEXT {
                Self::Infinite
            } else if text == INVALID_TEXT {
                Self::Invalid
            } else if let Some(suffix) = text.strip_prefix('-') {
                let std_dur =
                    humantime::parse_duration(suffix).map_err(|err| create_error(text, err))?;
                let dur = -chrono::Duration::from_std(std_dur).unwrap();
                dur.into()
            } else {
                let dur =
                    humantime::parse_duration(&text).map_err(|err| create_error(text, err))?;
                dur.into()
            };
            Ok(dur)
        }
    }
}
