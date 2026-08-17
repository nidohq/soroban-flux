//! Negative controls: each fn here is DELIBERATELY unverifiable and marked
//! `#[should_fail]` — flux errors if one of them unexpectedly verifies.
//! Guards against a vacuous checker (e.g. an inconsistent extern_spec
//! making everything provable).

use soroban_flux::prelude::*;

/// Missing overdraft guard: result can be negative.
#[should_fail]
#[sig(fn(balance: i128{v: v >= 0}, amount: i128{v: v >= 0}) -> i128{v: v >= 0})]
pub fn spend_unguarded(balance: i128, amount: i128) -> i128 {
    balance - amount
}

/// Missing bound: index can exceed the array.
#[should_fail]
#[sig(fn(stack: &mut [u32; 128], sp: usize, value: u32))]
pub fn stack_write_unguarded(stack: &mut [u32; 128], sp: usize, value: u32) {
    stack[sp] = value;
}

/// Vacuity canary: `false` must never be provable.
#[should_fail]
#[sig(fn() -> bool[false])]
pub fn absurd() -> bool {
    true
}
