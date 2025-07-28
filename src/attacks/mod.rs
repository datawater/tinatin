pub mod magic;
mod non_sliding;
mod precomputed;
mod sliding;

use crate::board::Board;
use crate::types::{BB, Bitboard, Color, Direction, Piece, Square};
use non_sliding::NON_SLIDING_ATTACKS;
use precomputed::{BISHOP_MAGICS, ROOK_MAGICS};
use sliding::{BISHOP_ATTACKS_TABLE, ROOK_ATTACKS_TABLE};

#[allow(clippy::cast_lossless)]
impl Piece {
    #[inline]
    pub fn attacks(self, from: Square, occupied: Bitboard) -> Bitboard {
        let from = from.as_int() as usize;

        if self.as_int() == Self::WPawn.as_int() || self.as_int() == Self::BPawn.as_int() {
            return NON_SLIDING_ATTACKS[(self.as_int() != Self::WPawn.as_int()) as usize][from];
        }

        let t = unsafe { Self::from_int(self.type_of() as i8) };
        #[allow(unused_unsafe)]
        unsafe {
            match t {
                Piece::WKnight => NON_SLIDING_ATTACKS[2][from],
                Piece::WBishop => BISHOP_ATTACKS_TABLE[from][BISHOP_MAGICS[from].index(occupied)],
                Piece::WRook => ROOK_ATTACKS_TABLE[from][ROOK_MAGICS[from].index(occupied)],
                Piece::WQueen => {
                    BB(BISHOP_ATTACKS_TABLE[from][BISHOP_MAGICS[from].index(occupied)].0
                        | ROOK_ATTACKS_TABLE[from][ROOK_MAGICS[from].index(occupied)].0)
                }
                Piece::WKing => NON_SLIDING_ATTACKS[3][from],

                _ => {
                    unreachable!();
                }
            }
        }
    }
}

const fn piece_can_travel_in_sliding_direction(piece: Piece, direction: Direction) -> bool {
    let piece = piece.type_of_to_piece();
    match direction {
        Direction::North => matches!(piece, Piece::WRook | Piece::WQueen),
        Direction::South => matches!(piece, Piece::WRook | Piece::WQueen),
        Direction::East => matches!(piece, Piece::WRook | Piece::WQueen),
        Direction::West => matches!(piece, Piece::WRook | Piece::WQueen),
        Direction::NorthEast => matches!(piece, Piece::WBishop | Piece::WQueen),
        Direction::SouthEast => matches!(piece, Piece::WBishop | Piece::WQueen),
        Direction::NorthWest => matches!(piece, Piece::WBishop | Piece::WQueen),
        Direction::SouthWest => matches!(piece, Piece::WBishop | Piece::WQueen),

        _ => false,
    }
}

// Implemented in a way that can be turned into a const function
#[allow(clippy::cast_lossless)]
impl Board {
    pub const fn get_attacks<const SIDE: bool>(&self) -> Bitboard {
        self.state.attacks[Color(SIDE).0 as usize]
    }

    fn populate_attacks(&mut self) {
        self.state.attacks = {
            let mut ar = [BB(0); 2];

            let mut sidei = 0;
            while sidei < ar.len() {
                let bb = &mut ar[sidei];
                let side = Color(sidei != 0);
                let king = self.piece_bb[(if side.0 == Color::WHITE.0 {
                    Piece::BKing
                } else {
                    Piece::WKing
                })
                .to_index()];

                let mut index = 6 * (Color::WHITE.0 != side.0) as usize;
                while index < if side.0 == Color::WHITE.0 {6} else {12} {
                    let mut piece = self.piece_bb[index];
                    while piece.0 != BB(0).0 {
                        let square = unsafe { Square::from_int(piece.0.trailing_zeros() as u8) };
                        let attacks = Piece::from_index(index)
                            .attacks(square, BB(self.color_bb[0].0 | self.color_bb[1].0));

                        if king.0 & attacks.0 != 0
                            && piece.0 & self.color_bb[side.0 as usize].0 != 0
                        {
                            self.state.checkers.0 |= square.to_bitboard().0;
                        }

                        bb.0 |= attacks.0;
                        piece.0 &= !square.to_bitboard().0;
                    }

                    index += 1;
                }

                sidei += 1;
            }

            ar
        };
    }

    fn populate_pinners_and_blockers(&mut self) {
        let mut side = 0;
        while side < 2 {
            let color_bb = self.color_bb[side].0;
            let inverse_color_bb = self.color_bb[(side == 0) as usize].0;
            let occupancies = color_bb | inverse_color_bb;

            let king = self.piece_bb[if Color(side != 0).0 == Color::WHITE.0 {
                Piece::WKing
            } else {
                Piece::BKing
            }
            .to_index()];
            let king_square = unsafe { Square::from_int(king.0.trailing_zeros() as u8) };

            let directions = [
                Direction::North,
                Direction::South,
                Direction::East,
                Direction::West,
                Direction::NorthEast,
                Direction::NorthWest,
                Direction::SouthEast,
                Direction::SouthWest,
            ];

            let mut di = 0;

            while di < directions.len() {
                let direction = directions[di];
                let blocker =
                    sliding::sliding_attacks(king_square, BB(occupancies), direction).0 & color_bb;

                if blocker == 0 {
                    di += 1;
                    continue;
                }

                let blocker = if direction.is_increasing() {
                    blocker & (1u64 << 63).wrapping_shr(blocker.leading_zeros())
                } else {
                    blocker & blocker.wrapping_neg()
                };

                let blocker_square = unsafe { Square::from_int(blocker.trailing_zeros() as u8) };
                let pinner = sliding::sliding_attacks(blocker_square, BB(occupancies), direction).0
                    & inverse_color_bb;

                if pinner == 0 {
                    di += 1;
                    continue;
                }

                let pinner = if direction.is_increasing() {
                    pinner & (1u64 << 63).wrapping_shr(pinner.leading_zeros())
                } else {
                    pinner & pinner.wrapping_neg()
                };

                let pinner_piece = self.mailbox[pinner.trailing_zeros() as usize];
                if !piece_can_travel_in_sliding_direction(pinner_piece, direction) {
                    di += 1;
                    continue;
                }

                self.state.pinners[side].0 |= pinner;
                self.state.king_blockers[side].0 |= blocker;

                di += 1;
            }

            side += 1;
        }
    }

    pub fn populate_state(&mut self) {
        self.populate_attacks();
        self.populate_pinners_and_blockers();
    }
}
