# 🧱 Lenient Deserialize

A Rust crate for **fault-tolerant deserialization** using Serde. Automatically wraps fields to ensure invalid values fall back to sensible defaults — instead of breaking the entire deserialization process.

This is useful when handling user inputs, external APIs, or anything where input reliability is uncertain.

---

## ✨ Features

- `Lenient<T>`: wraps any type to gracefully fallback to `T::default()` on deserialization failure.
- `Optional<T>`: alias for `Lenient<Option<T>>`.
- `#[derive(LenientDeserialize)]`: a procedural macro to generate fault-tolerant wrappers for entire structs.
- Support for field-level `#[lenient]` and `#[optional]` attributes.
- Optional error logging via [`tracing`](https://docs.rs/tracing).
- Ergonomic access with `Deref` and `DerefMut` on `Lenient<T>`.

---

## 📦 Crate Structure

This workspace contains two crates:

```
lenient/
├── lenient/            # Main library (Lenient wrapper, re-exports macro)
├── lenient_derive/     # Procedural macro crate (LenientDeserialize)
```

---

## 🚀 Quick Start

### 1. Add Dependencies

In your `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tracing = "0.1"
lenient = { path = "./lenient" }
```

---

### 2. Example: Lenient Field Wrapping

```rust
use lenient::LenientDeserialize;
use serde::Deserialize;

#[derive(Debug, Default)]
struct Age(pub u32);
impl<'de> Deserialize<'de> for Age {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let val = u32::deserialize(de)?;
        Ok(Age(val))
    }
}

#[derive(Debug, Default)]
struct Score(pub u8);
impl<'de> Deserialize<'de> for Score {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
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
```

---

### 3. Example Input Handling with `Deref`

```rust
use serde_json::json;

let input = json!({ "age": "oops", "score": 90, "nickname": "Nina" });
let profile: UserProfile = serde_json::from_value(input).unwrap();

let age = profile.age.0;
let score = profile.score.0;
let nickname = &profile.nickname;

println!("User: {nickname}, Age: {age}, Score: {score}");
```

---

## 🧪 Tests

### Run Unit Tests for `lenient`

```sh
cargo test -p lenient
```

### Sample Test

```rust
#[test]
fn test_lenient_invalid() {
    let json = r#"{ "age": "invalid", "score": 55, "nickname": "test" }"#;

    let result: UserProfile = serde_json::from_str(json).unwrap();
    assert_eq!(result.age.0, 0); // fallback to default
    assert_eq!(result.nickname, "test");
}
```

---

## 🔖 License

MIT