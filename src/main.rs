#![feature(core, io, unicode, collections)]
use std::old_io as io;
use color::Color;
use chess_move::MoveType;
use board::ChessBoard;

mod color;
mod board;
mod piece;
mod chess_move;

fn main() {
	let mut game_board = ChessBoard::new();
	let mut current_color = Color::White;
	
	loop{
		//display logic
		println!("\n");
		game_board.display();
		println!("\n");
		print!("Current color is {}", current_color.display());
		if game_board.check_king_check(current_color){
			print!(". Your king is in check");
			if game_board.check_king_mate(current_color){
				println!(" and mate! You lose");
				return;
			}
		}
		println!(". Make your move: ");
		
		//input logic
		let input = io::stdin().read_line().ok().expect("Failed to read line");
		let trimmed = input.trim();
		if trimmed == "exit" {
			return;
		}
		
		let mvt = match MoveType::parse(&trimmed){
			Some(mvt) => mvt,
			None => {
				println!("Invalid Move!");
				continue;
			},
		};
		
		// move logic
		if !game_board.check_move(mvt, current_color) { println!("Invalid Move!"); continue; } else {
			game_board = game_board.apply_move_type(mvt);
		}
		
		//End of turn logic
		current_color = current_color.other();
	}
}

#[test]
fn basic(){
	let game = ChessBoard::new();
	
	assert!(!game.check_king_check(Color::White));
	assert!(!game.check_king_check(Color::Black));
	assert!(!game.check_king_mate(Color::White));
	assert!(!game.check_king_mate(Color::Black));
	assert!(game.check_move(MoveType::parse("a2 a3").unwrap(), Color::White));
	assert!(game.check_move(MoveType::parse("a2 a4").unwrap(), Color::White));
	assert!(game.check_move(MoveType::parse("b2 b3").unwrap(), Color::White));
	assert!(game.check_move(MoveType::parse("b2 b4").unwrap(), Color::White));
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