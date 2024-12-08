# into_enum

into_enum provides a single macro, IntoEnum.

Enums with unique variants can derive IntoEnum to provide straight-forward `From` implementations.

```rust
use into_enum::IntoEnum;

#[derive(IntoEnum)]
enum MyError {
    IO(std::io::Error),         // impl From<std::io::Error> for MyError
    Reqwest(reqwest::Error),    // impl From<reqwest::Error> for MyError
    Custom(crate::Error),       // impl From<crate::Error> for MyError
}
```

The macro provides the attribute `into_enum(skip)` to ignore a variant, in case of type conflict or unwanted conversion.

```rust
#[derive(IntoEnum)]
enum Results {
    Default(u32),       // impl From<u32> for Results
    #[into_enum(skip)]
    Special(u32),       // nothing generated
    Message(String),    // impl From<String> for Results
}
```

The macro also generates conversions for unit variants and tuple variants. Variants with named fields are skipped.

```rust
#[derive(IntoEnum)]
enum Numerical {
    Default,                // impl From<()> for Numerical
    Number(u32),            // impl From<u32> for Numerical
    Double(u32, u32),       // impl From<(u32, u32)> for Numerical
    Complex { base: u32 }   // Nothing generated
}
```

The macro maintains trait bounds for generic types.