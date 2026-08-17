//! Probe 2: coexistence — functions that use soroban-sdk host types with NO
//! specs. The checker must compile the sdk (under flux's nightly) and treat
//! these calls as unrefined without ICEing.

use soroban_flux::prelude::*;
use soroban_sdk::{Bytes, BytesN, Env, Vec};

/// TRUST: sdk host calls are unverified; this fn only checks that flux
/// tolerates them appearing in a checked crate.
#[trusted]
pub fn build_vec(env: &Env) -> Vec<u32> {
    let mut v = Vec::new(env);
    v.push_back(7);
    v
}

/// Same, without `#[trusted]`: flux should check this fn body, treating the
/// unspec'd sdk methods as uninterpreted. If this ICEs, probe 3's extern
/// specs become mandatory rather than optional.
pub fn hash_prefix(env: &Env, hash: &BytesN<32>) -> Bytes {
    let all = Bytes::from_array(env, &hash.to_array());
    all.slice(0..4)
}
