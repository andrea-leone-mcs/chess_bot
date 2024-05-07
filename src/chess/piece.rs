use crate::chess::{PieceColor, PieceType, Piece, Move, Board};

impl Move {
    pub(crate) fn new(to: (u8, u8), capture: Option<Piece>, promotion: Option<PieceType>) -> Self {
        Self {
            to,
            capture,
            promotion,
            castling: false,
            rook_to: None,
        }
    }
    pub(crate) fn castle(king_to: (u8, u8), rook_to: (u8, u8)) -> Self {
        Self {
            to: king_to,
            capture: None,
            promotion: None,
            castling: true,
            rook_to: Some(rook_to),
        }
    
    }
}

impl PieceColor {
    fn img_index(&self) -> i32 {
        match self {
            PieceColor::White => 0,
            PieceColor::Black => 1,
        }
    }
    
    pub(crate) fn id(&self) -> u8 {
        match self {
            PieceColor::White => 64,
            PieceColor::Black => 128,
        }
    }
    
    pub(crate) fn opposite(&self) -> Self {
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
    pub fn id(&self) -> u8 {
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

impl Piece {
    pub(crate) fn new(piece_type: PieceType, color: PieceColor, row: u8, col: u8) -> Self {
        Self {
            piece_type,
            color,
            row,
            col,
        }
    }

    pub(crate) fn from_fen(id: char, row: u8, col: u8) -> Option<Self> {
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

    pub(crate) fn get_png(&self) -> String {
        format!("{}{}.png", self.color.img_index(), self.piece_type.img_index())
    }

    pub(crate) fn id(&self) -> u8 {
        self.piece_type.id() | self.color.id()
    }

    pub(crate) fn move_piece(&mut self, _move: &Move) {
        if _move.castling && self.piece_type == PieceType::Rook {
            self.row = _move.rook_to.unwrap().0;
            self.col = _move.rook_to.unwrap().1;
        } else {
            self.row = _move.to.0;
            self.col = _move.to.1;
        }
    }

    pub(crate) fn generate_moves(&self, board: &Board) -> Vec<Move> {
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
        //println!("row_pinned: {}, col_pinned: {}, diag_pinned: {}", row_pinned, col_pinned, diag_pinned);
        assert!(self.color == board.turn, "generate_moves: piece color does not match turn");
        match self.piece_type {
            PieceType::Pawn => {
                if !(row_pinned || diag_pinned) {
                    let (delta, start_row, promotion_row): (i32, u8, usize) = match self.color {
                        PieceColor::White => (-1, 6u8, 0),
                        PieceColor::Black => (1, 1u8, 7),
                    };
                    let (mut r, c) = ((self.row as i32 + delta) as usize, self.col as usize);
                    if board.board[r][c].is_none() {
                        if r == promotion_row {
                            result.push(Move::new((r as u8, c as u8), None, Some(PieceType::Queen)));
                            result.push(Move::new((r as u8, c as u8), None, Some(PieceType::Rook)));
                            result.push(Move::new((r as u8, c as u8), None, Some(PieceType::Bishop)));
                            result.push(Move::new((r as u8, c as u8), None, Some(PieceType::Knight)));
                        } else {
                            result.push(Move::new((r as u8, c as u8), None, None));
                        }
                        if self.row == start_row {
                            r = (r as i32 + delta) as usize;
                            if board.board[r][c].is_none() {
                                result.push(Move::new((r as u8, c as u8), None, None));
                            }
                        }
                    }
                    // else {
                    //     println!("There is a {:?} {:?} in {:?} {:?}", board.board[r][c].as_ref().unwrap().color, board.board[r][c].as_ref().unwrap().piece_type, r, c);
                    // }
                    // compute captures
                    if !col_pinned {
                        let captures: Vec<i32> = match self.color {
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
                            if r >= Board::ROWS || c >= Board::COLS {
                                continue;
                            }
                            if let Some(piece) = &board.board[r][c] {
                                if piece.color != self.color {
                                    if r == promotion_row {
                                        result.push(Move::new((r as u8, c as u8), board.board[r][c], Some(PieceType::Queen)));
                                        result.push(Move::new((r as u8, c as u8), board.board[r][c], Some(PieceType::Rook)));
                                        result.push(Move::new((r as u8, c as u8), board.board[r][c], Some(PieceType::Bishop)));
                                        result.push(Move::new((r as u8, c as u8), board.board[r][c], Some(PieceType::Knight)));
                                    } else {
                                        result.push(Move::new((r as u8, c as u8), board.board[r][c], None));
                                    }
                                }
                            } else if (r as u8, c as u8) == board.en_passant.unwrap_or((9, 9)) {
                                result.push(Move::new((r as u8, c as u8), board.board[r][c], None));
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
                                    result.push(Move::new((r as u8, c as u8), board.board[r][c], None));
                                }
                            } else {
                                result.push(Move::new((r as u8, c as u8), board.board[r][c], None));
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
                        while (0..Board::ROWS as i8).contains(&r) && (0..Board::COLS as i8).contains(&c) {
                            if let Some(occupying) = &board.board[r as usize][c as usize] {
                                if occupying.color == self.color {
                                    break;
                                } else {
                                    result.push(Move::new((r as u8, c as u8), board.board[r as usize][c as usize], None));
                                    break;
                                }
                            } else {
                                result.push(Move::new((r as u8, c as u8), None, None));
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
                        while (0..Board::ROWS as i8).contains(&r) && (0..Board::COLS as i8).contains(&c) {
                            if let Some(occupying) = &board.board[r as usize][c as usize] {
                                if occupying.color == self.color {
                                    break;
                                } else {
                                    result.push(Move::new((r as u8, c as u8), board.board[r as usize][c as usize], None));
                                    break;
                                }
                            } else {
                                result.push(Move::new((r as u8, c as u8), None, None));
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
                        while (0..Board::ROWS as i8).contains(&r) && (0..Board::COLS as i8).contains(&c) {
                            if let Some(occupying) = &board.board[r as usize][c as usize] {
                                if occupying.color == self.color {
                                    break;
                                } else {
                                    result.push(Move::new((r as u8, c as u8), board.board[r as usize][c as usize], None));
                                    break;
                                }
                            } else {
                                result.push(Move::new((r as u8, c as u8), None, None));
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
                        while (0..Board::ROWS as i8).contains(&r) && (0..Board::COLS as i8).contains(&c) {
                            if let Some(occupying) = &board.board[r as usize][c as usize] {
                                if occupying.color == self.color {
                                    break;
                                } else {
                                    result.push(Move::new((r as u8, c as u8), board.board[r as usize][c as usize], None));
                                    break;
                                }
                            } else {
                                result.push(Move::new((r as u8, c as u8), None, None));
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
                    if r < 8 && c < 8 && !board.is_attacked(r, c, &self.color) {
                        if let Some(occupying) = &board.board[r][c] {
                            if occupying.color != self.color {
                                result.push(Move::new((r as u8, c as u8), board.board[r][c], None));
                            }
                        } else {
                            result.push(Move::new((r as u8, c as u8), None, None));
                        }
                    }
                }
                // println!("{:?} {:?} {:?} {:?}", board.wk_castle, board.wq_castle, board.bk_castle, board.bq_castle);
                // castling
                match self.color {
                    PieceColor::White => {
                        if board.wk_castle && !board.is_attacked(7, 4, &self.color) && board.board[7][5].is_none() && board.board[7][6].is_none() && !board.is_attacked(7, 5, &self.color) && !board.is_attacked(7, 6, &self.color) {
                            result.push(Move::castle((7, 6), (7, 5)));
                        }
                        if board.wq_castle && !board.is_attacked(7, 4, &self.color) && board.board[7][1].is_none() && board.board[7][2].is_none() && board.board[7][3].is_none() && !board.is_attacked(7, 2, &self.color) && !board.is_attacked(7, 3, &self.color) {
                            result.push(Move::castle((7, 2), (7, 3)));
                        }
                    },
                    PieceColor::Black => {
                        if board.bk_castle && !board.is_attacked(0, 4, &self.color) && board.board[0][5].is_none() && board.board[0][6].is_none() && !board.is_attacked(0, 5, &self.color) && !board.is_attacked(0, 6, &self.color) {
                            result.push(Move::castle((0, 6), (0, 5)));
                        }
                        if board.bq_castle && !board.is_attacked(0, 4, &self.color) && board.board[0][1].is_none() && board.board[0][2].is_none() && board.board[0][3].is_none() && !board.is_attacked(0, 2, &self.color) && !board.is_attacked(0, 3, &self.color) {
                            result.push(Move::castle((0, 2), (0, 3)));
                        }
                    },
                };
            }
        }
        result
    }

    pub(crate) fn matches(&self, mask: u8) -> bool {
        let my_id = self.id();
        my_id & mask == my_id
    }
}