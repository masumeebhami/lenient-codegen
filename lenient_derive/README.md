# 🛠️ lenient_derive

This crate provides the `#[derive(LenientDeserialize)]` procedural macro to enable **fault-tolerant Serde deserialization** for struct fields in Rust.

It is designed to work in conjunction with the [`lenient`](https://crates.io/crates/lenient) crate, which defines wrappers for lenient field handling using `serde`.

---

## ✨ Features

- `#[derive(LenientDeserialize)]` for struct-level deserialization
- Field-level `#[lenient]` to fallback to default on deserialization failure
- Field-level `#[optional]` to fallback to `None` on failure
- Designed for use with `Lenient<T>` and `Optional<T>` wrappers
- Automatically implements `Deserialize` for the annotated struct

---

## 📦 Example Usage

In your `Cargo.toml`:

```toml
[dependencies]
lenient = "0.1"
```

In your Rust code:

```rust
use lenient_derive::LenientDeserialize;
use serde::Deserialize;

#[derive(Debug, Default)]
struct Age(pub u32);
impl<'de> Deserialize<'de> for Age {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        Ok(Age(u32::deserialize(de)?))
    }
}

#[derive(Debug, Default)]
struct Score(pub u8);
impl<'de> Deserialize<'de> for Score {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        Ok(Score(u8::deserialize(de)?))
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

## 🧪 How It Works

The macro generates an internal deserialization wrapper struct like:

```rust
#[derive(Deserialize)]
struct UserProfileInternal {
    #[serde(default)]
    age: Lenient<Age>,
    #[serde(default)]
    score: Optional<Score>,
    #[serde(default)]
    nickname: Lenient<String>,
}
```

Then it provides a `Deserialize` implementation for the outer struct using the inner one.

---

## 📄 License

MIT
