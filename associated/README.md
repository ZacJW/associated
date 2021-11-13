# associated

A trait to associate enum variants with constants. See [associated-derive](https://docs.rs/associated-derive) for deriving this trait automatically.

Derive support is enabled with the `"derive"` feature.

## Example

```rust
#[derive(Associated)]
#[associated(Type = &'static str)]
enum Phonetic {
    #[assoc_const("Alpha")] Alpha,
    #[assoc(&"Bravo")] // #[assoc] requires an expression of type &'static Type
    Bravo = 3 // supports explicit enum discriminants
    // ...
}

Phonetic::Alpha.get_associated() // returns a static lifetime reference to "Alpha"
```
