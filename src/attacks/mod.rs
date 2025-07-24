pub mod magic;
mod non_sliding;
mod precomputed;
mod sliding;

use crate::types::{Bitboard, Piece, Square, BB};
use crate::board::Board;
use precomputed::{BISHOP_MAGICS, ROOK_MAGICS};
use non_sliding::NON_SLIDING_ATTACKS;
use sliding::{BISHOP_ATTACKS_TABLE, ROOK_ATTACKS_TABLE};

impl Piece {
    #[inline]
    pub fn attacks(self, from: Square, occupied: Bitboard) -> Bitboard {
        let from = from.as_int() as usize;

        if self.as_int() == Self::WPawn.as_int() || self.as_int() == Self::BPawn.as_int() {
            return NON_SLIDING_ATTACKS[if self.as_int() == Self::WPawn.as_int() {
                0
            } else {
                1
            }][from];
        }

        let t = unsafe { Self::from_int(self.type_of() as i8) };
        #[allow(unused_unsafe)]
        unsafe {
            match t {
                Piece::WKnight => NON_SLIDING_ATTACKS[2][from],
                Piece::WBishop => BISHOP_ATTACKS_TABLE[from][BISHOP_MAGICS[from].index(occupied)],
                Piece::WRook => ROOK_ATTACKS_TABLE[from][ROOK_MAGICS[from].index(occupied)],
                Piece::WQueen => {
                    BISHOP_ATTACKS_TABLE[from][BISHOP_MAGICS[from].index(occupied)]
                        | ROOK_ATTACKS_TABLE[from][ROOK_MAGICS[from].index(occupied)]
                }
                Piece::WKing => NON_SLIDING_ATTACKS[3][from],

                _ => {panic!("{t:?}")},
            }
        }
    }
}

impl Board {
    pub fn get_attacks(&self) -> Bitboard {
        let mut bb = BB(0);

        for s in 0..64 {
            let p = self.mailbox[s as usize]; 
            if p == Piece::None || p.color() != self.side_to_move {
                continue;
            }

            bb = bb | p.attacks(unsafe { Square::from_int(s) }, self.color_bb[0] | self.color_bb[1]);
        }

        bb
    }
}