//! Pins `BN254_R_BE` to the decimal field order and checks the helper
//! behavior at the boundary values `r-1`, `r`, `r+1`.

use soroban_flux::bn254::{is_canonical_be, reduce_be, BN254_R_BE};
use soroban_sdk::{BytesN, Env};

/// Recompute the byte constant from the decimal string with plain bignum
/// arithmetic — no shared code path with the constant itself.
fn r_bytes_from_decimal() -> [u8; 32] {
    const R_DEC: &str =
        "21888242871839275222246405745257275088548364400416034343698204186575808495617";
    let mut digits: Vec<u8> = R_DEC.bytes().map(|b| b - b'0').collect();
    let mut out = [0u8; 32];
    for byte in out.iter_mut().rev() {
        // digits mod 256 -> byte; digits /= 256, one long-division pass each.
        let mut rem: u32 = 0;
        let mut next = Vec::with_capacity(digits.len());
        for &d in &digits {
            let cur = rem * 10 + u32::from(d);
            next.push((cur / 256) as u8);
            rem = cur % 256;
        }
        while next.first() == Some(&0) {
            next.remove(0);
        }
        *byte = rem as u8;
        digits = next;
    }
    assert!(digits.is_empty(), "r must fit in 32 bytes");
    out
}

#[test]
fn constant_matches_decimal_r() {
    assert_eq!(BN254_R_BE, r_bytes_from_decimal());
}

#[test]
fn canonicity_boundaries() {
    let env = Env::default();

    let r = BytesN::from_array(&env, &BN254_R_BE);
    assert!(!is_canonical_be(&env, &r), "r itself is not canonical");

    let mut below = BN254_R_BE;
    below[31] -= 1; // ...00_01 -> ...00_00
    let below = BytesN::from_array(&env, &below);
    assert!(is_canonical_be(&env, &below), "r - 1 is canonical");

    let mut above = BN254_R_BE;
    above[31] += 1; // ...00_01 -> ...00_02
    let above = BytesN::from_array(&env, &above);
    assert!(!is_canonical_be(&env, &above), "r + 1 is not canonical");

    assert_eq!(
        reduce_be(&env, &r),
        BytesN::from_array(&env, &[0u8; 32]),
        "r mod r == 0"
    );
    assert_eq!(reduce_be(&env, &below), below, "(r - 1) mod r == r - 1");
}
