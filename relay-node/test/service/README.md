# `test-service`

## Testing

Before running `cargo test` in this crate, make sure the worker binaries are built first. This can be done with:

```sh
cargo build -p zkv-relay --bin zkv-relay-execute-worker --bin zkv-relay-prepare-worker
```
