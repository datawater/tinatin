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
