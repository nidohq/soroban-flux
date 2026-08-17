//! Probe 4: an inline flux spec on a `#[contractimpl]` method.
//!
//! Two findings this probe pins down:
//!
//! 1. **Attr stripping is already upstream.** The 2024 flux-rs sdk fork
//!    patched the macros to strip `flux_tool::*` attributes from generated
//!    copies; modern sdks (25+) do this generally —
//!    `pass_through_attr_to_gen_code` allowlists only doc/cfg/allow/deny onto
//!    generated code. No sdk patch is needed for inline annotation.
//!
//! 2. **The generated dispatcher is checked as a caller.** `#[contractimpl]`
//!    emits `__Probe__probe_spend__invoke_raw`, which calls the method with
//!    host-decoded (arbitrary) values — so flux rejects *input refinements*
//!    on public entry points as unprovable preconditions. That is soundness,
//!    not friction: a real caller can pass any i128. Entry points therefore
//!    validate at the boundary and carry postconditions; rich preconditions
//!    belong on the internal (macro-free) library layer.

use soroban_flux::prelude::*;
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct Probe;

#[contractimpl]
impl Probe {
    /// Overdraft-safe spend as a real contract entry point: unrefined inputs
    /// (the host can send anything), boundary validation inside, and a
    /// flux-proven non-negative result.
    #[sig(fn(env: Env, balance: i128, amount: i128) -> i128{v: v >= 0})]
    pub fn probe_spend(env: Env, balance: i128, amount: i128) -> i128 {
        let _ = env;
        if balance < 0 || amount < 0 || amount > balance {
            return 0;
        }
        balance - amount
    }
}
