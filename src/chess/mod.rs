mod board;
mod piece;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
struct Piece {
    piece_type: PieceType,
    color: PieceColor,
    row: u8,
    col: u8,
}

#[derive(Debug, Clone)]
struct Move {
    to: (u8, u8),
    capture: Option<Piece>,
    promotion: Option<PieceType>,
    castling: bool,
    rook_to: Option<(u8, u8)>,
}

pub struct Board {
    board: Vec<Vec<Option<Piece>>>,
    turn: PieceColor,
    wq_castle: bool,
    wk_castle: bool,
    bq_castle: bool,
    bk_castle: bool,
    en_passant: Option<(u8, u8)>,
    halfmove_clock: u16,
    fullmove_number: u16,
    moves: Vec<(u8, u8, Move)>
}