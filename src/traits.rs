//! This module contains the traits that are used to define the gacha system.

pub mod pool {
    use anyhow::Result;

    use super::result::GachaResult;

    /// A trait that defines the functionality of a gacha pool.
    pub trait GachaPool {
        fn pull(&self) -> Result<impl GachaResult>;
        fn pull_n(&self, n: usize) -> Result<Vec<impl GachaResult>>;
    }
}

pub mod result {
    use anyhow::Result;

    /// Marker trait for gacha rarities.
    pub trait GachaRarity {}

    /// A trait that defines the functionality of a gacha result.
    pub trait GachaResult {
        fn get_result_name(&self) -> Result<String>;
        fn get_result_rarity(&self) -> Result<impl GachaRarity>;
    }
}
