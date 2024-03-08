use serde::{Deserialize, Deserializer};

pub fn string_or_env<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    let value = String::deserialize(deserializer)?;
    match value.strip_prefix("env=") {
        Some(key) => match std::env::var(key) {
            Ok(val) => {
                if val.is_empty() {
                    Err(D::Error::custom(format!(
                        "Environment variable {} is empty",
                        key
                    )))
                } else {
                    Ok(val)
                }
            }
            Err(_) => Err(D::Error::custom(format!(
                "Environment variable {} not found",
                key
            ))),
        },
        None => Ok(value),
    }
}
