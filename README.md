# Rust Playground
My first time trying rust

## About Rust
Rust is a language with focus on:
- Performance
    - no garbage collector
    - no runtime
    - memory efficiency
- Reliability
    - "Rust’s rich type system and ownership model guarantee memory-safety and thread-safety"
- Productivity
    - integrated package manager and build tool cargo
    - various additional tools

## Cheatsheet
### Setting up a new project
- ``cargo new <project_name>`` creates a new project
- ``cargo run`` runs project in current directory
- ``cargo build`` installs dependencies from ``Cargo.toml``

### References
Short example:
```
let x: i32 = 42;      // a normal (immutable by default!) variable
let r: &i32 = &x;     // a reference to variable x
let v: i32 = *r;      // the value of the reference
let mut y: i32 = 42;  // a mutable variable
let m: &i32 = &mut y; // a mutable reference
```
A good explanation can be found [here](https://hashrust.com/blog/references-in-rust/)

### Misc
To initialize a buffered writer:
```
use std::io::{stdout, BufWriter};
fn main() {
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
}
```