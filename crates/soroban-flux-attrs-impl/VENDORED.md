# Vendored from flux-rs/flux

`soroban-flux-attrs-impl`, `soroban-flux-attrs`, and `soroban-flux-rs` are
byte-identical copies of `lib/flux-attrs-impl`, `lib/flux-attrs`, and
`lib/flux-rs` from https://github.com/flux-rs/flux (MIT — see
LICENSE-UPSTREAM) at the rev recorded in COMPATIBILITY.md / the justfile's
FLUX_REV. Only the Cargo.toml manifests differ (package renames via cargo
dependency renaming, so `src/` never changes).

Why vendor: crates.io forbids git dependencies, and flux publishes no
crates — vendoring is what makes soroban-flux publishable. The driver
interoperates by attribute namespace (`flux_tool::*`, registered by the
flux driver), not by crate identity, so renamed packages verify identically
(proven by this workspace's probe crate under `cargo flux`).

To re-vendor after a FLUX_REV bump:
`git -C <flux-clone> checkout $FLUX_REV && cp -r lib/flux-{attrs-impl,attrs,rs}/src crates/soroban-flux-*/` then re-run the full gate set.
