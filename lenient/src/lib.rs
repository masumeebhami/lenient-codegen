pub use lenient_derive::LenientDeserialize;

use serde::Deserialize;

#[derive(Debug, Default)]
pub struct Lenient<T, const E: bool = true>(pub T);

impl<'de, T, const E: bool> Deserialize<'de> for Lenient<T, E>
where
    T: Deserialize<'de> + Default + std::fmt::Debug,
{
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        match T::deserialize(de) {
            Ok(v) => Ok(Lenient(v)),
            Err(e) => {
                let def = T::default();
                if E {
                    tracing::error!("⚠️ [Lenient] Error: {e}, defaulting to {def:?}");
                }
                Ok(Lenient(def))
            }
        }
    }
}

pub type Optional<T, const E: bool = true> = Lenient<Option<T>, E>;
