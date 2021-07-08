# Slcsp Parser

## justin cremer

[git repo]

Slcsp Parser is built in the rust language and therefore requires the rust toolchain for execution.
The following script will install the core rust compiler (rustc), along with a few utilites, and most
notably, cargo, rust's package and project manager.  It will also check that cargo in installed correctly.

``` sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo --version
```
Install dependencies
``` sh
cargo update
```
Run a debug build
``` sh
cargo run
```
Build and execute a release build
``` sh
cargo build --release
./target/release/slcsp-parser
```
Run unit tests
``` sh
cargo test
```

[git repo]: <https://github.com/justincremer/slcsp-parser.git>
