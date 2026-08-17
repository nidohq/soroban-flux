//! Probe 1: pure refinement checking — i128 arithmetic, bounds, indexing.
//! No sdk types touched; failure here means flux itself is unusable in this
//! workspace.

use soroban_flux::prelude::*;

/// The design doc's flagship: overdraft-safe spend. Flux must prove the
/// subtraction cannot underflow and the result stays non-negative.
#[sig(fn(balance: i128{v: v >= 0}, amount: i128{v: v >= 0}) -> Option<i128{v: v >= 0}>)]
pub fn spend(balance: i128, amount: i128) -> Option<i128> {
    if amount > balance {
        return None;
    }
    Some(balance - amount)
}

/// Exact-successor nonce acceptance (nido controller.rs pattern). The
/// `stored + 1` overflow obligation is discharged by the precondition.
#[sig(fn(stored: u64{stored < 18446744073709551615}, nonce: u64) -> Option<u64{v: v == stored + 1}>)]
pub fn accept_nonce(stored: u64, nonce: u64) -> Option<u64> {
    if nonce != stored + 1 {
        return None;
    }
    Some(nonce)
}

/// Perch rpn.rs pattern: stack-pointer-bounded array write. The index bound
/// makes the write provably in-range with no runtime guard.
#[sig(fn(stack: &mut [u32; 128], sp: usize{v: v < 128}, value: u32))]
pub fn stack_write(stack: &mut [u32; 128], sp: usize, value: u32) {
    stack[sp] = value;
}

/// Refined-newtype round trip through the bridge crate's `Amount`.
#[sig(fn(start: i128{v: v >= 0}, fee: i128{v: v >= 0}) -> i128{v: v >= 0})]
pub fn settle(start: i128, fee: i128) -> i128 {
    let balance = Amount::new(start);
    match balance.spend(Amount::new(fee)) {
        Ok(rest) => rest.get(),
        Err(AmountError::Insufficient) => start,
    }
}
