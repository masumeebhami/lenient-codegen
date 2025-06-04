use lenient_derive::LenientDeserialize;
use serde::Deserialize;
use serde_json::json;

// Ensure `Lenient` and `Optional` are in scope to allow #[lenient] and #[optional] attributes
#[allow(unused_imports)]
use lenient::{Lenient, Optional};

#[derive(Debug, Default)]
struct Age(pub u32);
impl<'de> Deserialize<'de> for Age {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val = u32::deserialize(de)?;
        Ok(Age(val))
    }
}

#[derive(Debug, Default)]
struct Score(pub u8);
impl<'de> Deserialize<'de> for Score {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val = u8::deserialize(de)?;
        Ok(Score(val))
    }
}

#[derive(Debug, Default, LenientDeserialize)]
struct UserProfile {
    #[lenient]
    age: Age,
    #[optional]
    score: Score,
    #[lenient]
    nickname: String,
}

fn main() {
    let test_cases = vec![
        ("Empty input", json!({})),
        (
            "Valid input",
            json!({ "age": 30, "score": 88, "nickname": "Alice" }),
        ),
        (
            "Invalid age",
            json!({ "age": "old", "score": 88, "nickname": "Bob" }),
        ),
        (
            "Invalid score",
            json!({ "age": 22, "score": "excellent", "nickname": "Charlie" }),
        ),
        (
            "Invalid all",
            json!({ "age": [], "score": {}, "nickname": 404 }),
        ),
    ];

    for (desc, input) in test_cases {
        println!("\n== {desc} ==\nInput: {input}");
        match serde_json::from_value::<UserProfile>(input) {
            Ok(profile) => {
                let age_val = profile.age.0;
                let score_val = profile.score.0;
                println!(
                    "Deserialized: UserProfile {{ age: {age_val}, score: {score_val}, nickname: {:?} }}",
                    &profile.nickname
                );
            }
            Err(e) => println!("❌ Error: {e}"),
        }
    }
}
