#![allow(clippy::cast_possible_truncation)]

#[cfg(test)]
mod test;

use std::fmt;
use std::str::FromStr;

use crate::types::{Bitboard, CastlingRights, Color, Piece, Square};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub(crate) struct BoardState {
    pub(crate) castling_rights: u8,
    pub(crate) rule_50: u8,
    pub(crate) ep_square: Square,

    pub(crate) attacks: [Bitboard; 2],
    pub(crate) checkers: Bitboard,
    pub(crate) previous: Option<Box<Self>>,
    pub(crate) king_blockers: [Bitboard; 2],
    pub(crate) pinners: [Bitboard; 2],
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board {
    pub(crate) mailbox: [Piece; 64],
    pub(crate) piece_bb: [Bitboard; Piece::N_PIECES],
    pub(crate) color_bb: [Bitboard; 2],
    pub(crate) piece_count: [u8; Piece::N_PIECES],
    pub(crate) side_to_move: Color,

    pub(crate) state: Box<BoardState>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            mailbox: [Piece::None; 64],
            piece_bb: Default::default(),
            color_bb: Default::default(),
            piece_count: Default::default(),
            side_to_move: Color::default(),
            state: Box::default(),
        }
    }
}

impl BoardState {
    #[inline]
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    #[allow(dead_code)]
    pub fn new_starting() -> Self {
        Self {
            castling_rights: (CastlingRights::WhiteAll | CastlingRights::BlackAll) as u8,
            ..Default::default()
        }
    }
}

impl FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut self_ = Self::default();

        let s = s.as_bytes();
        let mut i = 0;

        let mut rank = 8;
        let mut file = 1;
        let mut phase = 0;

        while i < s.len() {
            let c = s[i];

            if c == b' ' {
                phase += 1;
                i += 1;
                continue;
            }

            if phase == 1 {
                if rank != 1 {
                    return Err("Too few ranks".to_owned());
                }

                if !matches!(c.to_ascii_lowercase(), b'w' | b'b') {
                    return Err(format!("Invalid side to move '{}'", c as char));
                }

                self_.side_to_move = Color::from(c as char);

                i += 1;
                continue;
            } else if phase == 2 {
                if !matches!(c, b'K' | b'Q' | b'k' | b'q' | b'-') {
                    return Err(format!("Invalid castling rights '{}'", c as char));
                }

                self_.state.castling_rights |= CastlingRights::from(c as char).as_int();

                i += 1;
                continue;
            } else if phase == 3 {
                match c {
                    b'-' => {}
                    b'a'..=b'h' => {
                        if i + 1 >= s.len() || !matches!(s[i + 1], b'1'..=b'8') {
                            return Err("Invalid en passant square".to_owned());
                        }

                        self_.state.ep_square = unsafe {
                            Square::unsafe_from_str(&String::from_utf8_unchecked(vec![c, s[i + 1]]))
                        }
                    }

                    _ => return Err(format!("Did not expect charachter '{}'", c as char)),
                }

                i += 1;
                continue;
            } else if phase == 4 {
                if !c.is_ascii_digit() {
                    return Err("Invalid Halfmove clock".to_owned());
                }

                self_.state.rule_50 = self_.state.rule_50 * 10 + (c - b'0');

                i += 1;
                continue;
            } else if phase == 5 {
                if !c.is_ascii_digit() {
                    return Err("Invalid fullmove number".to_owned());
                }

                i += 1;
                continue;
            } else if phase > 5 {
                return Err("Invalid FEN, too many phases".to_owned());
            }

            match c {
                b'P' | b'N' | b'B' | b'R' | b'Q' | b'K' | b'p' | b'n' | b'b' | b'r' | b'q'
                | b'k' => {
                    let piece = Piece::from_char(c as char);
                    let square =
                        u32::from(Square::from_rank_file(rank as u8 - 1, file as u8 - 1).as_int());

                    self_.mailbox[rank * 8 + file - 9] = piece;
                    self_.piece_bb[piece.to_index()].0 |= 1 << square;
                    self_.color_bb[usize::from(piece.color().0)].0 |= 1 << square;
                    self_.piece_count[piece.to_index()] += 1;
                    file += 1;
                }

                b'1'..=b'8' => {
                    let n = c - b'0';
                    file += n as usize;
                    if file > 9 {
                        return Err("Too many squares on rank".to_owned());
                    }
                }

                b'/' => {
                    if file != 9 {
                        return Err("Not enough squares covered on rank".to_owned());
                    }

                    if rank == 1 {
                        return Err("Too many ranks".to_owned());
                    }

                    rank -= 1;
                    file = 1;
                }
                _ => return Err(format!("Invalid charachter '{}'", c as char)),
            }

            i += 1;
        }

        Ok(self_)
    }
}

impl Board {
    #[inline]
    #[allow(dead_code)]
    pub fn new_starting() -> Self {
        #[allow(clippy::enum_glob_use)]
        use crate::types::{BB, Piece::*};

        Board {
            mailbox: [
                WRook, WKnight, WBishop, WQueen, WKing, WBishop, WKnight, WRook, WPawn, WPawn,
                WPawn, WPawn, WPawn, WPawn, WPawn, WPawn, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, BPawn, BPawn,
                BPawn, BPawn, BPawn, BPawn, BPawn, BPawn, BRook, BKnight, BBishop, BQueen, BKing,
                BBishop, BKnight, BRook,
            ],
            piece_bb: [
                BB(0xff00),
                BB(0x42),
                BB(0x24),
                BB(0x81),
                BB(0x8),
                BB(0x10),
                BB(0xff_0000_0000_0000),
                BB(0x4200_0000_0000_0000),
                BB(0x2400_0000_0000_0000),
                BB(0x8100_0000_0000_0000),
                BB(0x800_0000_0000_0000),
                BB(0x1000_0000_0000_0000),
            ],
            color_bb: [BB(0xffff), BB(0xffff_0000_0000_0000)],
            piece_count: [8, 2, 2, 2, 1, 1, 8, 2, 2, 2, 1, 1],
            side_to_move: Color::WHITE,
            state: Box::new(BoardState::new_starting()),
        }
    }

    #[inline]
    #[allow(dead_code)]
    pub fn new_empty() -> Self {
        Board::default()
    }
}

impl fmt::Display for Board {
    // Stolen from stockfish
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = " ---------------------------------\n";

        f.write_str(s)?;

        for rank in (0..=7).rev() {
            for file in 0..=7 {
                f.write_fmt(format_args!(
                    " | {}",
                    self.mailbox[Square::from_rank_file(rank, file).as_int() as usize].to_char()
                ))?;
            }

            f.write_fmt(format_args!(" | {}\n{s}", rank + 1))?;
        }

        f.write_str("   a   b   c   d   e   f   g   h\n")?;
        f.write_fmt(format_args!(
            "\ncastling_rights: {}, rule_50: {}, ep_square: {:?}\n",
            self.state.castling_rights, self.state.rule_50, self.state.ep_square
        ))?;
        f.write_fmt(format_args!(
            "checkers: {:?}\nblockers: {:?}\npinners: {:?}\n",
            self.state.checkers, self.state.king_blockers, self.state.pinners
        ))?;

        Ok(())
    }
}
