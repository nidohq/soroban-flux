//! M0 feasibility probe (see plan + COMPATIBILITY.md).
//!
//! Three questions, one module each:
//! 1. [`pure`] — does refinement checking work at all in a `#![no_std]` crate
//!    that depends on soroban-sdk (which must compile under flux's nightly)?
//! 2. [`sdk_types`] — can code that *uses* sdk host types coexist with the
//!    checker when those types carry no specs (trusted boundary)?
//! 3. [`extern_specs`] — can `extern_spec` express refinements on sdk types
//!    (`Vec` len, `BytesN<N>`)? Staging ground for `soroban_flux::specs`.

#![no_std]

pub mod extern_specs;
pub mod neg;
pub mod pure;
pub mod sdk_types;
