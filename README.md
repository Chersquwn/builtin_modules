# builtin_modules

A static list of the Node.js builtin modules from the latest Node.js version in rust.

## Install

```bash
cargo add builtin_modules
```

## Usage

```rust
use builtin_modules::BUILTIN_MODULES;

assert!(BUILTIN_MODULES.contains("path"));
```