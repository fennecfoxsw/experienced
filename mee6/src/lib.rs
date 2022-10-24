#![cfg_attr(test, feature(test))]
#![warn(clippy::all, clippy::cargo, clippy::nursery, clippy::pedantic)]
// We allow cast precision loss because we will never be messing with integers bigger then 52 bits realistically
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
)]
#![no_std]
//! A library to calculate mee6 levels.
//! This can be calculated using the `LevelInfo` struct.

/// `LevelInfo` stores all of the data calculated when using `LevelInfo::new`(), so it can be cheaply
/// gotten with getters.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct LevelInfo {
    xp: u64,
    level: u64,
    percentage: u8,
}

impl LevelInfo {
    /// Create a new `LevelInfo` struct. This operation calculates the current percentage and level
    /// immediately, rather then when the getter is called.
    #[must_use]
    pub fn new(xp: u64) -> Self {
        // The operation used to calculate how many XP a given level is is (5 / 6) * level * (2 * level * level + 27 * level + 91), but it's optimized here.
        let level = {
            let xp = xp as f64;
            let mut testxp = 0.0;
            let mut level = 0;
            while xp >= testxp {
                level += 1;
                testxp = Self::xp_to_level(f64::from(level));
            }
            level - 1
        };
        let last_level_xp_requirement = Self::xp_to_level(f64::from(level));
        let next_level_xp_requirement = Self::xp_to_level(f64::from(level + 1));
        Self {
            xp,
            level: level as u64,
            percentage: (((xp as f64 - last_level_xp_requirement)
                / (next_level_xp_requirement - last_level_xp_requirement))
                * 100.0) as u8,
        }
    }
    /// Get the xp that was input into this `LevelInfo`.
    #[must_use]
    #[inline]
    pub const fn xp(&self) -> u64 {
        self.xp
    }
    /// Get the level that this `LevelInfo` represents.
    #[must_use]
    #[inline]
    pub const fn level(&self) -> u64 {
        self.level
    }
    /// Get the percentage of the way this `LevelInfo` is to gaining a level, from the last level.
    #[must_use]
    #[inline]
    pub const fn percentage(&self) -> u8 {
        self.percentage
    }
    // mul_add is not no-std
    #[allow(clippy::suboptimal_flops)]
    #[inline]
    fn xp_to_level(level: f64) -> f64 {
        (5.0 / 6.0) * level * (2.0 * level * level + 27.0 * level + 91.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    #[test]
    fn level() {
        let inf = LevelInfo::new(3255);
        assert_eq!(inf.level(), 8);
    }
    #[test]
    fn xp() {
        let inf = LevelInfo::new(3255);
        assert_eq!(inf.xp(), 3255);
    }
    #[test]
    fn percentage() {
        let inf = LevelInfo::new(3255);
        assert_eq!(inf.percentage(), 43);
    }

    #[bench]
    fn create_levelinfo(b: &mut test::Bencher) {
        b.iter(|| {
            for i in 1..1_000_000 {
                test::black_box(LevelInfo::new(i));
            }
        })
    }
}
