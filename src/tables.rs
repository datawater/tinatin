use crate::types::{BB, Bitboard};

pub const FILE_MASKS: [Bitboard; 8] = {
    let mut array = [Bitboard(0); 8];
    let mut i = 0;
    while i < 8 {
        array[i] = BB(0x101_0101_0101_0101 << i);
        i += 1;
    }

    array
};

pub const RANK_MASKS: [Bitboard; 8] = {
    let mut array = [Bitboard(0); 8];
    let mut i = 0;
    while i < 8 {
        array[i] = BB(0xff << (i * 8));
        i += 1;
    }

    array
};

// pub const DIAG_MASK: [Bitboard; 15] = [
//     Bitboard(0x8040_2010_0804_0201),
//     Bitboard(0x0080_4020_1008_0402),
//     Bitboard(0x0000_8040_2010_0804),
//     Bitboard(0x0000_0080_4020_1008),
//     Bitboard(0x0000_0000_8040_2010),
//     Bitboard(0x0000_0000_0080_4020),
//     Bitboard(0x0000_0000_0000_8040),
//     Bitboard(0x0000_0000_0000_0080),
//     Bitboard(0x0100_0000_0000_0000),
//     Bitboard(0x0201_0000_0000_0000),
//     Bitboard(0x0402_0100_0000_0000),
//     Bitboard(0x0804_0201_0000_0000),
//     Bitboard(0x1008_0402_0100_0000),
//     Bitboard(0x2010_0804_0201_0000),
//     Bitboard(0x4020_1008_0402_0100),
// ];

// pub const ANTIDIAG_MASK: [Bitboard; 15] = [
//     Bitboard(0x0102_0408_1020_4080),
//     Bitboard(0x0204_0810_2040_8000),
//     Bitboard(0x0408_1020_4080_0000),
//     Bitboard(0x0810_2040_8000_0000),
//     Bitboard(0x1020_4080_0000_0000),
//     Bitboard(0x2040_8000_0000_0000),
//     Bitboard(0x4080_0000_0000_0000),
//     Bitboard(0x8000_0000_0000_0000),
//     Bitboard(0x0000_0000_0000_0001),
//     Bitboard(0x0000_0000_0000_0102),
//     Bitboard(0x0000_0000_0001_0204),
//     Bitboard(0x0000_0000_0102_0408),
//     Bitboard(0x0000_0001_0204_0810),
//     Bitboard(0x0000_0102_0408_1020),
//     Bitboard(0x0001_0204_0810_2040),
// ];
