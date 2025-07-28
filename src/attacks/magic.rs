use crate::types::{BB, Bitboard};
use cfg_if::cfg_if;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Magic {
    pub mask: Bitboard,

    #[cfg(not(target_feature = "bmi2"))]
    #[allow(clippy::struct_field_names)]
    magic: Bitboard,
    #[cfg(not(target_feature = "bmi2"))]
    shift: u32,
}

impl Magic {
    const fn _empty() -> Magic {
        cfg_if! {
            if #[cfg(not(target_feature = "bmi2"))] {
                Magic {
                    mask: BB(0),
                    magic: BB(0),
                    shift: 0
                }
            } else {
                Magic {
                    mask: BB(0),
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

    cfg_if! {
        if #[cfg(not(target_feature = "bmi2"))] {
            pub const fn new(mask: Bitboard, magic: Bitboard, shift: u32) -> Self {
                Self { mask, magic, shift }
            }
        } else {
            pub const fn new(mask: Bitboard) -> Self {
                Self { mask }
            }
        }
    }
}
