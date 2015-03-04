#[allow(unused_imports)]
use color::Color;
#[allow(unused_imports)]
use chess_move::MoveType;
#[allow(unused_imports)]
use board::ChessBoard;

#[test]
fn basic(){
	let game = ChessBoard::new();
	
	assert!(!game.check_king_check(Color::White));
	assert!(!game.check_king_check(Color::Black));
	assert!(!game.check_king_mate(Color::White));
	assert!(!game.check_king_mate(Color::Black));
	assert!(game.check_move(MoveType::parse("a2 a3"), Color::White));
	assert!(game.check_move(MoveType::parse("a2 a4"), Color::White));
	assert!(game.check_move(MoveType::parse("b2 b3"), Color::White));
	assert!(game.check_move(MoveType::parse("b2 b4"), Color::White));
}

#[test]
fn check(){
	let game = ChessBoard::new()
		.apply_move_type(MoveType::parse("e2 e4").unwrap())
		.apply_move_type(MoveType::parse("a7 a5").unwrap())
		.apply_move_type(MoveType::parse("e4 e5").unwrap())
		.apply_move_type(MoveType::parse("a8 a6").unwrap())
		.apply_move_type(MoveType::parse("e5 e6").unwrap())
		.apply_move_type(MoveType::parse("a6 e6").unwrap());
		
	assert!(game.check_king_check(Color::White));
	assert!(!game.check_king_mate(Color::White));
}

#[test]
fn mate(){	
	let game = ChessBoard::new()
		.apply_move_type(MoveType::parse("f2 f4").unwrap())
		.apply_move_type(MoveType::parse("e7 e5").unwrap())
		.apply_move_type(MoveType::parse("g2 g4").unwrap())
		.apply_move_type(MoveType::parse("d8 h4").unwrap());
	
	assert!(game.check_king_check(Color::White));
	assert!(game.check_king_mate(Color::White));
}

#[test]
fn promotion(){
	let game = ChessBoard::new()
		.apply_move_type(MoveType::parse("f2 f7").unwrap())
		.apply_move_type(MoveType::parse("d1 f3").unwrap());
		
	assert!(game.check_king_check(Color::Black));
	assert!(game.check_king_mate(Color::Black));
}