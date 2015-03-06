use color::Color;
use chess_move::{MoveType, Move, Point};
use std::num::SignedInt;

#[derive(Copy, Clone, PartialEq)]
pub enum Piece{
	Pawn,
	Rook,
	Knight,
	Bishop,
	Queen,
	King,
}

impl Piece{
	pub fn parse(input: &str) -> Option<Piece>{
		match input{
			"pawn" =>   Some(Piece::Pawn),
			"rook" =>   Some(Piece::Rook),
			"knight" => Some(Piece::Knight),
			"bishop" => Some(Piece::Bishop),
			"queen" =>  Some(Piece::Queen),
			"king" =>   Some(Piece::King),
			_ => None,
		}
	}
}

#[derive(Copy, PartialEq)]
pub enum Tile{
	Empty,
	Piece(Piece, Color),
}

impl Tile{
	pub fn display(&self) -> &str{
		match *self{
			Tile::Empty => "  ",
			
			Tile::Piece(Piece::Pawn,	Color::White) => "wp",
			Tile::Piece(Piece::Rook,	Color::White) => "wr",
			Tile::Piece(Piece::Knight,	Color::White) => "wk",
			Tile::Piece(Piece::Bishop, 	Color::White) => "wb",
			Tile::Piece(Piece::Queen, 	Color::White) => "wq",
			Tile::Piece(Piece::King, 	Color::White) => "wK",
			
			Tile::Piece(Piece::Pawn,	Color::Black) => "bp",
			Tile::Piece(Piece::Rook,	Color::Black) => "br",
			Tile::Piece(Piece::Knight,	Color::Black) => "bk",
			Tile::Piece(Piece::Bishop, 	Color::Black) => "bb",
			Tile::Piece(Piece::Queen, 	Color::Black) => "bq",
			Tile::Piece(Piece::King, 	Color::Black) => "bK",
		}
	}
	
	fn color(&self) -> Option<Color>{
		match *self{
			Tile::Piece(_, c) => Some(c),
			Tile::Empty => None,
		}
	}
}

#[derive(Clone)]
pub struct ChessBoard{
	board: [[Tile; 8]; 8],
	moves: Vec<Move>,
}

impl ChessBoard{
	pub fn new() -> ChessBoard{
		ChessBoard{
			board: [
				[
					Tile::Piece(Piece::Rook,	Color::White),
					Tile::Piece(Piece::Knight,	Color::White),
					Tile::Piece(Piece::Bishop, 	Color::White),
					Tile::Piece(Piece::Queen, 	Color::White),
					Tile::Piece(Piece::King, 	Color::White),
					Tile::Piece(Piece::Bishop, 	Color::White),
					Tile::Piece(Piece::Knight, 	Color::White),
					Tile::Piece(Piece::Rook,	Color::White),
				],
				[Tile::Piece(Piece::Pawn,	Color::White); 8],
				[Tile::Empty; 8],
				[Tile::Empty; 8],
				[Tile::Empty; 8],
				[Tile::Empty; 8],
				[Tile::Piece(Piece::Pawn,	Color::Black); 8],
				[
					Tile::Piece(Piece::Rook,	Color::Black),
					Tile::Piece(Piece::Knight,	Color::Black),
					Tile::Piece(Piece::Bishop, 	Color::Black),
					Tile::Piece(Piece::Queen, 	Color::Black),
					Tile::Piece(Piece::King, 	Color::Black),
					Tile::Piece(Piece::Bishop, 	Color::Black),
					Tile::Piece(Piece::Knight, 	Color::Black),
					Tile::Piece(Piece::Rook,	Color::Black),
				],
			],
			moves: vec![],
		}
	}
	
	fn tile_at(&self, p: Point) -> Tile{
		self.board[p.y][p.x]
	}
	
	pub fn display(&self){
		println!(" | A| B| C| D| E| F| G| H|");
		println!("---------------------------");
		for (row, num) in self.board.iter().zip(0u8 .. 8).rev(){
			print!("{}|", num + 1);
			for tile in row.iter(){
				print!("{}|", tile.display());
			}
			println!("{}", num + 1);
			println!("---------------------------");
		}
		println!(" | A| B| C| D| E| F| G| H|");
	}
	
	pub fn check_move(&self, opt: Option<MoveType>, color: Color) -> bool{
		match opt{
			None => false,
			Some(mvt) => (match mvt{
				MoveType::Basic(mv) => self.check_basic_move(mv, color),
				MoveType::EnPassant(mv) => self.check_enpassant(mv, color),
				MoveType::Promotion(mv, piece) => self.check_promotion(mv, color, piece),
				MoveType::Castling(mvk, mvr) => self.check_castling(mvk, mvr, color),
			}) && !self.apply_move_type(mvt).check_king_check(color)
		}
	}
	
	fn check_enpassant(&self, mv: Move, color: Color) -> bool{
		self.tile_at(mv.from) == Tile::Piece(Piece::Pawn, color) &&
		self.tile_at(mv.to) == Tile::Empty && 
		match color{
			Color::White => (mv.from.x == mv.to.x - 1 || mv.from.x == mv.to.x + 1) && mv.from.y == 4 && mv.to.y == 5
				&& self.moves[self.moves.len() - 1] == Move{from:Point{x:mv.to.x,y:6},to:Point{x:mv.to.x,y:4}},
			Color::Black => (mv.from.x == mv.to.x - 1 || mv.from.x == mv.to.x + 1) && mv.from.y == 3 && mv.to.y == 2
				&& self.moves[self.moves.len() - 1] == Move{from:Point{x:mv.to.x,y:1},to:Point{x:mv.to.x,y:3}},
		}
	}
	
	fn check_promotion(&self, mv: Move, color: Color, piece: Piece) -> bool{
		self.tile_at(mv.from) == Tile::Piece(Piece::Pawn, color) && match piece{
			Piece::Rook|Piece::Knight|Piece::Bishop|Piece::Queen => true,
			_ => false,
		} && self.check_pawn_move(mv, color, true)
	}
	
	fn check_castling(&self, mvk: Move, mvr: Move, color: Color) -> bool{
		self.tile_at(mvk.from) == Tile::Piece(Piece::King, color) &&
		self.tile_at(mvr.from) == Tile::Piece(Piece::Rook, color) &&
		self.check_no_moves(mvk.from) && self.check_no_moves(mvr.from) &&
		(mvr.to.x == 3 && mvk.to.x == 2 || mvr.to.x == 5 && mvk.to.x == 6) && mvk.to.y == mvk.from.y &&
		self.check_rook_move(mvr) && !self.check_king_check(color) && 
		!self.apply_move(Move{
			from: mvk.from,
			to:   Point{x: ((mvk.to.x as i8 - mvk.from.x as i8)/2 + mvk.from.x as i8) as usize, y: mvk.to.y}
		}).check_king_check(color) &&
		!self.apply_move_type(MoveType::Basic(mvk)).check_king_check(color)
	}
	
	fn check_no_moves(&self, p: Point) -> bool{
		!self.moves.iter().any(|mv| (mv.from == p) || (mv.to == p))
	}
	
	fn check_basic_move(&self, mv: Move, color: Color) -> bool{
		match self.board[mv.from.y][mv.from.x]{
			Tile::Empty => false,
			Tile::Piece(p, c) => c == color && match self.tile_at(mv.to){
				Tile::Empty => true,
				Tile::Piece(_, c2) => c2 != color,
			} && self.check_piece_move(p, mv, c)
		}
	}
	
	fn check_piece_move(&self, piece: Piece, mv: Move, color: Color) -> bool{
		match piece{
			Piece::Pawn => self.check_pawn_move(mv, color, false),
			Piece::Rook => self.check_rook_move(mv),
			Piece::Knight => self.check_knight_move(mv),
			Piece::Bishop => self.check_bishop_move(mv),
			Piece::Queen => self.check_rook_move(mv) || self.check_bishop_move(mv),
			Piece::King => self.check_king_move(mv),
		}
	}
	
	fn check_pawn_move(&self, mv: Move, color: Color, promo: bool) -> bool{
		match color{
			Color::White => (match self.tile_at(mv.to){
				Tile::Empty => mv.from.x == mv.to.x && (mv.from.y == mv.to.y - 1 || mv.from.y == 1 && mv.to.y == 3),
				_ => mv.from.y == mv.to.y - 1 && (mv.from.x == mv.to.x - 1 || mv.from.x == mv.to.x + 1),
			}) && (mv.to.y != 7 || promo || self.tile_at(mv.to) == Tile::Piece(Piece::King, Color::Black)),
			Color::Black => (match self.tile_at(mv.to){
				Tile::Empty => mv.from.x == mv.to.x && (mv.from.y == mv.to.y + 1 || mv.from.y == 6 && mv.to.y == 4),
				_ => mv.from.y == mv.to.y + 1 && (mv.from.x == mv.to.x - 1 || mv.from.x == mv.to.x + 1),
			}) && (mv.to.y != 0 || promo || self.tile_at(mv.to) == Tile::Piece(Piece::King, Color::White)),
		}
	}
	
	fn check_rook_move(&self, mv: Move) -> bool{
		mv.from.x == mv.to.x && if mv.from.y > mv.to.y {
			mv.to.y + 1 .. mv.from.y
		} else {
			mv.from.y + 1 .. mv.to.y
		}.all(|y:usize| self.board[y][mv.to.x] == Tile::Empty) || mv.from.y == mv.to.y && if mv.from.x > mv.to.x {
			mv.to.x + 1 .. mv.from.x
		} else {
			mv.from.x + 1 .. mv.to.x
		}.all(|x:usize| self.board[mv.to.y][x] == Tile::Empty)
	}
	
	fn check_knight_move(&self, mv: Move) -> bool{
		(mv.from.x as i8 - mv.to.x as i8).abs() == 2 && (mv.from.y as i8 - mv.to.y as i8).abs() == 1 ||
		(mv.from.x as i8 - mv.to.x as i8).abs() == 1 && (mv.from.y as i8 - mv.to.y as i8).abs() == 2
	}
	
	fn check_bishop_move(&self, mv: Move) -> bool{
		let d_x = mv.to.x as i8 - mv.from.x as i8;
		let d_y = mv.to.y as i8 - mv.from.y as i8;
		
		d_x.abs() == d_y.abs() && if (d_x > 0) == (d_y > 0){
			if d_x > 0 {
				(mv.from.x + 1 .. mv.to.x).zip(mv.from.y + 1 .. mv.to.y)
			} else {
				(mv.to.x + 1 .. mv.from.x).zip(mv.to.y + 1 .. mv.from.y)
			}.all(|(x, y):(usize, usize)| self.board[y][x] == Tile::Empty)
		} else {
			if d_x > 0 {
				(mv.from.x + 1 .. mv.to.x).zip((mv.to.y + 1 .. mv.from.y).rev())
			} else {
				(mv.to.x + 1 .. mv.from.x).zip((mv.from.y + 1 .. mv.to.y).rev())
			}.all(|(x, y):(usize, usize)| self.board[y][x] == Tile::Empty)
		}
	}
	
	fn check_king_move(&self, mv: Move) -> bool{
		(mv.from.x as i8 - mv.to.x as i8).abs() <= 1 && (mv.from.y as i8 - mv.to.y as i8).abs() <= 1
	}
	
	pub fn check_king_check(&self, color: Color) -> bool{
		let king = self.trace_moves(Point{x:4, y:match color {
			Color::White => 0,
			Color::Black => 7,
		}}).unwrap();
		
		self.collect_piece_coords(color.other()).iter().any(|&p|
			self.check_basic_move(Move{
				from: p,
				to:   king,
			}, color.other())
		)
	}
	
	pub fn check_king_mate(&self, color: Color) -> bool{
		self.collect_piece_coords(color).iter().all(|&p|{
			(0usize .. 8).all(|x|{
				(0usize .. 8).all(|y|{
					!self.check_move(Some(MoveType::Basic(Move{
						from: p,
						to:   Point{x:x, y:y},
					})), color)
				})
			})
		})
	}
	
	fn collect_piece_coords(&self, color: Color) -> Vec<Point>{
		(0usize .. 8).flat_map(|x|(match color{
			Color::White => 0usize .. 2,
			Color::Black => 6usize .. 8,
		}).map(move |y|Point{x:x, y:y})).filter_map(|p|self.trace_moves(p)).collect()
	}
	
	fn trace_moves(&self, p: Point) -> Option<Point>{
		self.moves.iter().fold(Some(p), |opt: Option<Point>, mv: &Move| match opt{
			None => None,
			Some(p) => if mv.from.x == p.x && mv.from.y == p.y {
				Some(mv.to)
			} else if mv.to.x == p.x && mv.to.y == p.y{
				None
			} else {
				Some(p)
			},
		})
	}
	
	pub fn apply_move_type(&self, mvt: MoveType) -> ChessBoard{
		match mvt{
			MoveType::Basic(mv) => self.apply_move(mv),
			MoveType::EnPassant(mv) => self.apply_move(Move{
				from: mv.from, to: Point{x: mv.to.x, y: mv.from.y}
			}).apply_move(Move{
				from: Point{x: mv.to.x, y: mv.from.y}, to: mv.to,
			}),
			MoveType::Promotion(mv, piece) => self.apply_move(mv).set_tile(mv.to, Tile::Piece(piece, self.tile_at(mv.from).color().unwrap())),
			MoveType::Castling(mvk, mvr) => self.apply_move(mvk).apply_move(mvr),
		}
	}
	
	fn apply_move(&self, mv: Move) -> ChessBoard{
		let mut new_board = self.clone();
		new_board.board[mv.to.y][mv.to.x] = self.tile_at(mv.from);
		new_board.board[mv.from.y][mv.from.x] = Tile::Empty;
		new_board.moves.push(mv);
		new_board
	}
	
	fn set_tile(&self, p: Point, t: Tile) -> ChessBoard{
		let mut new_board = self.clone();
		new_board.board[p.y][p.x] = t;
		new_board
	}
}
