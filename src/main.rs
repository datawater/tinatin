#![warn(clippy::pedantic, clippy::missing_const_for_fn)]
#![deny(clippy::perf)]
#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    incomplete_features,
    long_running_const_eval
)]
#![feature(generic_const_exprs, generic_const_items)]
mod attacks;
mod board;
mod tables;
mod types;
mod utils;

use std::str::FromStr;

fn main() {
    let board =
        board::Board::from_str("R2BK3/p4pp1/2p5/4Q3/1N3p1p/7p/1pp3P1/8 w - - 0 1").unwrap();

    let attacks = board.get_attacks();

    println!(" {board}\n\n{attacks}");
}
