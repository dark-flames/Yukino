# Yukino
Yukino is an ORM framework for Rust

### Features
* Define model by types and annotation
* Database abstract layer
* Query builder
* Association management
* Delayed execute
* Auto cache and reverse cache
* Command-Line management tools

## Contributing to Yukino
This project is built with Rust Stable.
1. setup test schema
```shell script
cargo run --package yukino-test-entity --bin cli setup
```
2. run cargo test
```shell script
cargo test --all-target
```
