# soroban-flux

Flux refinement-type bridge for Soroban smart contracts and the Rust around
them. One dependency gives you:

- **`soroban_flux::prelude`** — the [Flux](https://github.com/flux-rs/flux)
  attribute macros (`sig`, `spec`, `refined_by`, `invariant`, `trusted`,
  `extern_spec`, …). They erase to no-ops under normal `cargo build`: no
  nightly, no z3, zero runtime cost. Verification is a separate, lint-like
  gate (`just flux`).
- **Refined newtypes** (`types`) — e.g. `Amount` (non-negative `i128`) whose
  invariants Flux enforces at compile time.
- **Shared predicates** (`defs`) — `bn254_canonical`, `succ`, `is_btwn`.
- **Extern specs** (`specs`) — refinements for `soroban-sdk` types (`Vec`
  length, `BytesN<N>`, …), grown as they are proven out.

## Architecture rule

Flux verifies **macro-free library code**. Keep refinement logic in plain
functions/modules and annotate those; keep `#[contractimpl]` blocks as thin
marshalling marked `#[trusted]`. See the trust-boundary notes in
`crates/soroban-flux/src/lib.rs`.

## Toolchain

`just flux-setup` installs everything (z3, liquid-fixpoint, cargo-flux built
from the pinned `FLUX_REV`). `COMPATIBILITY.md` records which flux rev /
nightly / soroban-sdk line each version of this crate is tested against.

## Status

M0 feasibility spike. `crates/probe-sdk26` is the live probe; results in
`COMPATIBILITY.md`. Research corpus behind the design: `docs/research/`.
