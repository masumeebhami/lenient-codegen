use serde::de::Deserializer;
use serde::Deserialize;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Default)]
pub struct Lenient<T, const E: bool = true>(pub T);

impl<'de, T, const E: bool> Deserialize<'de> for Lenient<T, E>
where
    T: Deserialize<'de> + Default + std::fmt::Debug,
{
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match T::deserialize(de) {
            Ok(v) => Ok(Lenient(v)),
            Err(e) => {
                let def = T::default();
                if E {
                    tracing::error!(
                        "⚠️ [Lenient] Error while deserializing: {e}, using Default({def:?})"
                    );
                }
                Ok(Lenient(def))
            }
        }
    }
}

impl<T, const E: bool> Deref for Lenient<T, E> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const E: bool> DerefMut for Lenient<T, E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub type Optional<T, const E: bool = true> = Lenient<Option<T>, E>;

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Default, PartialEq, Deserialize)]
    struct MyType {
        value: usize,
    }

    #[test]
    fn test_lenient_valid() {
        let json = r#"{ "value": 42 }"#;
        let result: Lenient<MyType> = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, 42);
    }

    #[test]
    fn test_lenient_invalid() {
        let json = r#"{ "value": "oops" }"#;
        let result: Lenient<MyType> = serde_json::from_str(json).unwrap();
        assert_eq!(result.value, 0); // fallback to default
    }

    #[test]
    fn test_deref() {
        let wrapper: Lenient<MyType, true> = Lenient(MyType { value: 100 });
        assert_eq!(wrapper.value, 100);
        assert_eq!(wrapper.deref().value, 100);
    }

    #[test]
    fn test_optional_valid() {
        let json = r#"{ "value": 7 }"#;
        let result: Optional<MyType> = serde_json::from_str(json).unwrap();
        assert_eq!(result.0.unwrap().value, 7);
    }

    #[test]
    fn test_optional_invalid() {
        let json = r#"{ "value": false }"#;
        let result: Optional<MyType> = serde_json::from_str(json).unwrap();
        assert_eq!(result.0, None);
    }
}
