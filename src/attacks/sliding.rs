#![allow(dead_code)]

use super::magic::Magic;
use crate::attacks::precomputed::{BISHOP_MAGICS, ROOK_MAGICS};
use crate::types::{BB, Bitboard, Direction, Square};

// Stolen from https://github.com/aronpetko/integral/blob/main/src/magics/attacks.cc

macro_rules! sliding_ {
    ($starting:expr, $from:expr, $occupied:expr, $direction:expr, $check:expr) => {
        let mut attacks = BB(0);
        let mut from_bb = $from.to_bitboard();

        let mut i = $starting;
        while i < $from.distance_from_edge($direction) {
            from_bb = from_bb.shift_by_direction($direction);
            attacks.0 |= from_bb.0;

            if $check($occupied, attacks) {
                break;
            }

            i += 1;
        }

        return attacks;
    };
}

const fn sliding_attacks(from: Square, occupied: Bitboard, direction: Direction) -> Bitboard {
    const fn and_collision(a: Bitboard, b: Bitboard) -> bool {
        a.0 & b.0 != 0
    }

    sliding_!(0, from, occupied, direction, and_collision);
}

const fn sliding_occupancies(from: Square, direction: Direction) -> Bitboard {
    const fn no_check(_a: Bitboard, _b: Bitboard) -> bool {
        false
    }

    sliding_!(0, from, BB(0), direction, no_check);
}

const fn const_popcnt(mut x: u64) -> usize {
    const M: [u64; 4] = [
        0x5555_5555_5555_5555,
        0x3333_3333_3333_3333,
        0x0f0f_0f0f_0f0f_0f0f,
        0x0101_0101_0101_0101,
    ];

    x -= (x >> 1) & M[0];
    x = (x & M[1]) + ((x >> 2) & M[1]);
    x = (x + (x >> 4)) & M[2];
    (x.wrapping_mul(M[3]) >> 56) as usize
}

const fn blockers_size(x: u64) -> usize {
    1 + (1 << const_popcnt(x))
}

// Basically stolen from integral by aronpetko. modified to fit into const though
struct BlockingSet<const B: u64>();
impl<const MOVES: u64> BlockingSet<MOVES> {
    pub const SET: [Bitboard; blockers_size(MOVES)] = {
        let mut set_bits = [0u8; const_popcnt(MOVES)];
        
        let mut i = 0 ;
        let mut set_bits_i = 0;
        while i < 64 {
            if MOVES & (1u64 << i) != 0 {
                set_bits[set_bits_i] = i;
                set_bits_i += 1;
            }
            i += 1;
        }
        
        let mut subset = MOVES;
        
        let permutations = blockers_size(MOVES);
        let mut blockers = [BB(0); blockers_size(MOVES)];
        let mut blockers_i = 0;
        
        let mut i = 0;
        while i < permutations {
            let mut blocker = BB(0);
            
            let mut j = 0;
            while j < set_bits_i {
                let bit = 1u64 << set_bits[j];
                if subset & bit != 0 {
                    blocker.0 |= bit;
                }
                j += 1;
            }
        
            blockers[blockers_i] = blocker;
            blockers_i += 1;
            subset = subset.wrapping_sub(1) & MOVES;
        
            i += 1;
        }
        
        blockers
    } where [(); blockers_size(MOVES)]:, [(); const_popcnt(MOVES)]:;
}

const fn sliding_moves(from: Square, occupied: Bitboard, directions: &[Direction]) -> Bitboard {
    let mut moves = 0;
    let mut i = 0;
    while i < directions.len() {
        moves |= sliding_attacks(from, occupied, directions[i]).0;
        i += 1;
    }

    BB(moves)
}

const fn bishop_moves(from: Square, occupied: Bitboard) -> Bitboard {
    let directions = [
        Direction::NorthWest,
        Direction::SouthWest,
        Direction::NorthEast,
        Direction::SouthEast,
    ];
    sliding_moves(from, occupied, &directions)
}

const fn rook_moves(from: Square, occupied: Bitboard) -> Bitboard {
    let directions = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];
    sliding_moves(from, occupied, &directions)
}

const fn generate_attacks<const INDEX: u64>(
    table: &mut [Bitboard],
    square: Square,
    magic: Magic,
    is_rook: bool,
) where
    [(); const_popcnt(INDEX)]:,
    [(); blockers_size(INDEX)]:,
{
    let blockers = BlockingSet::<{ INDEX }>::SET;

    let mut i = 0;
    while i < blockers.len() {
        let occupied = blockers[i];
        let magic_index = magic.const_index(occupied);
        table[magic_index] = if is_rook {
            rook_moves(square, occupied)
        } else {
            bishop_moves(square, occupied)
        };

        i += 1;
    }
}

#[rustfmt::skip]
macro_rules! populate_array_64 {
    ($f:ident, $a1:ident, $a2:ident, $a3:ident, $a4:ident) => {{
        $f::<{ $a3[0].mask.0 }>(&mut $a1[0], unsafe { Square::$a2(0) }, $a3[0], $a4); $f::<{ $a3[1].mask.0 }>(&mut $a1[1], unsafe { Square::$a2(1) }, $a3[1], $a4); $f::<{ $a3[2].mask.0 }>(&mut $a1[2], unsafe { Square::$a2(2) }, $a3[2], $a4); $f::<{ $a3[3].mask.0 }>(&mut $a1[3], unsafe { Square::$a2(3) }, $a3[3], $a4); $f::<{ $a3[4].mask.0 }>(&mut $a1[4], unsafe { Square::$a2(4) }, $a3[4], $a4); $f::<{ $a3[5].mask.0 }>(&mut $a1[5], unsafe { Square::$a2(5) }, $a3[5], $a4); $f::<{ $a3[6].mask.0 }>(&mut $a1[6], unsafe { Square::$a2(6) }, $a3[6], $a4); $f::<{ $a3[7].mask.0 }>(&mut $a1[7], unsafe { Square::$a2(7) }, $a3[7], $a4);
        $f::<{ $a3[8].mask.0 }>(&mut $a1[8], unsafe { Square::$a2(8) }, $a3[8], $a4); $f::<{ $a3[9].mask.0 }>(&mut $a1[9], unsafe { Square::$a2(9) }, $a3[9], $a4); $f::<{ $a3[10].mask.0 }>(&mut $a1[10], unsafe { Square::$a2(10) }, $a3[10], $a4); $f::<{ $a3[11].mask.0 }>(&mut $a1[11], unsafe { Square::$a2(11) }, $a3[11], $a4); $f::<{ $a3[12].mask.0 }>(&mut $a1[12], unsafe { Square::$a2(12) }, $a3[12], $a4); $f::<{ $a3[13].mask.0 }>(&mut $a1[13], unsafe { Square::$a2(13) }, $a3[13], $a4); $f::<{ $a3[14].mask.0 }>(&mut $a1[14], unsafe { Square::$a2(14) }, $a3[14], $a4); $f::<{ $a3[15].mask.0 }>(&mut $a1[15], unsafe { Square::$a2(15) }, $a3[15], $a4);
        $f::<{ $a3[16].mask.0 }>(&mut $a1[16], unsafe { Square::$a2(16) }, $a3[16], $a4); $f::<{ $a3[17].mask.0 }>(&mut $a1[17], unsafe { Square::$a2(17) }, $a3[17], $a4); $f::<{ $a3[18].mask.0 }>(&mut $a1[18], unsafe { Square::$a2(18) }, $a3[18], $a4); $f::<{ $a3[19].mask.0 }>(&mut $a1[19], unsafe { Square::$a2(19) }, $a3[19], $a4); $f::<{ $a3[20].mask.0 }>(&mut $a1[20], unsafe { Square::$a2(20) }, $a3[20], $a4); $f::<{ $a3[21].mask.0 }>(&mut $a1[21], unsafe { Square::$a2(21) }, $a3[21], $a4); $f::<{ $a3[22].mask.0 }>(&mut $a1[22], unsafe { Square::$a2(22) }, $a3[22], $a4); $f::<{ $a3[23].mask.0 }>(&mut $a1[23], unsafe { Square::$a2(23) }, $a3[23], $a4);
        $f::<{ $a3[24].mask.0 }>(&mut $a1[24], unsafe { Square::$a2(24) }, $a3[24], $a4); $f::<{ $a3[25].mask.0 }>(&mut $a1[25], unsafe { Square::$a2(25) }, $a3[25], $a4); $f::<{ $a3[26].mask.0 }>(&mut $a1[26], unsafe { Square::$a2(26) }, $a3[26], $a4); $f::<{ $a3[27].mask.0 }>(&mut $a1[27], unsafe { Square::$a2(27) }, $a3[27], $a4); $f::<{ $a3[28].mask.0 }>(&mut $a1[28], unsafe { Square::$a2(28) }, $a3[28], $a4); $f::<{ $a3[29].mask.0 }>(&mut $a1[29], unsafe { Square::$a2(29) }, $a3[29], $a4); $f::<{ $a3[30].mask.0 }>(&mut $a1[30], unsafe { Square::$a2(30) }, $a3[30], $a4); $f::<{ $a3[31].mask.0 }>(&mut $a1[31], unsafe { Square::$a2(31) }, $a3[31], $a4);
        $f::<{ $a3[32].mask.0 }>(&mut $a1[32], unsafe { Square::$a2(32) }, $a3[32], $a4); $f::<{ $a3[33].mask.0 }>(&mut $a1[33], unsafe { Square::$a2(33) }, $a3[33], $a4); $f::<{ $a3[34].mask.0 }>(&mut $a1[34], unsafe { Square::$a2(34) }, $a3[34], $a4); $f::<{ $a3[35].mask.0 }>(&mut $a1[35], unsafe { Square::$a2(35) }, $a3[35], $a4); $f::<{ $a3[36].mask.0 }>(&mut $a1[36], unsafe { Square::$a2(36) }, $a3[36], $a4); $f::<{ $a3[37].mask.0 }>(&mut $a1[37], unsafe { Square::$a2(37) }, $a3[37], $a4); $f::<{ $a3[38].mask.0 }>(&mut $a1[38], unsafe { Square::$a2(38) }, $a3[38], $a4); $f::<{ $a3[39].mask.0 }>(&mut $a1[39], unsafe { Square::$a2(39) }, $a3[39], $a4);
        $f::<{ $a3[40].mask.0 }>(&mut $a1[40], unsafe { Square::$a2(40) }, $a3[40], $a4); $f::<{ $a3[41].mask.0 }>(&mut $a1[41], unsafe { Square::$a2(41) }, $a3[41], $a4); $f::<{ $a3[42].mask.0 }>(&mut $a1[42], unsafe { Square::$a2(42) }, $a3[42], $a4); $f::<{ $a3[43].mask.0 }>(&mut $a1[43], unsafe { Square::$a2(43) }, $a3[43], $a4); $f::<{ $a3[44].mask.0 }>(&mut $a1[44], unsafe { Square::$a2(44) }, $a3[44], $a4); $f::<{ $a3[45].mask.0 }>(&mut $a1[45], unsafe { Square::$a2(45) }, $a3[45], $a4); $f::<{ $a3[46].mask.0 }>(&mut $a1[46], unsafe { Square::$a2(46) }, $a3[46], $a4); $f::<{ $a3[47].mask.0 }>(&mut $a1[47], unsafe { Square::$a2(47) }, $a3[47], $a4);
        $f::<{ $a3[48].mask.0 }>(&mut $a1[48], unsafe { Square::$a2(48) }, $a3[48], $a4); $f::<{ $a3[49].mask.0 }>(&mut $a1[49], unsafe { Square::$a2(49) }, $a3[49], $a4); $f::<{ $a3[50].mask.0 }>(&mut $a1[50], unsafe { Square::$a2(50) }, $a3[50], $a4); $f::<{ $a3[51].mask.0 }>(&mut $a1[51], unsafe { Square::$a2(51) }, $a3[51], $a4); $f::<{ $a3[52].mask.0 }>(&mut $a1[52], unsafe { Square::$a2(52) }, $a3[52], $a4); $f::<{ $a3[53].mask.0 }>(&mut $a1[53], unsafe { Square::$a2(53) }, $a3[53], $a4); $f::<{ $a3[54].mask.0 }>(&mut $a1[54], unsafe { Square::$a2(54) }, $a3[54], $a4); $f::<{ $a3[55].mask.0 }>(&mut $a1[55], unsafe { Square::$a2(55) }, $a3[55], $a4);
        $f::<{ $a3[56].mask.0 }>(&mut $a1[56], unsafe { Square::$a2(56) }, $a3[56], $a4); $f::<{ $a3[57].mask.0 }>(&mut $a1[57], unsafe { Square::$a2(57) }, $a3[57], $a4); $f::<{ $a3[58].mask.0 }>(&mut $a1[58], unsafe { Square::$a2(58) }, $a3[58], $a4); $f::<{ $a3[59].mask.0 }>(&mut $a1[59], unsafe { Square::$a2(59) }, $a3[59], $a4); $f::<{ $a3[60].mask.0 }>(&mut $a1[60], unsafe { Square::$a2(60) }, $a3[60], $a4); $f::<{ $a3[61].mask.0 }>(&mut $a1[61], unsafe { Square::$a2(61) }, $a3[61], $a4); $f::<{ $a3[62].mask.0 }>(&mut $a1[62], unsafe { Square::$a2(62) }, $a3[62], $a4); $f::<{ $a3[63].mask.0 }>(&mut $a1[63], unsafe { Square::$a2(63) }, $a3[63], $a4);
    }};
}

#[cfg(not(debug_assertions))]
pub(super) static BISHOP_ATTACKS_TABLE: [[Bitboard; 512]; 64] = {
    let mut table = [[BB(0); 512]; 64];
    populate_array_64!(generate_attacks, table, from_int, BISHOP_MAGICS, false);

    table
};

#[cfg(not(debug_assertions))]
pub(super) static ROOK_ATTACKS_TABLE: [[Bitboard; 4096]; 64] = {
    let mut table = [[BB(0); 4096]; 64];
    populate_array_64!(generate_attacks, table, from_int, ROOK_MAGICS, true);

    table
};

#[cfg(debug_assertions)]
pub(super) static mut BISHOP_ATTACKS_TABLE: [[Bitboard; 512]; 64] = [[BB(0); 512]; 64];
#[cfg(debug_assertions)]
pub(super) static mut ROOK_ATTACKS_TABLE: [[Bitboard; 4096]; 64] = [[BB(0); 4096]; 64];

#[cfg(debug_assertions)]
#[ctor::ctor]
unsafe fn magic_init() {
    #[allow(unused_unsafe)]
    unsafe {
        populate_array_64!(
            generate_attacks,
            BISHOP_ATTACKS_TABLE,
            from_int,
            BISHOP_MAGICS,
            false
        );
    };
    #[allow(unused_unsafe)]
    unsafe {
        populate_array_64!(
            generate_attacks,
            ROOK_ATTACKS_TABLE,
            from_int,
            ROOK_MAGICS,
            true
        );
    };
}
