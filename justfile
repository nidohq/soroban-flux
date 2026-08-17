# soroban-flux — Flux refinement-type bridge for Soroban.
#
# FLUX_REV is the single source of truth for the flux toolchain across every
# consumer repo. Bump it here first, burn in the CI matrix, then roll the new
# rev out to consumers' justfiles. FLUX_NIGHTLY must match the
# rust-toolchain.toml inside flux at that rev.

FLUX_REV := "283ad737569f80951973747ef16f1a89d64d9728"
FLUX_NIGHTLY := "nightly-2026-02-05"
FLUXFLAGS := "-Fpointer-width=32 -Fcheck-overflow=strict -Fcache=target/flux-cache"

# Verify all flux-enabled crates ([package.metadata.flux] enabled = true).
flux *ARGS:
    FLUXFLAGS="{{FLUXFLAGS}}" cargo flux {{ARGS}}

# Prove the attribute-erasure claim: everything must also build + test on
# plain stable with flux attrs stripped to no-ops.
test:
    RUSTUP_TOOLCHAIN=stable cargo test --workspace

check:
    RUSTUP_TOOLCHAIN=stable cargo fmt --all --check
    RUSTUP_TOOLCHAIN=stable cargo clippy --workspace --all-targets -- -D warnings

# One-shot toolchain bootstrap: z3 + liquid-fixpoint prebuilts, then build
# cargo-flux/flux-driver from source at FLUX_REV. Idempotent; ~15 min cold.
flux-setup:
    #!/usr/bin/env bash
    set -euo pipefail
    BIN="$HOME/.cargo/bin"
    command -v z3 >/dev/null || brew install z3
    if ! command -v fixpoint >/dev/null; then
        curl -fsSL "https://github.com/ucsd-progsys/liquid-fixpoint/releases/download/nightly/fixpoint-aarch64-apple-darwin.tar.gz" | tar xz -C "$BIN"
        chmod +x "$BIN/fixpoint"
    fi
    if ! command -v cargo-flux >/dev/null; then
        tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
        git clone https://github.com/flux-rs/flux "$tmp/flux"
        git -C "$tmp/flux" checkout {{FLUX_REV}}
        (cd "$tmp/flux" && cargo xtask install)
    fi
    echo "flux-setup ok: $(z3 --version) | fixpoint + cargo-flux in $BIN"
