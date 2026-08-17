# Compatibility matrix

Single source of truth: the `FLUX_REV` / `FLUX_NIGHTLY` constants in the
`justfile`. Consumer repos copy those values into their own `flux` recipes;
bump here first, burn in, then roll out.

| soroban-flux | flux rev | nightly | soroban-sdk | z3 | fixpoint | status |
|---|---|---|---|---|---|---|
| 0.1.0-dev | `283ad73` (2026-08-17) | nightly-2026-02-05 | =26.0.1 | 5.0.0 | nightly (2026-08) | **M0 GO** (2026-08-17) |

## M0 probe log (2026-08-17) — GO

All three probes green; whole run sub-second warm, ~25 s cold (dep compile).

- **Probe 1 (pure)**: `spend` (overdraft-safe i128), `accept_nonce`
  (`v == stored + 1` successor proof), `stack_write` (`sp < 128` array bound,
  no runtime guard), `settle` (refined `Amount` round trip) — all verified.
  Per-fn solver time 0.1–22 ms.
- **Probe 2 (sdk coexistence)**: the ENTIRE soroban-sdk 26.0.1 dep tree
  (soroban-env-host, ark-bn254, p256, wasmi) compiles under flux's
  nightly-2026-02-05 with zero ICEs. Un-spec'd sdk calls (`BytesN::to_array`,
  `Bytes::slice`) coexist with checking — `hash_prefix` is checked, not
  trusted.
- **Probe 3 (extern_spec)**: `Vec` refined by length (`new -> Vec<T>[0]`,
  `push_back` strong-update `n + 1`, `len -> u32[n]`) verifies end-to-end:
  `two_items -> u32[2]` proven.
- **Negative controls**: `#[should_fail]` works; removing it from the vacuity
  canary (`fn() -> bool[false]`) produces the expected hard error — the
  checker is not vacuous.

### Syntax/behavior drift discovered vs. the research docs (docs/research/)

1. `-Fcheck-overflow` now takes a mode: `none | lazy | strict` (bare flag
   errors). We use `strict`.
2. `defs!` grammar rejects `///` doc comments — plain `//` only.
3. Flux integer literals cap below ~2^128: BN254 `r` must be assembled from
   64-bit limbs (see `src/defs.rs`).
4. `{v: v >= 0}` existentials bind no index — an output refinement that
   references an input needs constraint-type (`{i128[@val] | val >= 0}`) or
   argument syntax (`stored: u64{stored < MAX}`).
5. `extern_spec` impl blocks must mirror the SDK's *block structure*, not just
   signatures: `Vec::new`/`push_back` live in the `T: IntoVal + TryFromVal`
   block, `len` in a separate unbounded `impl<T> Vec<T>`.

## contractimpl inline-annotation findings (2026-08-17, probe 4)

- The 2024 flux-rs sdk-fork patch (`filter_out_flux_attrs`) is **obsolete**:
  soroban-sdk 25+ strips all non-doc/cfg/allow/deny attributes from
  macro-generated code via `pass_through_attr_to_gen_code`, so `flux_tool::*`
  attrs never reach generated copies. Inline `#[sig]` on `#[contractimpl]`
  methods works without any sdk patch.
- The generated dispatcher (`__Ty__fn__invoke_raw`) is flux-checked **as a
  caller** of the annotated method: input refinements on public entry points
  are rejected as unprovable preconditions (the host can pass anything).
  Correct pattern: entry points validate at the boundary and carry
  postconditions; rich preconditions live on the internal macro-free layer.
