# CoolPotOS Idris2

This repository now contains a minimal `riscv64` kernel framework that can be booted by the bundled builder and focuses on two things:

- a Rust kernel crate with an exported `kernel_entry` plus a Limine bootable entry binary
- a Rust implementation of the Idris2 `refc` runtime ABI needed to execute a basic Idris `main`

## Layout

- `kernel/src/lib.rs`: Rust kernel entry that jumps into the generated Idris program
- `kernel/src/idris2_runtime.rs`: bootstrap refc runtime implemented in Rust
- `kernel/include/runtime.h`: the C ABI surface used by Idris2 `refc` output
- `kernel/idris/kernel.ipkg`: Idris package definition used by both the LSP and the Cargo build
- `kernel/idris/src/KernelMain.idr`: the initial Idris2 entry module
- `kernel/build.rs`: rewrites the Idris package into `OUT_DIR`, builds it with `refc`, cross-compiles the generated C, and links it into the Rust crate

## Build

Inside the dev shell, from the repo root:

```bash
cargo kbuild
cargo krun --serial
```

Useful aliases:

```bash
cargo kbuild
cargo kbuild --release
cargo krun --serial
cargo krun --release --serial
cargo kclippy
```

These aliases are defined in `.cargo/config.toml` and invoke the Rust `builder`
tool directly, so there is no `builder.sh` wrapper in the workflow anymore.

The Idris side is built as a package rather than a single hard-coded entry file. That means you can grow `kernel/idris/src` into a larger module tree and keep package metadata in `kernel/idris/kernel.ipkg`, which also gives Idris2 LSP a stable package file to discover while Cargo still rewrites the build/output paths into `OUT_DIR`.

If you use Nix before initializing a Git repository, prefer:

```bash
nix develop path:.
```

instead of bare `nix develop`.

## Current runtime scope

The runtime is intentionally a bootstrap kernel runtime, not a full drop-in replacement for Idris2's hosted `refc` support library.

- Closure application and trampoline execution are implemented in Rust.
- Heap allocation uses `talc` over a fixed in-kernel arena.
- Runtime objects that reach reference count zero release their storage back to `talc` for the runtime types implemented so far.
- `Integer` is currently represented as `i64`, which is enough for the basic entry path and early kernel bring-up but not arbitrary-precision math.

That keeps the boundary small enough to run Idris-generated code in a freestanding kernel, while leaving a clear place to grow strings, arrays, richer FFI, and a real allocator once the rest of the kernel exists.
