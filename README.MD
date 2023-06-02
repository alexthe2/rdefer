# rdefer

A Rust crate providing defer functionality for both synchronous and asynchronous code.

## Usage

### Synchronous Defer

```rust
use rdefer::defer;

let _d = defer!({
    println!("This will be printed last");
});
println!("This will be printed first");
```
### Asynchronous Defer (Feature gated)
This feature is behind the async feature flag.

To use this feature, add rdefer to your Cargo.toml with the async feature enabled:
```toml
[dependencies]
rdefer = { version = "*", features = ["async"] }
```
Then you can use it as follows:
```rust
use rdefer::{async_defer, exec_before_defer};
use std::sync::Arc;

let defer = async_defer!(2, async {
    println!("This will be printed last");
});

exec_before_defer!(defer, || println!("This will be printed first"));
exec_before_defer!(defer, || println!("This will be printed second"));
```