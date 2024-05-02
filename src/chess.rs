use gtk::glib::object::Cast;
use gtk::{gdk_pixbuf, Button, Grid, Picture};
use gtk::prelude::{GridExt, ButtonExt};

#[derive(Debug, Clone, PartialEq, Eq)]
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PieceColor {
    White,
    Black,
}
struct Move {
    to: (u8, u8),
    capture: bool,
    promotion: Option<PieceType>,
}

impl Move {
    pub fn new(to: (u8, u8), capture: bool, promotion: Option<PieceType>) -> Self {
        Self {
            to,
            capture,
            promotion,
        }
    }
    pub fn to(to: (u8, u8)) -> Self {
        Self::new(to, false, None)
    }
}

impl PieceColor {
    fn img_index(&self) -> i32 {
        match self {
            PieceColor::White => 0,
            PieceColor::Black => 1,
        }
    }
    fn id(&self) -> u8 {
        match self {
            PieceColor::White => 64,
            PieceColor::Black => 128,
        }
    }
    fn opposite(&self) -> Self {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}

impl PieceType {
    fn value(&self) -> i32 {
        match self {
            PieceType::Pawn => 1,
            PieceType::Rook => 5,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Queen => 9,
            PieceType::King => i32::MAX,
        }
    }
    fn id(&self) -> u8 {
        match self {
            PieceType::Pawn => 1,
            PieceType::Rook => 2,
            PieceType::Knight => 4,
            PieceType::Bishop => 8,
            PieceType::Queen => 16,
            PieceType::King => 32,
        }
    }
    fn img_index(&self) -> i32 {
        match self {
            PieceType::Pawn => 5,
            PieceType::Rook => 2,
            PieceType::Knight => 3,
            PieceType::Bishop => 4,
            PieceType::Queen => 0,
            PieceType::King => 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Piece {
    piece_type: PieceType,
    color: PieceColor,
    row: u8,
    col: u8,
}

impl Piece {
    fn new(piece_type: PieceType, color: PieceColor, row: u8, col: u8) -> Self {
        Self {
            piece_type,
            color,
            row,
            col,
        }
    }

    fn from_fen(id: char, row: u8, col: u8) -> Option<Self> {
        match id {
            'P' => Some(Self::new(PieceType::Pawn, PieceColor::White, row, col)),
            'p' => Some(Self::new(PieceType::Pawn, PieceColor::Black, row, col)),
            'R' => Some(Self::new(PieceType::Rook, PieceColor::White, row, col)),
            'r' => Some(Self::new(PieceType::Rook, PieceColor::Black, row, col)),
            'N' => Some(Self::new(PieceType::Knight, PieceColor::White, row, col)),
            'n' => Some(Self::new(PieceType::Knight, PieceColor::Black, row, col)),
            'B' => Some(Self::new(PieceType::Bishop, PieceColor::White, row, col)),
            'b' => Some(Self::new(PieceType::Bishop, PieceColor::Black, row, col)),
            'Q' => Some(Self::new(PieceType::Queen, PieceColor::White, row, col)),
            'q' => Some(Self::new(PieceType::Queen, PieceColor::Black, row, col)),
            'K' => Some(Self::new(PieceType::King, PieceColor::White, row, col)),
            'k' => Some(Self::new(PieceType::King, PieceColor::Black, row, col)),
            _ => None,
        }
    }

    fn get_value(&self) -> i32 {
        self.piece_type.value()
    }

    pub fn get_png(&self) -> String {
        format!("{}{}.png", self.color.img_index(), self.piece_type.img_index())
    }

    pub fn id(&self) -> u8 {
        self.piece_type.id() | self.color.id()
    }

    fn move_piece(&mut self, _move: Move) {
        self.row = _move.to.0;
        self.col = _move.to.1;
    }

    pub fn generate_moves(&self, board: &Board) -> Vec<Move> {
        let mut result = Vec::new();
        let row_pinned = board.row_pin(self.row as usize, self.col as usize, 
            PieceType::King.id() | self.color.id(),
            PieceType::Queen.id() | PieceType::Rook.id() | self.color.opposite().id());
        let col_pinned = board.col_pin(self.row as usize, self.col as usize,
            PieceType::King.id() | self.color.id(),
            PieceType::Queen.id() | PieceType::Rook.id() | self.color.opposite().id());
        let back_diag_pinned = board.back_diag_pin(self.row as usize, self.col as usize,
            PieceType::King.id() | self.color.id(),
            PieceType::Queen.id() | PieceType::Bishop.id() | self.color.opposite().id());
        let forward_diag_pinned = board.forward_diag_pin(self.row as usize, self.col as usize,
            PieceType::King.id() | self.color.id(),
            PieceType::Queen.id() | PieceType::Bishop.id() | self.color.opposite().id());
        let diag_pinned = back_diag_pinned || forward_diag_pinned;
        match self.piece_type {
            PieceType::Pawn => {
                if !(row_pinned || diag_pinned) {
                    let (delta, start_row, promotion_row): (i32, usize, usize) = match self.color {
                        PieceColor::White => (-1, 6, 0),
                        PieceColor::Black => (1, 1, 7),
                    };
                    let (mut r, mut c) = ((self.row as i32 + delta) as usize, self.col as usize);
                    if board.board[r][c].is_none() {
                        if r == promotion_row {
                            result.push(Move::new((r as u8, c as u8), false, Some(PieceType::Queen)));
                            result.push(Move::new((r as u8, c as u8), false, Some(PieceType::Rook)));
                            result.push(Move::new((r as u8, c as u8), false, Some(PieceType::Bishop)));
                            result.push(Move::new((r as u8, c as u8), false, Some(PieceType::Knight)));
                        } else {
                            result.push(Move::new((r as u8, c as u8), false, None));
                        }
                    }
                    if self.row as usize == start_row {
                        r = (self.row as i32 + 2*delta) as usize;
                        c = self.col as usize;
                        if board.board[r][c].is_none() {
                            result.push(Move::new((r as u8, c as u8), false, None));
                        }
                    }
                    // compute captures
                    if !col_pinned {
                        let captures = match self.color {
                            PieceColor::White => {
                                if back_diag_pinned {
                                    vec![-1]
                                } else if forward_diag_pinned {
                                    vec![1]
                                } else {
                                    vec![-1, 1]
                                }
                            },
                            PieceColor::Black => {
                                if back_diag_pinned {
                                    vec![1]
                                } else if forward_diag_pinned {
                                    vec![-1]
                                } else {
                                    vec![-1, 1]
                                }
                            },
                        };
                        for dc in captures {
                            let r = (self.row as i32 + delta) as usize;
                            let c = (self.col as i32 + dc) as usize;
                            if let Some(piece) = &board.board[r][c] {
                                if piece.color != self.color {
                                    if r == promotion_row {
                                        result.push(Move::new((r as u8, c as u8), true, Some(PieceType::Queen)));
                                        result.push(Move::new((r as u8, c as u8), true, Some(PieceType::Rook)));
                                        result.push(Move::new((r as u8, c as u8), true, Some(PieceType::Bishop)));
                                        result.push(Move::new((r as u8, c as u8), true, Some(PieceType::Knight)));
                                    } else {
                                        result.push(Move::new((r as u8, c as u8), true, None));
                                    }
                                }
                            } else if (r as u8, c as u8) == board.en_passant.unwrap_or((9, 9)) {
                                result.push(Move::new((r as u8, c as u8), true, None));
                            }
                        }
                    }
                }
            },
            PieceType::Knight => {
                if !(row_pinned || col_pinned || diag_pinned) {
                    let jumps = vec![(1, 2), (2, 1), (-1, 2), (-2, 1), (1, -2), (2, -1), (-1, -2), (-2, -1)];
                    for (dr, dc) in jumps {
                        let r = (self.row as i8 + dr) as usize;
                        let c = (self.col as i8 + dc) as usize;
                        if r < 8 && c < 8 {
                            if let Some(occupying) = &board.board[r][c] {
                                if occupying.color != self.color {
                                    result.push(Move::new((r as u8, c as u8), true, None));
                                }
                            } else {
                                result.push(Move::new((r as u8, c as u8), false, None));
                            }
                        }
                    }
                }
            },
            PieceType::Bishop => {
                if !(row_pinned || col_pinned) {
                    let mut directions: Vec<(i8, i8)> = vec![];
                    if !forward_diag_pinned {
                        directions.push((1, 1));
                        directions.push((-1, -1));
                    }
                    if !back_diag_pinned {
                        directions.push((1, -1));
                        directions.push((-1, 1));
                    }
                    for (dr, dc) in directions {
                        let (mut r, mut c) = (self.row as i8 + dr, self.col as i8 + dc);
                        while r >= 0 && r < 8 && c >= 0 && c < 8 {
                            if let Some(occupying) = &board.board[r as usize][c as usize] {
                                if occupying.color == self.color {
                                    break;
                                } else {
                                    result.push(Move::new((r as u8, c as u8), true, None));
                                    break;
                                }
                            } else {
                                result.push(Move::new((r as u8, c as u8), false, None));
                            }
                            r += dr;
                            c += dc;
                        }
                    }
                }
            },
            PieceType::Rook => {
                if !(diag_pinned) {
                    let mut directions: Vec<(i8, i8)> = vec![];
                    if !row_pinned {
                        directions.push((1, 0));
                        directions.push((-1, 0));
                    }
                    if !col_pinned {
                        directions.push((0, 1));
                        directions.push((0, -1));
                    }
                    for (dr, dc) in directions {
                        let (mut r, mut c) = (self.row as i8 + dr, self.col as i8 + dc);
                        while r >= 0 && r < 8 && c >= 0 && c < 8 {
                            if let Some(occupying) = &board.board[r as usize][c as usize] {
                                if occupying.color == self.color {
                                    break;
                                } else {
                                    result.push(Move::new((r as u8, c as u8), true, None));
                                    break;
                                }
                            } else {
                                result.push(Move::new((r as u8, c as u8), false, None));
                            }
                            r += dr;
                            c += dc;
                        }
                    }
                }
            },
            PieceType::Queen => { // double check
                if !(row_pinned || col_pinned) {
                    let mut directions: Vec<(i8, i8)> = vec![];
                    if !forward_diag_pinned {
                        directions.push((1, 1));
                        directions.push((-1, -1));
                    }
                    if !back_diag_pinned {
                        directions.push((1, -1));
                        directions.push((-1, 1));
                    }
                    for (dr, dc) in directions {
                        let (mut r, mut c) = (self.row as i8 + dr, self.col as i8 + dc);
                        while r >= 0 && r < 8 && c >= 0 && c < 8 {
                            if let Some(occupying) = &board.board[r as usize][c as usize] {
                                if occupying.color == self.color {
                                    break;
                                } else {
                                    result.push(Move::new((r as u8, c as u8), true, None));
                                    break;
                                }
                            } else {
                                result.push(Move::new((r as u8, c as u8), false, None));
                            }
                            r += dr;
                            c += dc;
                        }
                    }
                }
                if !(diag_pinned) {
                    let mut directions: Vec<(i8, i8)> = vec![];
                    if !row_pinned {
                        directions.push((1, 0));
                        directions.push((-1, 0));
                    }
                    if !col_pinned {
                        directions.push((0, 1));
                        directions.push((0, -1));
                    }
                    for (dr, dc) in directions {
                        let (mut r, mut c) = (self.row as i8 + dr, self.col as i8 + dc);
                        while r >= 0 && r < 8 && c >= 0 && c < 8 {
                            if let Some(occupying) = &board.board[r as usize][c as usize] {
                                if occupying.color == self.color {
                                    break;
                                } else {
                                    result.push(Move::new((r as u8, c as u8), true, None));
                                    break;
                                }
                            } else {
                                result.push(Move::new((r as u8, c as u8), false, None));
                            }
                            r += dr;
                            c += dc;
                        }
                    }
                }
            },
            PieceType::King => {
                let directions = [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];
                for (dr, dc) in directions.iter() {
                    let r = (self.row as i8 + dr) as usize;
                    let c = (self.col as i8 + dc) as usize;
                    if r < 8 && c < 8 && !board.is_attacked(r, c, self.color) {
                        if let Some(occupying) = &board.board[r][c] {
                            if occupying.color != self.color {
                                result.push(Move::new((r as u8, c as u8), true, None));
                            }
                        } else {
                            result.push(Move::new((r as u8, c as u8), false, None));
                        }
                    }
                }
            }
            _ => {}
        }
        result
    }

    fn matches(&self, mask: u8) -> bool {
        let my_id = self.id();
        my_id & mask == my_id
    }
}

pub struct Board {
    board: Vec<Vec<Option<Piece>>>,
    turn: PieceColor,
    wq_castle: bool,
    wk_castle: bool,
    bq_castle: bool,
    bk_castle: bool,
    en_passant: Option<(u8, u8)>,
    halfmove_clock: u8,
    fullmove_number: u8,
}

impl Board {
    const ROWS: usize = 8;
    const COLS: usize = 8;

    pub fn new() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    pub fn from_fen(fen: &str) -> Result<Self, &str> {
        let mut board: Vec<Vec<Option<Piece>>> = vec![vec![None; Board::COLS]; Board::ROWS];
        let (mut row, mut col) : (usize, usize) = (0, 0);
        let parts: Vec<&str> = fen.split(' ').collect();
        // fill the board
        for c in parts[0].chars() {
            match c {
                '1'..='8' => {
                    let increment = match c.to_digit(10) {
                        Some(d) => d,
                        None => return Err("from_fen: error getting digit value.")
                    } as usize;
                    col += increment;
                },
                '/' => {
                    col = 0;
                    row += 1;
                },
                _ => {
                    let piece = match Piece::from_fen(c, row as u8, col as u8) {
                        Some(piece) => piece,
                        None => return Err("from_fen: error getting piece.")
                    };
                    board[row][col] = Some(piece);
                    col += 1;                    
                }
            }
        }
        // pick turn
        let turn = match parts[1] {
            "w" => PieceColor::White,
            "b" => PieceColor::Black,
            _  => return Err("from_fen: error getting turn")
        };

        // pick castling rights
        let (wq_castle, wk_castle, bq_castle, bk_castle) = (
            parts[2].contains("Q"),
            parts[2].contains("K"),
            parts[2].contains("q"),
            parts[2].contains("k")
        );
        
        let en_passant = match parts[3] {
            "-" => None,
            _ => match Board::coords_to_u8(parts[3]) {
                Ok(pair) => Some(pair),
                Err(_) => return Err("from_fen: error getting en passant move")
            }
        };

        let halfmove_clock: u8 = match parts[4].parse::<u8>() {
            Ok(v) => v,
            Err(_) => return Err("from_fen: error getting halfmove clock")
        };

        let fullmove_number: u8 = match parts[5].parse::<u8>() {
            Ok(v) => v,
            Err(_) => return Err("from_fen: error getting fullmove number")
        };

        Ok(Self {
            board,
            turn,
            wq_castle,
            wk_castle,
            bq_castle,
            bk_castle,
            en_passant,
            halfmove_clock,
            fullmove_number,
        })
    }

    fn is_attacked(&self, row: usize, col: usize, color: PieceColor) -> bool {
        let knight_moves = vec![(1, 2), (2, 1), (-1, 2), (-2, 1), (1, -2), (2, -1), (-1, -2), (-2, -1)];
        for (dr, dc) in knight_moves {
            let r = (row as i8 + dr) as usize;
            let c = (col as i8 + dc) as usize;
            if r < 8 && c < 8 {
                if let Some(piece) = &self.board[r][c] {
                    if piece.piece_type == PieceType::Knight && piece.color != color {
                        return true;
                    }
                }
            }
        }
        let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        for (dr, dc) in directions.iter() {
            let (mut r, mut c) = (row as i8 + dr, col as i8 + dc);
            while r >= 0 && r < 8 && c >= 0 && c < 8 {
                if let Some(piece) = &self.board[r as usize][c as usize] {
                    if piece.color == color && piece.piece_type != PieceType::King {
                        break;
                    } else {
                        if piece.piece_type == PieceType::Queen || piece.piece_type == PieceType::Rook || piece.piece_type == PieceType::King {
                            return true;
                        } else {
                            break;
                        }
                    }
                }
                r += dr;
                c += dc;
            }
        }
        let directions = [(1, 1), (-1, -1), (1, -1), (-1, 1)];
        for (dr, dc) in directions.iter() {
            let (mut r, mut c) = (row as i8 + dr, col as i8 + dc);
            while r >= 0 && r < 8 && c >= 0 && c < 8 {
                if let Some(piece) = &self.board[r as usize][c as usize] {
                    if piece.color == color && piece.piece_type != PieceType::King {
                        break;
                    } else {
                        if piece.piece_type == PieceType::Queen || piece.piece_type == PieceType::Bishop || piece.piece_type == PieceType::King {
                            return true;
                        } else {
                            break;
                        }
                    }
                }
                r += dr;
                c += dc;
            }
        }
        todo!()
    }

    fn row_pin(&self, row: usize, col: usize, mask1: u8, mask2: u8) -> bool {
        let (mut found1, mut found2) = (false, false);
        let mut c = col-1;
        while c >= 0 {
            if let Some(piece) = &self.board[row][c] {
                found1 |= piece.matches(mask1);
                found2 |= piece.matches(mask2);
                break;
            } else {
                c -= 1;
            }
        }
        c = col+1;
        while c < Board::COLS {
            if let Some(piece) = &self.board[row][c] {
                found1 |= piece.matches(mask1);
                found2 |= piece.matches(mask2);
                break;
            } else {
                c += 1;
            }
        }
        found1 && found2
    }

    fn col_pin(&self, row: usize, col: usize, mask1: u8, mask2: u8) -> bool {
        let (mut found1, mut found2) = (false, false);
        let mut r = row-1;
        while r >= 0 {
            if let Some(piece) = &self.board[r][col] {
                found1 |= piece.matches(mask1);
                found2 |= piece.matches(mask2);
                break;
            } else {
                r -= 1;
            }
        }
        r = row+1;
        while r < Board::ROWS {
            if let Some(piece) = &self.board[r][col] {
                found1 |= piece.matches(mask1);
                found2 |= piece.matches(mask2);
                break;
            } else {
                r += 1;
            }
        }
        found1 && found2
    }

    fn back_diag_pin(&self, row: usize, col: usize, mask1: u8, mask2: u8) -> bool {
        let (mut found1, mut found2) = (false, false);
        let mut r = row-1;
        let mut c = col-1;
        while r >= 0 && c >= 0 {
            if let Some(piece) = &self.board[r][c] {
                found1 |= piece.matches(mask1);
                found2 |= piece.matches(mask2);
                break;
            } else {
                r -= 1;
                c -= 1;
            }
        }
        r = row+1;
        c = col+1;
        while r < Board::ROWS && c < Board::COLS {
            if let Some(piece) = &self.board[r][c] {
                found1 |= piece.matches(mask1);
                found2 |= piece.matches(mask2);
                break;
            } else {
                r += 1;
                c += 1;
            }
        }
        found1 && found2
    }

    fn forward_diag_pin(&self, row: usize, col: usize, mask1: u8, mask2: u8) -> bool {
        let (mut found1, mut found2) = (false, false);
        let mut r = row-1;
        let mut c = col+1;
        while r >= 0 && c < Board::COLS {
            if let Some(piece) = &self.board[r][c] {
                found1 |= piece.matches(mask1);
                found2 |= piece.matches(mask2);
                break;
            } else {
                r -= 1;
                c += 1;
            }
        }
        r = row+1;
        c = col-1;
        while r < Board::ROWS && c >= 0 {
            if let Some(piece) = &self.board[r][c] {
                found1 |= piece.matches(mask1);
                found2 |= piece.matches(mask2);
                break;
            } else {
                r += 1;
                c -= 1;
            }
        }
        found1 && found2
    }

    fn same_row(&self, row: usize, mask1: u8, mask2: u8) -> bool {
        let (mut found1, mut found2) = (false, false);
        for col in 0..Board::COLS {
            if let Some(piece) = &self.board[row][col] {
                found1 |= piece.matches(mask1);
                found2 |= piece.matches(mask2);
            }
        }
        found1 && found2
    }

    fn same_col(&self, col: usize, mask1: u8, mask2: u8) -> bool {
        let (mut found1, mut found2) = (false, false);
        for row in 0..Board::ROWS {
            if let Some(piece) = &self.board[row][col] {
                found1 |= piece.matches(mask1);
                found2 |= piece.matches(mask2);
            }
        }
        found1 && found2
    }

    fn same_diagonal(&self, row: usize, col: usize, mask1: u8, mask2: u8) -> bool {
        let (mut found1, mut found2) = (false, false);
        let mut pos = (row * 8 + col) % 9;
        while pos < Board::ROWS*Board::COLS {
            let r = (pos / 8) as usize;
            let c = (pos % 8) as usize;
            if r != row && c != col {
                if let Some(piece) = &self.board[r][c] {
                    found1 |= piece.matches(mask1);
                    found2 |= piece.matches(mask2);
                }
            }
            pos += 9;
        }
        if found1 && found2 {
            return true;
        }
        found1 = false;
        found2 = false;
        pos = row * 8 + col;
        while pos/8 != (pos-7)/8 {
            pos -= 7;
        }
        while pos < Board::ROWS*Board::COLS && pos/8 != (pos+7)/8 {
            let r = (pos / 8) as usize;
            let c = (pos % 8) as usize;
            if r != row && c != col {
                if let Some(piece) = &self.board[r][c] {
                    found1 |= piece.matches(mask1);
                    found2 |= piece.matches(mask2);
                }
            }
            pos += 7;
        }
        found1 && found2
    }

    pub fn apply_to_grid(&self, grid: &Grid) {
        for row in 0..8 {
            for col in 0..8 {
                let tmp = grid.child_at(col, row);
                if let Some(widget) = tmp {
                    let button = widget.downcast_ref::<Button>().unwrap();
                    // button.set_child(self.board[row][col])
                    let child = match &self.board[row as usize][col as usize] {
                        None => None,
                        Some(piece) => {
                            let image_path = format!("images/{}", piece.get_png());
                            let pixbuf = gdk_pixbuf::Pixbuf::from_file(image_path)
                                .expect("Failed to load image");
                            // Create picture from the sub-image
                            let picture = Picture::new();
                            picture.set_pixbuf(Some(&pixbuf));
                            Some(picture)
                        }
                    };
                    button.set_child(child.as_ref());
                }
            }
        }
    }

    fn coords_to_u8(coords: &str) -> Result<(u8, u8), &str> {
        if coords.len() != 2 {
            return Err("coords_to_u8: invalid length");
        }
        let col = match coords.chars().nth(0).unwrap() {
            'a'..='h' => coords.chars().nth(0).unwrap() as u8 - 'a' as u8,
            _ => return Err("coords_to_u8: invalid column")
        };
        let row = match coords.chars().nth(1).unwrap() {
            '1'..='8' => coords.chars().nth(1).unwrap() as u8 - '1' as u8,
            _ => return Err("coords_to_u8: invalid row")
        };
        Ok((row, col))
    }
}