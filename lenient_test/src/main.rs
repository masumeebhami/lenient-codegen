use lenient_derive::LenientDeserialize;
use serde::Deserialize;
use serde_json::json;

// Ensure `Lenient` and `Optional` are in scope to allow #[lenient] and #[optional] attributes
#[allow(unused_imports)]
use lenient::{Lenient, Optional};

#[derive(Debug, Default)]
struct From(pub usize);
impl<'de> Deserialize<'de> for From {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val = usize::deserialize(de)?;
        Ok(From(val))
    }
}

#[derive(Debug, Default)]
struct Size(pub usize);
impl<'de> Deserialize<'de> for Size {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val = usize::deserialize(de)?;
        Ok(Size(val))
    }
}

#[derive(Debug, Default, LenientDeserialize)]
struct Offset {
    #[lenient]
    from: From,
    #[optional]
    size: Size,
    #[lenient]
    label: String,
}

fn main() {
    let test_cases = vec![
        ("Empty input", json!({})),
        ("Valid input", json!({ "from": 5, "size": 10, "label": "ok" })),
        ("Invalid 'from'", json!({ "from": "invalid", "size": 10 })),
        ("Invalid 'size'", json!({ "from": 3, "size": "oops", "label": "label" })),
        ("Invalid all", json!({ "from": "?", "size": [], "label": 55 })),
        ("Missing all", json!({})),
    ];

    for (desc, input) in test_cases {
        println!("
== {desc} ==
Input: {input}");
        match serde_json::from_value::<Offset>(input) {
            Ok(offset) => {
                // Test Deref access (From is Deref'd via Lenient)
                let from_val: usize = offset.from.0;
                let size_val: usize = offset.size.0;
                println!("Deserialized: Offset {{ from: {from_val}, size: {size_val}, label: {:?} }}", &offset.label);
            }
            Err(e) => println!("❌ Error: {e}"),
        }
    }
}