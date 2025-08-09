#![allow(dead_code)]

use std::fmt::Display;

use crate::board::Board;
use crate::types::{Color, Piece, Square};

// Should I reduce the size of this array? yes. Will I? no.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub moved: Piece,
    pub captured: Piece,
    pub promotion: Option<Piece>,
    pub is_castling: bool,
}

const MAX_MOVES: usize = 256;
pub type MoveList = smallvec::SmallVec<[Move; 32]>;

impl Display for Move {
    // UCI
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.from, self.to)?;
        if let Some(promotion) = self.promotion {
            write!(f, "={}", promotion.type_of_to_piece().to_char())?;
        }

        Ok(())
    }
}

impl Board {
    pub fn generate_moves(&self) -> MoveList {
        let mut movelist = MoveList::default();

        if !self.state.checkers.0.is_power_of_two() {
            let king = self.piece_bb[(if self.side_to_move == Color::WHITE {
                Piece::WKing
            } else {
                Piece::BKing
            })
            .to_index()];
        }

        movelist
    }
}
