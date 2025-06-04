use lenient::LenientDeserialize;
use serde::Deserialize;

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
    from: From,
    size: Size,
}

fn main() {
    let test_cases = vec![
        ("Empty input", serde_json::json!({})),
        ("Valid input", serde_json::json!({ "from": 5, "size": 10 })),
        (
            "Invalid 'from'",
            serde_json::json!({ "from": "invalid", "size": 10 }),
        ),
        (
            "Invalid 'size'",
            serde_json::json!({ "from": 3, "size": "oops" }),
        ),
        (
            "Both invalid",
            serde_json::json!({ "from": "?", "size": [] }),
        ),
        (
            "Partial type mismatch",
            serde_json::json!({ "from": true, "size": 100 }),
        ),
    ];

    for (desc, input) in test_cases {
        println!("\n== {desc} ==\nInput: {input}");
        match serde_json::from_value::<Offset>(input) {
            Ok(offset) => println!("Deserialized: {:?}", offset),
            Err(e) => println!("❌ Error: {e}"),
        }
    }
}
