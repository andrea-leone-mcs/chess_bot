use gtk::glib::object::Cast;
use gtk::{gdk_pixbuf, Button, Grid, Picture};
use gtk::prelude::{GridExt, ButtonExt};

use crate::chess::{Board, PieceColor, Piece, Move, PieceType};

impl Board {
    pub const ROWS: usize = 8;
    pub const COLS: usize = 8;

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
            parts[2].contains('Q'),
            parts[2].contains('K'),
            parts[2].contains('q'),
            parts[2].contains('k')
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
            moves: Vec::new(),
        })
    }

    pub(crate) fn is_attacked(&self, row: usize, col: usize, color: &PieceColor) -> bool {
        let knight_moves = vec![(1, 2), (2, 1), (-1, 2), (-2, 1), (1, -2), (2, -1), (-1, -2), (-2, -1)];
        for (dr, dc) in knight_moves {
            let r = (row as i8 + dr) as usize;
            let c = (col as i8 + dc) as usize;
            if r < 8 && c < 8 {
                if let Some(piece) = &self.board[r][c] {
                    if piece.piece_type == PieceType::Knight && piece.color != *color {
                        return true;
                    }
                }
            }
        }
        let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        for (dr, dc) in directions.iter() {
            let (mut r, mut c) = ((row as i8 + dr) as usize, (col as i8 + dc) as usize);
            let mut row_col_mask = PieceType::Queen.id() | PieceType::Rook.id() | PieceType::King.id() | color.opposite().id();
            while r < Board::ROWS && c < Board::COLS {
                if let Some(piece) = &self.board[r][c] {
                    /*if piece.color == *color {
                        break;
                    } else */if piece.matches(row_col_mask) {
                        return true;
                    } else {
                        break;
                    }
                }
                r = (r as i8 + dr) as usize;
                c = (c as i8 + dc) as usize;
                row_col_mask &= !PieceType::King.id();
            }
        }
        let directions = [(1, 1), (-1, -1), (1, -1), (-1, 1)];
        for (dr, dc) in directions.iter() {
            let (mut r, mut c) = ((row as i8 + dr) as usize, (col as i8 + dc) as usize);
            let mut diag_mask = PieceType::Queen.id() | PieceType::Bishop.id() | PieceType::King.id() | color.opposite().id();
            while r < Board::ROWS && c < Board::COLS {
                if let Some(piece) = &self.board[r][c] {
                    /*if piece.color == *color {
                        break;
                    } else */if piece.matches(diag_mask) {
                        return true;
                    } else {
                        break;
                    }
                }
                r = (r as i8 + dr) as usize;
                c = (c as i8 + dc) as usize;
                diag_mask &= !PieceType::King.id();
            }
        }
        let directions = match color {
            PieceColor::White => [(-1, -1), (-1, 1)],
            PieceColor::Black => [(1, -1), (1, 1)],
        };
        for (dr, dc) in directions.iter() {
            let r = (row as i8 + dr) as usize;
            let c = (col as i8 + dc) as usize;
            if r < 8 && c < 8 {
                if let Some(piece) = &self.board[r][c] {
                    if piece.piece_type == PieceType::Pawn && piece.color != *color {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub(crate)fn row_pin(&self, row: usize, col: usize, mask1: u8, mask2: u8) -> bool {
        let (mut found1, mut found2) = (false, false);
        let mut c = col as i32 - 1;
        while c >= 0 {
            if let Some(piece) = &self.board[row][c as usize] {
                found1 |= piece.matches(mask1);
                found2 |= piece.matches(mask2);
                break;
            } else {
                c -= 1;
            }
        }
        let mut c = col+1;
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

    pub(crate)fn col_pin(&self, row: usize, col: usize, mask1: u8, mask2: u8) -> bool {
        let (mut found1, mut found2) = (false, false);
        let mut r = row as i32 - 1;
        while r >= 0 {
            if let Some(piece) = &self.board[r as usize][col] {
                found1 |= piece.matches(mask1);
                found2 |= piece.matches(mask2);
                break;
            } else {
                r -= 1;
            }
        }
        let mut r = row+1;
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

    pub(crate)fn back_diag_pin(&self, row: usize, col: usize, mask1: u8, mask2: u8) -> bool {
        let (mut found1, mut found2) = (false, false);
        let mut r = row as i32 - 1;
        let mut c = col as i32 - 1;
        while r >= 0 && c >= 0 {
            if let Some(piece) = &self.board[r as usize][c as usize] {
                found1 |= piece.matches(mask1);
                found2 |= piece.matches(mask2);
                break;
            } else {
                r -= 1;
                c -= 1;
            }
        }
        let mut r = row+1;
        let mut c = col+1;
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

    pub(crate) fn forward_diag_pin(&self, row: usize, col: usize, mask1: u8, mask2: u8) -> bool {
        let (mut found1, mut found2) = (false, false);
        let mut r = row as i32 - 1;
        let mut c = col+1;
        while r >= 0 && c < Board::COLS {
            if let Some(piece) = &self.board[r as usize][c] {
                found1 |= piece.matches(mask1);
                found2 |= piece.matches(mask2);
                break;
            } else {
                r -= 1;
                c += 1;
            }
        }
        let mut r = row+1;
        let mut c = col as i32 - 1;
        while r < Board::ROWS && c >= 0 {
            if let Some(piece) = &self.board[r][c as usize] {
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

    // fn same_row(&self, row: usize, mask1: u8, mask2: u8) -> bool {
    //     let (mut found1, mut found2) = (false, false);
    //     for col in 0..Board::COLS {
    //         if let Some(piece) = &self.board[row][col] {
    //             found1 |= piece.matches(mask1);
    //             found2 |= piece.matches(mask2);
    //         }
    //     }
    //     found1 && found2
    // }

    // fn same_col(&self, col: usize, mask1: u8, mask2: u8) -> bool {
    //     let (mut found1, mut found2) = (false, false);
    //     for row in 0..Board::ROWS {
    //         if let Some(piece) = &self.board[row][col] {
    //             found1 |= piece.matches(mask1);
    //             found2 |= piece.matches(mask2);
    //         }
    //     }
    //     found1 && found2
    // }

    // fn same_diagonal(&self, row: usize, col: usize, mask1: u8, mask2: u8) -> bool {
    //     let (mut found1, mut found2) = (false, false);
    //     let mut pos = (row * 8 + col) % 9;
    //     while pos < Board::ROWS*Board::COLS {
    //         let r = (pos / 8) as usize;
    //         let c = (pos % 8) as usize;
    //         if r != row && c != col {
    //             if let Some(piece) = &self.board[r][c] {
    //                 found1 |= piece.matches(mask1);
    //                 found2 |= piece.matches(mask2);
    //             }
    //         }
    //         pos += 9;
    //     }
    //     if found1 && found2 {
    //         return true;
    //     }
    //     found1 = false;
    //     found2 = false;
    //     pos = row * 8 + col;
    //     while pos/8 != (pos-7)/8 {
    //         pos -= 7;
    //     }
    //     while pos < Board::ROWS*Board::COLS && pos/8 != (pos+7)/8 {
    //         let r = (pos / 8) as usize;
    //         let c = (pos % 8) as usize;
    //         if r != row && c != col {
    //             if let Some(piece) = &self.board[r][c] {
    //                 found1 |= piece.matches(mask1);
    //                 found2 |= piece.matches(mask2);
    //             }
    //         }
    //         pos += 7;
    //     }
    //     found1 && found2
    // }

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
            'a'..='h' => coords.chars().nth(0).unwrap() as u8 - b'a',
            _ => return Err("coords_to_u8: invalid column")
        };
        let row = match coords.chars().nth(1).unwrap() {
            '1'..='8' => coords.chars().nth(1).unwrap() as u8 - b'1',
            _ => return Err("coords_to_u8: invalid row")
        };
        Ok((row, col))
    }

    fn king_coords(&self, color: &PieceColor) -> (u8, u8) {
        for row in 0..Board::ROWS {
            for col in 0..Board::COLS {
                if let Some(piece) = &self.board[row][col] {
                    if piece.piece_type == PieceType::King && piece.color == *color {
                        return (row as u8, col as u8);
                    }
                }
            }
        }
        (9, 9)
    }

    fn is_check(&self, color: &PieceColor) -> bool {
        let king_position = self.king_coords(color);
        self.is_attacked(king_position.0 as usize, king_position.1 as usize, color)        
    }

    pub fn play_random_move(&mut self) {
        let mut moves = Vec::new();
        for row in 0..Board::ROWS {
            for col in 0..Board::COLS {
                if let Some(piece) = &self.board[row][col] {
                    if piece.color == self.turn {
                        let piece_moves = piece.generate_moves(self);
                        for mv in piece_moves {
                            if mv.castling {
                                return self.play_move((row, col), &mv);
                            }
                            moves.push((row, col, mv));
                        }
                    }
                }
            }
        }
        println!("{} moves available", moves.len());
        if self.is_check(&self.turn) {
            println!("CHECK");
            while self.is_check(&self.turn) {
                let random_idx = rand::random::<usize>() % moves.len();
                let (row, col, mv) = moves.swap_remove(random_idx);
                
                self.play_move((row, col), &mv);
                if self.is_check(&self.turn.opposite()) {
                    self.rollback_move();
                }
            }
        } else {
            let random_idx = rand::random::<usize>() % moves.len();
            let (row, col, mv) = moves.swap_remove(random_idx);
            
            self.play_move((row, col), &mv);
        }
    }

    fn play_move(&mut self, from: (usize, usize), mv: &Move) {
        self.moves.push((from.0 as u8, from.1 as u8, mv.clone()));

        let mut piece = self.board[from.0][from.1].take().unwrap();
        println!("{:?} {:?} {:?} from {:?} to {:?} capture={:?} promote={:?}", if mv.castling {"Castling"} else {"Moving"}, piece.color, piece.piece_type, Board::u8_coords_to_str((from.0 as u8, from.1 as u8)), Board::u8_coords_to_str(mv.to), mv.capture, mv.promotion);
        
        if mv.castling {
            let mut rook = self.board[piece.row as usize][if mv.rook_to.unwrap().1 == 5 {7} else {0}].take().unwrap();
            rook.move_piece(mv);
            self.board[mv.rook_to.unwrap().0 as usize][mv.rook_to.unwrap().1 as usize] = Some(rook);
        }
        

        if piece.piece_type == PieceType::King {
            match piece.color {
                PieceColor::White => {
                    self.wk_castle = false;
                    self.wq_castle = false;
                },
                PieceColor::Black => {
                    self.bk_castle = false;
                    self.bq_castle = false;
                },
            };
        } else if piece.piece_type == PieceType::Rook {
            match piece.color {
                PieceColor::White => {
                    if piece.col == 0 {
                        self.wq_castle = false;
                    } else if piece.col == 7 {
                        self.wk_castle = false;
                    }
                },
                PieceColor::Black => {
                    if piece.col == 0 {
                        self.bq_castle = false;
                    } else if piece.col == 7 {
                        self.bk_castle = false;
                    }
                },
            };
        }

        piece.move_piece(mv);
        self.board[mv.to.0 as usize][mv.to.1 as usize] = match mv.promotion {
            Some(piece_type) => Some(Piece::new(piece_type, piece.color, mv.to.0, mv.to.1)),
            None => Some(piece),
        };
        
        self.halfmove_clock += 1;
        if self.turn == PieceColor::Black {
            self.fullmove_number += 1;
        }
        self.turn = self.turn.opposite();
    }

    fn rollback_move(&mut self) {
        self.turn = self.turn.opposite();
        let (row, col, mv) = self.moves.pop().unwrap();
        if mv.castling {
            let (r_rook, c_rook) = mv.rook_to.unwrap();
            let mut rook = self.board[r_rook as usize][c_rook as usize].take().unwrap();
            let c_to = if c_rook == 5 {7} else {0};
            rook.move_piece(&Move::new((r_rook, c_to), None, None));
            self.board[r_rook as usize][c_to as usize] = Some(rook);
            if c_to == 7 {
                match self.turn {
                    PieceColor::White => self.wk_castle = true,
                    PieceColor::Black => self.bk_castle = true,
                };
            } else {
                match self.turn {
                    PieceColor::White => self.wq_castle = true,
                    PieceColor::Black => self.bq_castle = true,
                };
            }
        }
        let mut piece = self.board[mv.to.0 as usize][mv.to.1 as usize].take().unwrap();
        if mv.capture.is_some() {
            self.board[mv.to.0 as usize][mv.to.1 as usize] = mv.capture;
        }
        
        if mv.promotion.is_some() {
            piece.piece_type = PieceType::Pawn;
        }
        piece.move_piece(&Move::new((row, col), None, None));
        self.board[row as usize][col as usize] = Some(piece);

        self.halfmove_clock -= 1;
        if self.turn == PieceColor::Black {
            self.fullmove_number -= 1;
        }
    }

    fn u8_coords_to_str(coords: (u8, u8)) -> String {
        format!("{}{}", (b'a' + coords.1) as char, (b'1' + (Board::ROWS as i32-coords.0 as i32-1) as u8) as char)
    }
}