//! Serde helper functions for serializing UOM types as flat numeric values.
//!
//! These functions use `&Option<T>` signatures because that is what serde's
//! `serialize_with` attribute passes. Clippy's `ref_option` lint does not apply.

use serde::{self, Serializer};
use uom::si::{f64::Length as Length_f64, f64::Velocity, u16::Length as Length_u16};

#[allow(clippy::trivially_copy_pass_by_ref, clippy::ref_option)]
pub fn serialize_opt_velocity<S>(v: &Option<Velocity>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match v {
        Some(vel) => s.serialize_some(&vel.value),
        None => s.serialize_none(),
    }
}

#[allow(clippy::trivially_copy_pass_by_ref, clippy::ref_option)]
pub fn serialize_opt_length_f64<S>(v: &Option<Length_f64>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match v {
        Some(len) => s.serialize_some(&len.value),
        None => s.serialize_none(),
    }
}

#[allow(clippy::trivially_copy_pass_by_ref, clippy::ref_option)]
pub fn serialize_opt_length_u16<S>(v: &Option<Length_u16>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match v {
        Some(len) => s.serialize_some(&len.value),
        None => s.serialize_none(),
    }
}
