use serde::{Deserialize, Serialize};
use serde::{Deserializer, Serializer};

/// Duration is a wrapper around chrono::Duration that Serializes into and Deserializes from
/// millisecond precision integers.
#[derive(Debug, Clone, PartialEq)]
pub struct Duration(chrono::Duration);

// Allow Duration to be used as a chrono::Duration.
impl std::ops::Deref for Duration {
    type Target = chrono::Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de> Deserialize<'de> for Duration {
    fn deserialize<D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let u = i64::deserialize(deserializer)?;
        Ok(Duration(chrono::Duration::milliseconds(u)))
    }
}

impl Serialize for Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(self.0.num_milliseconds())
    }
}

#[cfg(test)]
mod test {
    use super::Duration;
    use serde_json;

    #[test]
    fn test_serialize_duration() {
        let ms = 86400000i64;
        let d = Duration(chrono::Duration::milliseconds(ms));
        assert_eq!(serde_json::to_string(&d).unwrap(), ms.to_string());
    }

    #[test]
    fn test_deserialize_duration() {
        let ms = 86400000i64;
        let d: Duration = serde_json::from_str(&ms.to_string()).unwrap();
        assert_eq!(d.num_milliseconds(), ms);
    }
}
