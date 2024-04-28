## Rust Axum Full Course Follow Along

This repository contains code related to a follow along of [Jeremy Chone's Rust Axum Full Course Tutorial](https://www.youtube.com/watch?v=XZtlD_m59sM).

**Command Line:**

Throughout this tutorial, Jeremy uses the `[httpc-test](https://crates.io/crates/httpc-test)` crate to test the http routes without having to use the browser. The command line invocations are quite lengthy, and will require two terminals (one to run the server and the other to run httpc-test).

```sh
# Runs the Server
cargo watch -q -c -w src/ -x run
```

```sh
# Runs the httpc-test suite
cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"
```
