//! # Gacha Traits
//!
//! This crate defines a set of traits which describe the functionality of gacha systems.
//!

pub use anyhow::Result;

mod traits;

pub use self::traits::{
    pool::GachaPool,
    result::{GachaRarity, GachaResult},
};
