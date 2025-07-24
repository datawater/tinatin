# TINATIN
Maybe a chess engine in rust. Maybe not.
feast your eyes on the horrid, unstable, nightly, const code, that takes way too much time to compile and doesn't really benefit runtime.

> [!WARNING]
> Currnetly does absolutely nothing

### Building
For debug builds
```bash
cargo build
```

For release builds
```bash
# Warning, release takes a while to compile
RUSTFLAGS="-Ctarget-cpu=native" cargo build --release
```