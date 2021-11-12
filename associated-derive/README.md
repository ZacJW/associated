# associated-derive

Derive macro for `Associated`.

## Usage

Add `#[derive(Associated)]` to an enum definition. This is not compatible with structs or unions.

When deriving `Associated` you must include a `#[associate(Type = associated_type)]` attribute beneath
the `#[derive(Associated)]` attribute, replacing `associated_type` with the type of the constants you
want to associate with the enum variants.

For each and **every** variant of the enum you must include either a `#[assoc(expr)]` or
`#[assoc_const(const_expr)]` attribute above or inline before the variant, with `expr` or `const_expr`
replaced with the expression or value you want to associate.

### Example

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

#### Generated Implementation

```rust
impl associated::Associated for Phonetic {
    type AssociatedType = &'static str;
    fn get_associated(&self) -> &'static Self::AssociatedType {
        match self {
            Phonetic::Alpha => {
                const ASSOCIATED: &'static str = "Alpha";
                &ASSOCIATED 
            },
            Phonetic::Bravo => &"Bravo",
        }
    }
}
```

### Note

If you give a variant both an `#[assoc]` and an `#[assoc_const]` attribute, or multiple `#[assoc]`
or `#[assoc_const]` attributes, only the first will be considered. Including more than one is not
currently an error, but this **will** change so only use one `#[assoc]` or `#[assoc_const]`
attribute per variant.

See [associated](https://docs.rs/associated) for retrieving associated constants.
