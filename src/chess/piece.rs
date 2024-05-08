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
    fn value(&self) -> usize {
        match self {
            PieceType::Pawn => 1,
            PieceType::Rook => 5,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Queen => 9,
            PieceType::King => 0,
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

    pub(crate) fn to_fen(&self) -> char {
        match (self.color, self.piece_type) {
            (PieceColor::White, PieceType::Pawn) => 'P',
            (PieceColor::Black, PieceType::Pawn) => 'p',
            (PieceColor::White, PieceType::Rook) => 'R',
            (PieceColor::Black, PieceType::Rook) => 'r',
            (PieceColor::White, PieceType::Knight) => 'N',
            (PieceColor::Black, PieceType::Knight) => 'n',
            (PieceColor::White, PieceType::Bishop) => 'B',
            (PieceColor::Black, PieceType::Bishop) => 'b',
            (PieceColor::White, PieceType::Queen) => 'Q',
            (PieceColor::Black, PieceType::Queen) => 'q',
            (PieceColor::White, PieceType::King) => 'K',
            (PieceColor::Black, PieceType::King) => 'k',
        }
    }

    pub(crate) fn get_value(&self) -> usize {
        self.piece_type.value()
    }
    pub(crate) fn get_potential_value(&self) -> usize {
        match self.piece_type {
            PieceType::Pawn => PieceType::Queen.value(),
            _ => self.get_value(),
        }
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

    /**
     * Check if a piece which is not of PieceType::King can solve a check by capturing the checking piece or moving between the checking piece and the king
     * If the checking piece is a pawn or a knight, the only way to solve the check is to capture it.
     * Otherwise the piece can solve the check if it can move to the same row, column or diagonal as the king and the checking piece is between them.
     * To verify this we check that the vector from the king to the checking piece and the vector from the king to the move_to position are parallel and have the same direction.
     */
    fn solves_check(&self, board: &Board, move_to: (u8, u8), king_position: (u8, u8), checking_piece_position: &(u8, u8)) -> bool {
        if let Some(checking_piece) = board.board[checking_piece_position.0 as usize][checking_piece_position.1 as usize] {
            let pawn_knight_mask = PieceType::Pawn.id() | PieceType::Knight.id() | board.turn.opposite().id();
            if checking_piece.matches(pawn_knight_mask) {
                return move_to == *checking_piece_position;
            }
            let check_offset = (king_position.0 as i32 - checking_piece_position.0 as i32, king_position.1 as i32 - checking_piece_position.1 as i32);
            let defense_offset = (king_position.0 as i32 - move_to.0 as i32, king_position.1 as i32 - move_to.1 as i32);
            
            let check_dir = (if check_offset.0 == 0 {check_offset.0} else {check_offset.0 / check_offset.0.abs()},
                             if check_offset.1 == 0 {check_offset.1} else {check_offset.1 / check_offset.1.abs()});
            let defense_dir = (if defense_offset.0 == 0 {defense_offset.0} else {defense_offset.0 / defense_offset.0.abs()},
                               if defense_offset.1 == 0 {defense_offset.1} else {defense_offset.1 / defense_offset.1.abs()});

            if check_offset.0 * defense_offset.1 == check_offset.1 * defense_offset.0 && check_dir == defense_dir {
                let check_dist = check_offset.0.abs().max(check_offset.1.abs());
                let defense_dist = defense_offset.0.abs().max(defense_offset.1.abs());
                return defense_dist <= check_dist;
            }
            false            
        } else {
            true
        }
    }

    pub(crate) fn generate_moves(&self, board: &Board, checking_pieces: &Vec<(u8, u8)>) -> Vec<Move> {
        let mut result = Vec::new();
        let king_position = board.king_coords(&self.color);
        if checking_pieces.len() > 1 && self.piece_type != PieceType::King {
            return result;
        }

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
        
        match self.piece_type {
            PieceType::Pawn => {
                if !(row_pinned || diag_pinned) {
                    let (delta, start_row, promotion_row): (i32, u8, usize) = match self.color {
                        PieceColor::White => (-1, 6u8, 0),
                        PieceColor::Black => (1, 1u8, 7),
                    };
                    let (mut r, c) = ((self.row as i32 + delta) as usize, self.col as usize);
                    if board.board[r][c].is_none() && (!board.is_check || self.solves_check(board, (r as u8, c as u8), king_position, &checking_pieces[0])) {
                        if r == promotion_row {
                            result.push(Move::new((r as u8, c as u8), None, Some(PieceType::Queen)));
                            result.push(Move::new((r as u8, c as u8), None, Some(PieceType::Rook)));
                            result.push(Move::new((r as u8, c as u8), None, Some(PieceType::Bishop)));
                            result.push(Move::new((r as u8, c as u8), None, Some(PieceType::Knight)));
                        } else  {
                            result.push(Move::new((r as u8, c as u8), None, None));
                        }
                    }
                    if self.row == start_row {
                        r = (r as i32 + delta) as usize;
                        if board.board[r][c].is_none() && (!board.is_check || self.solves_check(board, (r as u8, c as u8), king_position, &checking_pieces[0])) {
                            result.push(Move::new((r as u8, c as u8), None, None));
                        }
                    }
                    if !col_pinned {
                        let captures: Vec<i32> = match (self.color, back_diag_pinned, forward_diag_pinned) {
                            (PieceColor::White, true, false) => vec![-1],
                            (PieceColor::White, false, true) => vec![1],
                            (PieceColor::Black, true, false) => vec![1],
                            (PieceColor::Black, false, true) => vec![-1],
                            (_, false, false) => vec![-1, 1],
                            _ => panic!("Invalid pawn diagonal pin"),
                        };
                        for dc in captures {
                            let r = (self.row as i32 + delta) as usize;
                            let c = (self.col as i32 + dc) as usize;
                            if r < Board::ROWS && c < Board::COLS && (!board.is_check || self.solves_check(board, (r as u8, c as u8), king_position, &checking_pieces[0])) {
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
                }
            },
            PieceType::Knight => {
                if !(row_pinned || col_pinned || diag_pinned) {
                    let jumps = vec![(1, 2), (2, 1), (-1, 2), (-2, 1), (1, -2), (2, -1), (-1, -2), (-2, -1)];
                    for (dr, dc) in jumps {
                        let r = (self.row as i8 + dr) as usize;
                        let c = (self.col as i8 + dc) as usize;
                        if r < 8 && c < 8 && (!board.is_check || self.solves_check(board, (r as u8, c as u8), king_position, &checking_pieces[0])) {
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
                            if !board.is_check || self.solves_check(board, (r as u8, c as u8), king_position, &checking_pieces[0]) {
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
                            if !board.is_check || self.solves_check(board, (r as u8, c as u8), king_position, &checking_pieces[0]) {
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
                            if !board.is_check || self.solves_check(board, (r as u8, c as u8), king_position, &checking_pieces[0]) {
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
                            if !board.is_check || self.solves_check(board, (r as u8, c as u8), king_position, &checking_pieces[0]) {
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
                    if r < 8 && c < 8 && board.get_attacking_pieces(r, c, &self.color, true).is_empty() {
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
                        if board.wk_castle 
                           && board.get_attacking_pieces(7, 4, &self.color, true).is_empty()
                           && board.board[7][5].is_none()
                           && board.board[7][6].is_none()
                           && board.get_attacking_pieces(7, 5, &self.color, true).is_empty()
                           && board.get_attacking_pieces(7, 6, &self.color, true).is_empty() {
                            result.push(Move::castle((7, 6), (7, 5)));
                        }
                        if board.wq_castle
                           && board.get_attacking_pieces(7, 4, &self.color, true).is_empty()
                           && board.board[7][1].is_none()
                           && board.board[7][2].is_none()
                           && board.board[7][3].is_none()
                           && board.get_attacking_pieces(7, 2, &self.color, true).is_empty()
                           && board.get_attacking_pieces(7, 3, &self.color, true).is_empty() {
                            result.push(Move::castle((7, 2), (7, 3)));
                        }
                    },
                    PieceColor::Black => {
                        if board.bk_castle
                           && board.get_attacking_pieces(0, 4, &self.color, true).is_empty()
                           && board.board[0][5].is_none()
                           && board.board[0][6].is_none()
                           && board.get_attacking_pieces(0, 5, &self.color, true).is_empty()
                           && board.get_attacking_pieces(0, 6, &self.color, true).is_empty() {
                            result.push(Move::castle((0, 6), (0, 5)));
                        }
                        if board.bq_castle
                           && board.get_attacking_pieces(0, 4, &self.color, true).is_empty()
                           && board.board[0][1].is_none()
                           && board.board[0][2].is_none()
                           && board.board[0][3].is_none()
                           && board.get_attacking_pieces(0, 2, &self.color, true).is_empty()
                           && board.get_attacking_pieces(0, 3, &self.color, true).is_empty() {
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