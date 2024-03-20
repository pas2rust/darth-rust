# Darth Rust

## About
DarthRust is a Rust procedural macro library that aims to enhance developer productivity by auto-generating essential methods for struct manipulation. It provides a suite of macros that can be derived on any struct to equip it with a rich API for accessing and modifying fields, performing calculations, and handling input/output operations. With features like mutable and immutable getters, setters, JSON conversion methods, and custom printer functions for debugging, DarthRust streamlines the boilerplate code typically associated with struct management in Rust applications. It also includes unique methods for regex validation and range checking, further extending its utility. The library emphasizes ease of use, requiring only the addition of relevant crates and a simple derive attribute to unlock its full potential.

## Install
```bash
cargo add darth-rust
cargo add regex
cargo add colorful
cargo add serde
cargo add chrono
```

## Use
### Examples
#### https://github.com/pas2rust/darth-rust/tree/master/tests
```rust
use darth_rust::DarthRust;
use serde::{Deserialize, Serialize};

#[derive(DarthRust, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct User {
    id: String,
    name: String,
    password: String,
    email: String,
    age: usize,
    friends: Vec<User>,
}
```

## Contribution
...

## License
MIT