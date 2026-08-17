//! Probe 3: extern_spec staging ground for soroban-sdk types. Each spec that
//! verifies here graduates to `soroban_flux::specs`.
//!
//! Extern structs are opaque-only, methods cannot take `self`, and impl
//! blocks must mirror the SDK's exact generics/bounds (soroban-sdk 26.0.1).

use soroban_flux::prelude::*;
// The trait imports are consumed only by the extern_spec expansion, which a
// normal (non-flux) build strips — hence the allow.
#[allow(unused_imports)]
use soroban_sdk::{Env, IntoVal, TryFromVal, Val, Vec};

// soroban_sdk::Vec<T>, refined by its length.
#[extern_spec(soroban_sdk)]
#[refined_by(n: int)]
#[invariant(n >= 0)]
struct Vec<T>;

// Extern impl blocks must mirror the SDK's own block structure exactly:
// `new`/`push_back` live in `impl<T> Vec<T> where T: IntoVal + TryFromVal`
// (vec.rs:358), `len` in a separate unbounded `impl<T> Vec<T>` (vec.rs:837).
#[extern_spec(soroban_sdk)]
impl<T: IntoVal<Env, Val> + TryFromVal<Env, Val>> Vec<T> {
    #[sig(fn(env: &Env) -> Vec<T>[0])]
    fn new(env: &Env) -> Vec<T>;

    #[sig(fn(v: &strg Vec<T>[@n], x: T) ensures v: Vec<T>[n + 1])]
    fn push_back(v: &mut Vec<T>, x: T);
}

#[extern_spec(soroban_sdk)]
impl<T> Vec<T> {
    #[sig(fn(v: &Vec<T>[@n]) -> u32[n])]
    fn len(v: &Vec<T>) -> u32;
}

/// Client fn exercising the specs: length reasoning across new + push_back.
#[sig(fn(env: &Env) -> u32[2])]
pub fn two_items(env: &Env) -> u32 {
    let mut v: Vec<u32> = Vec::new(env);
    v.push_back(1);
    v.push_back(2);
    v.len()
}
