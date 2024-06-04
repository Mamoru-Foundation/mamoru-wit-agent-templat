## Create wit agent template
cargo generate --git https://github.com/Mamoru-Foundation/mamoru-wit-agent-templat.git


## Requirements
Install Rust 
Run the following command to install all the dependencies:
cargo install cargo-generate
cargo install cargo-component
cargo install wasm-tools
rustup target add wasm32-unknown-unknown

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm32-unknown-unknown target](https://rustwasm.github.io/wasm-pack/installer/)

## Building
cargo component build --release
wasm-tools component wit target/wasm32-wasi/release/<crate>.wasm
