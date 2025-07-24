#[cfg(test)]
use super::*;
use std::str::FromStr;

fn expect_parse_ok(fen: &str) -> Board {
    let board = Board::from_str(fen);

    println!("{:?}", board.clone().err());
    assert!(board.is_ok(), "Should parse successfully: {fen}");

    board.unwrap()
}

fn expect_parse_err(fen: &str) {
    let board = Board::from_str(fen);

    println!("{:?}", board.clone().err());
    assert!(board.is_err(), "Should fail to parse: {fen}");
}

#[test]
fn test_valid_starting_position() {
    let board = expect_parse_ok("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    assert_eq!(board, Board::new_starting());
}

#[test]
fn test_valid_custom_position() {
    expect_parse_ok("r1bqkbnr/pppppppp/n7/8/8/8/PPPPPPPP/RNBQKBNR b KQ - 12 34");
}

#[test]
fn test_invalid_piece_in_castling_rights() {
    expect_parse_err("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQa - 0 1");
}

#[test]
fn test_invalid_rank_overflow() {
    expect_parse_err("rnbqkbnr/pppppppp/8/8/8/8/8/8/8 w - - 0 1"); // too many ranks
}

#[test]
fn test_invalid_rank_underflow() {
    expect_parse_err("rnbqkbnr/pppppppp/8/8/8/8/8 w - - 0 1"); // too few ranks
}

#[test]
fn test_too_many_squares_in_rank() {
    expect_parse_err("rnbqkbnr/pppppppp/9/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1");
}

#[test]
fn test_en_passant_invalid_format() {
    expect_parse_err("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - a 0 1"); // missing number
    expect_parse_err("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - a9 0 1"); // invalid square
}

#[test]
fn test_halfmove_non_numeric() {
    expect_parse_err("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - x 1");
}

#[test]
fn test_extra_characters() {
    expect_parse_err("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w - - 0 1 EXTRA");
}

#[test]
fn test_empty_board() {
    let board = expect_parse_ok("8/8/8/8/8/8/8/8 w - - 0 1");
    assert_eq!(board, Board::new_empty());
}

#[test]
fn test_invalid_characters_in_piece_placement() {
    expect_parse_err("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQXBNR w KQkq - 0 1");
}
