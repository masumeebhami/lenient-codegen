use lenient::LenientDeserialize;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Default)]
struct From(pub usize);
impl<'de> Deserialize<'de> for From {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let val = usize::deserialize(de)?;
        Ok(From(val))
    }
}

#[derive(Debug, Default)]
struct Size(pub usize);
impl<'de> Deserialize<'de> for Size {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
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
}

fn main() {
    let test_cases = vec![
        ("Empty input", json!({})),
        ("Valid input", json!({ "from": 5, "size": 10 })),
        ("Invalid 'from'", json!({ "from": "invalid", "size": 10 })),
        ("Invalid 'size'", json!({ "from": 3, "size": "oops" })),
        ("Both invalid", json!({ "from": "?", "size": [] })),
    ];

    for (desc, input) in test_cases {
        println!("\n== {desc} ==\nInput: {input}");
        match serde_json::from_value::<Offset>(input) {
            Ok(offset) => println!("Deserialized: {:?}", offset),
            Err(e) => println!("❌ Error: {e}"),
        }
    }
}