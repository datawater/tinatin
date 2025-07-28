#![warn(clippy::pedantic, clippy::missing_const_for_fn)]
#![deny(clippy::perf)]
#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    incomplete_features,
    long_running_const_eval
)]
#![feature(generic_const_exprs, generic_const_items, test)]
mod attacks;
mod board;
mod moves;
mod tables;
mod types;
mod utils;

use std::str::FromStr;

use crate::types::Color;

fn main() {
    let mut board =
        board::Board::from_str("rnbqkbnr/p1pp1Qp1/1p5p/1B2p3/4P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 4")
            .unwrap();
    board.populate_state();

    let attacks = board.get_attacks::<{ Color::WHITE.0 }>();
    println!("{board}\n\n{attacks}");
}