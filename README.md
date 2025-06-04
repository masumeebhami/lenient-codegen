# 🧱 Lenient Deserialize

A Rust crate for **fault-tolerant deserialization** using Serde. Automatically wraps fields to ensure invalid values fall back to sensible defaults — instead of breaking the entire deserialization process.

This is useful when handling user inputs, external APIs, or anything where input reliability is uncertain.

---

## ✨ Features

- `Lenient<T>`: wraps any type to gracefully fallback to `T::default()` on deserialization failure.
- `Optional<T>`: alias for `Lenient<Option<T>>`.
- `#[derive(LenientDeserialize)]`: a procedural macro to generate fault-tolerant wrappers for entire structs automatically.
- Optional error logging via [`tracing`](https://docs.rs/tracing).

---

## 📦 Crate Structure

This workspace contains two crates:

```
lenient/
├── lenient/            # The main library (Lenient wrapper, re-exports macro)
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
    from: Size,
    size: Size,
}
```

---

### 3. Example Input Handling

```rust
use serde_json::json;

let input = json!({ "from": "oops", "size": 10 });
let offset: Offset = serde_json::from_value(input).unwrap();

// Prints: from: defaulted due to invalid input, size: 10
dbg!(offset);
```

---

## 🧪 Running Tests

If you're using `main.rs` as an integration test:

```sh
cargo run --bin test_lenient
```

Or write unit tests in `lib.rs` using `#[cfg(test)]`.

---

## 🛠️ Building the Procedural Macro

The `lenient_derive` crate provides `#[derive(LenientDeserialize)]`. You typically **don’t use this directly** — it's re-exported from the `lenient` crate.

---

## 📚 TODOs

- [ ] Add support for field-level `#[lenient]`, `#[optional]` attributes.
- [ ] Implement `Deref` for ergonomic access to wrapped values.
- [ ] Publish to crates.io.

---

## 🔖 License

MIT or Apache 2.0 — your choice.
