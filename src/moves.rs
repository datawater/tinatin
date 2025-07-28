#![allow(dead_code)]
use std::fmt::Display;

use crate::types::{Piece, Square};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub moved: Piece,
    pub captured: Piece,
    pub promotion: Option<Piece>,
    pub is_castling: bool,
}

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
