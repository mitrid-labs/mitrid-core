[![Travis branch](https://img.shields.io/travis/mitrid-labs/mitrid-core/master.svg)](https://travis-ci.org/mitrid-labs/mitrid-core)
[![Coveralls github branch](https://img.shields.io/coveralls/github/mitrid-labs/mitrid-core/master.svg)](https://coveralls.io/github/mitrid-labs/mitrid-core?branch=master)
[![Crates.io](https://img.shields.io/crates/v/mitrid-core.svg)](https://crates.io/crates/mitrid-core)
[![Docs.rs](https://docs.rs/mitrid_core/badge.svg)](https://docs.rs/mitrid_core)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://github.com/mitrid-labs/mitrid-core/blob/master/LICENSE)

Mitrid Core is a framework for building blockchains and other block-based distributed technology ledgers. It is meant to be flexible enough to cover different architectures (permissionless and permissioned; with authenticated chains, trees or directed graphs) and to let developers use the cryptographical protocols they prefer. Add-ons will be added to ease development.
<br>
<br>

## Install

To install the Mitrid Core library, add in your Cargo.toml:


```toml
# Cargo.toml

[dependencies]
mitrid_core = "^0.6"
```

and in the root of your crate:

```rust

extern crate mitrid_core;
```

## Testing

To build the tests you need `libsodium` and `libclang` installed on your system.
If the dependencies are resolved, then you can run the tests by typing `cargo test` on your terminal.

## [License](LICENSE)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
