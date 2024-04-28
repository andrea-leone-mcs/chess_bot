use gtk::glib::object::Cast;
use gtk::{gdk_pixbuf, Button, Grid, Picture};
use gtk::prelude::{GridExt, ButtonExt};

#[derive(Debug, Clone)]
enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}
#[derive(Debug, Clone)]
enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    fn img_id(&self) -> i32 {
        match self {
            PieceColor::White => 0,
            PieceColor::Black => 1,
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
    fn img_id(&self) -> i32 {
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

    fn from_id(id: char, row: u8, col: u8) -> Option<Self> {
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
        format!("{}{}.png", self.color.img_id(), self.piece_type.img_id())
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
    pub fn new() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    pub fn from_fen(fen: &str) -> Result<Self, &str> {
        let mut board: Vec<Vec<Option<Piece>>> = vec![vec![None; 8]; 8];
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
                    let piece = match Piece::from_id(c, row as u8, col as u8) {
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