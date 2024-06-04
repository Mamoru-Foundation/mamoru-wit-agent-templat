# mamoru-wit-agent-templat
This project is a Rust-based application that uses the Mamoru Foundation's wit agent template.

## Prerequisites

Before you begin, ensure you have met the following requirements:

- You have installed [Rust](https://www.rust-lang.org/tools/install) 

## Create wit agent template

To create the wit agent template, run the following command:

```bash
cargo generate --git https://github.com/Mamoru-Foundation/mamoru-wit-agent-templat.git
```


## Installing Required Tools

To install all the dependencies, run the following commands:

```bash
 rustup target add wasm32-unknown-unknown
 cargo install cargo-generate
 cargo install cargo-component
 cargo install wasm-tools
```


## Building the Project
To build the project, run the following commands:

```bash
cargo component build --release

wasm-tools component wit target/wasm32-wasi/release/{{crate_name}}.wasm
```
