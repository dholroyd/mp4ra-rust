//! Common record-types used across several MP4RA CSV files

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Record {
    #[serde(with = "code_format")]
    pub code: String,
    pub description: String,
    pub specification: String,
}

#[derive(Debug, Deserialize)]
pub struct TypedRecord {
    #[serde(with = "code_format")]
    pub code: String,
    pub description: String,
    pub r#type: String,
    pub specification: String,
}

/// decode characters encoded as dolar-prefixed hex.
pub(crate) mod code_format {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let re = regex::Regex::new("\\$(\\d\\d)").unwrap();
        Ok(re
            .replace_all(&s, |caps: &regex::Captures| {
                String::from_utf8(vec![u8::from_str_radix(&caps[1], 16).unwrap()])
                    .unwrap_or_else(|_| panic!("invalid {:?}", &caps[0]))
            })
            .to_string())
    }

    #[cfg(test)]
    mod test {
        use serde::de::IntoDeserializer;

        #[test]
        fn test_decode() {
            let v: Result<String, serde::de::value::Error> =
                super::deserialize("A$20$42$43".into_deserializer());
            assert!(matches!(v, Ok(s) if s=="A BC"));
        }
    }
}
