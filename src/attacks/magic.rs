#![allow(dead_code)]

use crate::types::{BB, Bitboard};
use cfg_if::cfg_if;

pub(super) struct Xorshift64star(u64);

impl Xorshift64star {
    pub const fn new(seed: u64) -> Self {
        Self(if seed != 0 { seed } else { 1 })
    }

    pub const fn rand64(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;

        self.0 * 0x2545_f491_4f6c_dd1d_u64
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Magic<'a> {
    pub mask: Bitboard,
    attacks: &'a [Bitboard],

    #[cfg(not(target_feature = "bmi2"))]
    #[allow(clippy::struct_field_names)]
    magic: Bitboard,
    #[cfg(not(target_feature = "bmi2"))]
    shift: u32,
}

impl<'a> Magic<'a> {
    const fn _empty() -> Magic<'a> {
        cfg_if! {
            if #[cfg(not(target_feature = "bmi2"))] {
                Magic {
                    mask: BB(0),
                    attacks: &[],
                    magic: BB(0),
                    shift: 0
                }
            } else {
                Magic {
                    mask: BB(0),
                    attacks: &[]
                }
            }
        }
    }

    pub const fn const_index(&self, occupied: Bitboard) -> usize {
        (((occupied.0 & self.mask.0).wrapping_mul(self.magic.0)) >> self.shift) as usize
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn index(&self, occupied: Bitboard) -> usize {
        cfg_if! {
            if #[cfg(target_feature = "bmi2")] {
                std::arch::x86_64::_pext_u64(occupied.0, self.mask) as usize
            } else {
                self.const_index(occupied)
            }
        }
    }

    pub const fn const_attacks(&self, occupied: Bitboard) -> Bitboard {
        self.attacks[self.const_index(occupied)]
    }

    pub fn attacks(&self, occupied: Bitboard) -> Bitboard {
        self.attacks[self.index(occupied)]
    }

    cfg_if! {
        if #[cfg(not(target_feature = "bmi2"))] {
            pub const fn new(mask: Bitboard, magic: Bitboard, shift: u32) -> Self {
                Self { mask, attacks: &[], magic, shift }
            }
        } else {
            pub const fn new(mask: Bitboard) -> Self {
                Self { mask, attacks: &[] }
            }
        }
    }
}
