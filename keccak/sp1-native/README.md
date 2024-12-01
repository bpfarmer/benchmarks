# Proving bench

Compile with either:
- `cargo run --release` (SSE, packed 4x u32 per SIMD unit)
- `RUSTFLAGS="-C target-cpu=native" cargo run --release` (AVX2, packed 8x u32 per SIMD unit)

## Succinct Toolchain

The Succinct / SP1 toolchain is required to run these benchmarks. You can find installation instructions [here](https://docs.succinct.xyz/getting-started/install.html).
