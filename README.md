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
    from: Size,
    #[optional]
    size: Size,
}
```

---

### 3. Example Input Handling with `Deref`

```rust
use serde_json::json;

let input = json!({ "from": "oops", "size": 10 });
let offset: Offset = serde_json::from_value(input).unwrap();

// Access using Deref
let from_val = offset.from.0;
let size_val = offset.size.0;

println!("From: {from_val}, Size: {size_val}");
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
    let json = r#"{ "value": "invalid" }"#;

    #[derive(Debug, Default, PartialEq, Deserialize)]
    struct MyType {
        value: usize,
    }

    let result: lenient::Lenient<MyType> = serde_json::from_str(json).unwrap();
    assert_eq!(result.value, 0); // defaulted
}
```

---

## 🔖 License

MIT or Apache 2.0 — your choice.