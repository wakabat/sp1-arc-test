# sp1-arc-test

This repo demos one weird behavior of SP1's Rust toolchain. To replicate it, do the following:

```
$ git clone https://github.com/wakabat/sp1-arc-test
$ cd sp1-arc-test
$ rustup target add aarch64-unknown-linux-gnu
$ rustup target add riscv64gc-unknown-linux-gnu

$ cargo build
$ cargo build --target aarch64-unknown-linux-gnu
$ cargo build --target riscv64gc-unknown-linux-gnu
$ cargo +succinct build
```

All of the build commands above work fine, but the following one would fail:

```
cargo +succinct build --target riscv32im-succinct-zkvm-elf
```

It complains that while `Inner` implements `Archive`, `Arc<Inner>` does not actually implement `Archive`.

However, if we check the result of cargo expand(I've put the expanded source file [here](./cargo_expand_result.rs) in this repo), `Arc<Inner>` never implements `Archive` directly. The compiler is expected do the type coersion automatically. Weirdly enough, this behavior is not respected in `riscv32im-succinct-zkvm-elf` target of Succinct's rust toolchain.

Some more of my local environment:

```
$ rustc --version
rustc 1.85.0 (4d91de4e4 2025-02-17)
$ cargo --version
cargo 1.85.0 (d73d2caf9 2024-12-31)
$ rustc +succinct --version
rustc 1.85.0-dev
$ cargo +succinct --version
cargo 1.87.0-nightly (6cf826701 2025-03-14)
```

Note the problem goes away when we upgrade rkyv, see [this branch](https://github.com/wakabat/sp1-arc-test/tree/fix) for more details.
