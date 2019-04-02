# decklink-rs

This crate wraps the DeckLink SDK. To iterate connected devices, for example, you could just do something like this:

```rust
for device in decklink::Iterator::new().unwrap() {
    println!("{}", device.get_model_name().unwrap());
}
```
