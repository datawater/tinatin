#![allow(dead_code)]
use std::fmt::Display;
use std::ops::{BitAnd, BitOr, BitXor, Not};
use std::str::FromStr;

use crate::tables::{FILE_MASKS, RANK_MASKS};
use crate::utils::mini8;
use crate::{enum_i8, enum_u8};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Bitboard(pub u64);

enum_u8! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    #[repr(u8)]
    pub enum CastlingRights {
        #[default]
        None,
        WhiteOO,
        WhiteOOO = (Self::WhiteOO as u8) << 1,
        BlackOO  = (Self::WhiteOOO as u8) << 1,
        BlackOOO = (Self::BlackOO as u8) << 1,
        WhiteAll = Self::WhiteOO as u8 | Self::WhiteOOO as u8,
        BlackAll = Self::BlackOO as u8 | Self::BlackOOO as u8,
        AllSidesAll = Self::WhiteAll as u8 | Self::BlackAll as u8,
    }
}

enum_u8! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    #[repr(u8)]
    pub enum Square {
        A1, B1, C1, D1, E1, F1, G1, H1,
        A2, B2, C2, D2, E2, F2, G2, H2,
        A3, B3, C3, D3, E3, F3, G3, H3,
        A4, B4, C4, D4, E4, F4, G4, H4,
        A5, B5, C5, D5, E5, F5, G5, H5,
        A6, B6, C6, D6, E6, F6, G6, H6,
        A7, B7, C7, D7, E7, F7, G7, H7,
        A8, B8, C8, D8, E8, F8, G8, H8,
        #[default]
        None,
    }
}

enum_i8! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    #[repr(i8)]
    pub enum Piece {
        WPawn = 1,
        WKnight,
        WBishop,
        WRook,
        WQueen,
        WKing,
        BPawn = -(Self::WPawn as i8),
        BKnight = -(Self::WKnight as i8),
        BBishop = -(Self::WBishop as i8),
        BRook = -(Self::WRook as i8),
        BQueen = -(Self::WQueen as i8),
        BKing = -(Self::WKing as i8),
        #[default]
        None = 0,
    }
}

enum_i8! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
    #[repr(i8)]
    pub enum Direction {
        North = 8,
        NorthNorth = 16,
        South = -(Self::North as i8),
        SouthSouth = -(Self::NorthNorth as i8),

        East = 1,
        EastEast = 2,
        West = -(Self::East as i8),
        WestWest = -(Self::EastEast as i8),

        NorthEast = Self::North as i8 + Self::East as i8,
        SouthEast = Self::South as i8 + Self::East as i8,
        NorthWest = Self::North as i8 + Self::West as i8,
        SouthWest = Self::South as i8 + Self::West as i8,

        #[default]
        None = 0,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct File(pub usize);
impl File {
    pub const A: Self = Self(0);
    pub const B: Self = Self(1);
    pub const C: Self = Self(2);
    pub const D: Self = Self(3);
    pub const E: Self = Self(4);
    pub const F: Self = Self(5);
    pub const G: Self = Self(6);
    pub const H: Self = Self(7);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Rank(pub usize);
impl Rank {
    pub const ONE: Self = Self(0);
    pub const TWO: Self = Self(1);
    pub const THREE: Self = Self(2);
    pub const FOUR: Self = Self(3);
    pub const FIVE: Self = Self(4);
    pub const SIX: Self = Self(5);
    pub const SEVEN: Self = Self(6);
    pub const EIGHT: Self = Self(7);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Color(pub bool);
impl Color {
    pub const WHITE: Self = Self(false);
    pub const BLACK: Self = Self(true);
}

#[allow(non_snake_case)]
pub const fn BB(b: u64) -> Bitboard {
    Bitboard(b)
}

impl Piece {
    pub const N_PIECES: usize = 12;

    pub const fn is_white(self) -> bool {
        self.as_int().is_positive()
    }

    pub const fn is_black(self) -> bool {
        self.as_int().is_negative()
    }

    pub const fn from_char(c: char) -> Self {
        match c {
            'P' => Self::WPawn,
            'N' => Self::WKnight,
            'B' => Self::WBishop,
            'R' => Self::WRook,
            'Q' => Self::WQueen,
            'K' => Self::WKing,
            'p' => Self::BPawn,
            'n' => Self::BKnight,
            'b' => Self::BBishop,
            'r' => Self::BRook,
            'q' => Self::BQueen,
            'k' => Self::BKing,

            _ => Self::None,
        }
    }

    pub const fn to_char(self) -> char {
        match self {
            Self::WPawn => 'P',
            Self::WKnight => 'N',
            Self::WBishop => 'B',
            Self::WRook => 'R',
            Self::WQueen => 'Q',
            Self::WKing => 'K',
            Self::BPawn => 'p',
            Self::BKnight => 'n',
            Self::BBishop => 'b',
            Self::BRook => 'r',
            Self::BQueen => 'q',
            Self::BKing => 'k',

            Self::None => ' ',
        }
    }

    pub const fn type_of(self) -> u8 {
        self.as_int().unsigned_abs()
    }

    pub const fn color(self) -> Color {
        Color(self.as_int().is_negative())
    }

    pub(crate) const fn as_index(self) -> usize {
        let x = self.as_int() as isize;
        (x.abs() - 1 + (x.is_negative() as isize) * Self::WKing as isize) as usize
    }
}

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl From<char> for Color {
    fn from(mut v: char) -> Self {
        v = v.to_ascii_lowercase();
        debug_assert!(matches!(v, 'w' | 'b'));

        Self(v == 'b')
    }
}

impl Square {
    pub const fn from_rank_file(rank: u8, file: u8) -> Self {
        unsafe { Self::from_int(rank * 8 + file) }
    }

    pub const fn to_bitboard(self) -> Bitboard {
        BB(1u64 << self.as_int())
    }

    pub const fn rank(self) -> u8 {
        self.as_int().unbounded_shr(3)
    }

    pub const fn file(self) -> u8 {
        self.as_int() & 7
    }

    pub const fn distance_from_edge(self, d: Direction) -> u8 {
        #[allow(clippy::enum_glob_use)]
        use Direction::*;

        match d {
            North => 7 - self.rank(),
            NorthNorth => (8 - self.rank()) >> 1,
            South => self.rank(),
            SouthSouth => (self.rank() + 1) >> 1,
            East => 7 - self.file(),
            EastEast => (8 - self.file()) >> 1,
            West => self.file(),
            WestWest => (self.file() + 1) >> 1,
            NorthEast => mini8(
                self.distance_from_edge(North) as i8,
                self.distance_from_edge(East) as i8,
            ) as u8,
            SouthEast => mini8(
                self.distance_from_edge(South) as i8,
                self.distance_from_edge(East) as i8,
            ) as u8,
            NorthWest => mini8(
                self.distance_from_edge(North) as i8,
                self.distance_from_edge(West) as i8,
            ) as u8,
            SouthWest => mini8(
                self.distance_from_edge(South) as i8,
                self.distance_from_edge(West) as i8,
            ) as u8,
            None => 255,
        }
    }
}

impl FromStr for Square {
    type Err = ();

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        if str.len() != 2 {
            return Err(());
        }

        let s = str.to_lowercase();
        let s = s.as_bytes();

        if !matches!(s[0], b'a'..=b'h') || !matches!(s[1], b'1'..=b'8') {
            return Err(());
        }

        unsafe { Ok(Self::unsafe_from_str(str)) }
    }
}

impl Square {
    pub unsafe fn unsafe_from_str(s: &str) -> Self {
        let s = s.to_ascii_lowercase();
        let s = s.as_bytes();

        Self::from_rank_file(s[1] - b'1', s[0] - b'a')
    }
}

impl From<Square> for Bitboard {
    fn from(v: Square) -> Self {
        Self(1u64 << v.as_int())
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl Bitboard {
    pub const fn shift_by_direction(self, d: Direction) -> Self {
        #[allow(clippy::enum_glob_use)]
        use Direction::*;

        BB(match d {
            North => self.0 << 8,
            NorthNorth => self.0 << 16,
            South => self.0 >> 8,
            SouthSouth => self.0 >> 16,

            East => (self.0 & File::H.clear().0) << 1,
            EastEast => self.shift_by_direction(East).shift_by_direction(East).0,
            West => (self.0 & File::A.clear().0) >> 1,
            WestWest => self.shift_by_direction(West).shift_by_direction(West).0,

            NorthEast => self.shift_by_direction(North).shift_by_direction(East).0,
            SouthEast => self.shift_by_direction(South).shift_by_direction(East).0,
            NorthWest => self.shift_by_direction(North).shift_by_direction(West).0,
            SouthWest => self.shift_by_direction(South).shift_by_direction(West).0,
            None => self.0,
        })
    }
}

impl Display for Bitboard {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (0..=7)
            .rev()
            .map(|r| {
                (0..=7)
                    .map(|f| {
                        let s = Square::from_rank_file(r, f).as_int();
                        write!(
                            fmt,
                            "{}",
                            if (1u64 << s) & self.0 != 0 {
                                "x "
                            } else {
                                ". "
                            }
                        )
                        .and(if (s + 1).is_multiple_of(8) {
                            writeln!(fmt)
                        } else {
                            Ok(())
                        })
                    })
                    .reduce(Result::and)
                    .unwrap_or(Ok(()))
            })
            .reduce(Result::and)
            .unwrap_or(Ok(()))
    }
}

impl From<char> for CastlingRights {
    fn from(c: char) -> Self {
        debug_assert!(matches!(c, 'K' | 'Q' | 'k' | 'q' | '-'));

        match c {
            'K' => Self::WhiteOO,
            'Q' => Self::WhiteOOO,
            'k' => Self::BlackOO,
            'q' => Self::BlackOOO,
            _ => Self::None,
        }
    }
}

impl FromStr for CastlingRights {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s
            .chars()
            .map(|x| {
                if matches!(x, 'K' | 'Q' | 'k' | 'q') {
                    Self::from(x).as_int()
                } else {
                    255
                }
            })
            .fold(0, |acc, x| acc | x);

        if x == 255 {
            return Err("Invalid castling rights".to_owned());
        }

        Ok(unsafe { CastlingRights::from_int(x) })
    }
}

impl BitOr for CastlingRights {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { Self::from_int(self.as_int() | rhs.as_int()) }
    }
}

impl CastlingRights {
    pub unsafe fn unsafe_from_str(s: &str) -> Self {
        unsafe {
            Self::from_int(
                s.chars()
                    .map(|x| Self::from(x).as_int())
                    .fold(0, |acc, x| acc | x),
            )
        }
    }
}

impl Rank {
    pub const fn mask(self) -> Bitboard {
        RANK_MASKS[self.0]
    }

    pub const fn clear(self) -> Bitboard {
        BB(!RANK_MASKS[self.0].0)
    }
}

impl File {
    pub const fn mask(self) -> Bitboard {
        FILE_MASKS[self.0]
    }

    pub const fn clear(self) -> Bitboard {
        BB(!FILE_MASKS[self.0].0)
    }
}
