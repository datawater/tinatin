use crate::types::Direction::{NorthEast, NorthWest, SouthEast, SouthWest};
use crate::types::{BB, Bitboard, Square};

const fn generate_pawn_attacks(a: &mut [Bitboard; 64], white: bool) {
    let mut i = 0;
    while i < 64 {
        let b = BB(1u64 << i);
        a[i] = BB(if white {
            b.shift_by_direction(NorthEast).0 | b.shift_by_direction(NorthWest).0
        } else {
            b.shift_by_direction(SouthEast).0 | b.shift_by_direction(SouthWest).0
        });

        i += 1;
    }
}

const fn safe_step(s: Square, step: i8) -> Bitboard {
    let s = s.as_int() as i8;
    let sn = s + step;

    if (sn >= 0 && sn < 64)
        && crate::utils::maxi8(
            ((s & 7) - (sn & 7)).abs(),
            (s.unbounded_shr(3) - sn.unbounded_shr(3)).abs(),
        ) <= 2
    {
        BB(1u64 << sn)
    } else {
        BB(0u64)
    }
}

const fn generate_attacks_via_steps<const N: usize>(a: &mut [Bitboard; 64], steps: [i8; N]) {
    let mut i = 0;

    while i < 64 {
        let mut j = 0;
        while j < N {
            a[i].0 |= safe_step(unsafe { Square::from_int(i as u8) }, steps[j]).0;
            j += 1;
        }

        i += 1;
    }
}

const fn generate_knight_attacks(a: &mut [Bitboard; 64]) {
    let steps = [-17, -15, -10, -6, 6, 10, 15, 17];
    generate_attacks_via_steps(a, steps);
}

const fn generate_king_attacks(a: &mut [Bitboard; 64]) {
    let steps = [-9, -8, -7, -1, 1, 7, 8, 9];
    generate_attacks_via_steps(a, steps);
}

pub(super) const NON_SLIDING_ATTACKS: [[Bitboard; 64]; 4] = {
    let mut array = [[Bitboard(0); 64]; 4];

    generate_pawn_attacks(&mut array[0], true);
    generate_pawn_attacks(&mut array[1], false);

    generate_knight_attacks(&mut array[2]);

    generate_king_attacks(&mut array[3]);

    array
};
