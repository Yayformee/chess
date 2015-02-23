use board::Piece;

#[derive(Copy, Clone, PartialEq)]
pub struct Point{
	pub x: usize,
	pub y: usize,
}

impl Point{
	fn parse(input: &str) -> Option<Point>{
		if input.len() != 2 { None } else {
			let (x, y) = (input.char_at(0), input.char_at(1));
			
			if x < 'a' || x > 'h' || y < '1' || y > '8' { None } else {
				Some(Point{
					x: x.to_digit(20).unwrap() - 10,
					y: y.to_digit(10).unwrap() - 1,
				})
			}
		}
	}
}

#[derive(Copy, Clone, PartialEq)]
pub struct Move{
	pub from: Point,
	pub to:   Point,
}

impl Move{
	fn parse(from: &str, to: &str) -> Option<Move>{
		match (Point::parse(from), Point::parse(to)) {
			(_,None)|(None,_) => None,
			(Some(from),Some(to)) => Some(Move{from: from, to: to}),
		}
	}
}

#[derive(Copy, Clone)]
pub enum MoveType{
	Basic	 (Move),
	EnPassant(Move),
	Promotion(Move, Piece),
	Castling (Move, Move),
}

impl MoveType{
	pub fn parse(input: &str) -> Option<MoveType>{
		let mut words = input.words();
		match input.words().count(){
			2 => match Move::parse(words.next().unwrap(), words.next().unwrap()){
				None => None,
				Some(mv) => Some(MoveType::Basic(mv)),
			},
			3 => if words.next().unwrap() != "enpassant" { None } else {
				match Move::parse(words.next().unwrap(), words.next().unwrap()){
					None => None,
					Some(mv) => Some(MoveType::EnPassant(mv)),
				}
			},
			4 => if words.next().unwrap() != "promote" { None } else {
				match (Move::parse(words.next().unwrap(), words.next().unwrap()), Piece::parse(words.next().unwrap())){
					(_,None)|(None,_) => None,
					(Some(mv),Some(piece)) => Some(MoveType::Promotion(mv, piece)),
				}
			},
			5 => if words.next().unwrap() != "castle" { None } else {
				match (Move::parse(words.next().unwrap(), words.next().unwrap()), Move::parse(words.next().unwrap(), words.next().unwrap())){
					(_,None)|(None,_) => None,
					(Some(mvk),Some(mvr)) => Some(MoveType::Castling(mvk, mvr)),
				}
			},
			_ => None,
		}
	}
}