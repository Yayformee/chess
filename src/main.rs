#![feature(core, io, unicode, collections, str_words)]
use std::io::{stdin, BufRead};
use color::Color;
use chess_move::MoveType;
use board::ChessBoard;

mod color;
mod board;
mod chess_move;
mod tests;

fn main() {
	let mut game_board = ChessBoard::new();
	let mut current_color = Color::White;
	let stdin = stdin();
	
	loop{
		//display logic
		println!("\n");
		game_board.display();
		print!("\nCurrent color is {}", current_color.display());
		if game_board.check_king_check(current_color){
			print!(". Your king is in check");
			if game_board.check_king_mate(current_color){
				println!(" and mate! You lose");
				return;
			}
		}
		println!(". Make your move: ");
		
		//input logic
		let mut input = String::new();
		if let Err(_) = stdin.lock().read_line(&mut input){
			println!("Failed to read line.");
			continue;
		}
		let trimmed = input.trim();
		if trimmed == "exit" {
			return;
		}
		
		// move logic
		let opt = MoveType::parse(&trimmed);
		if !game_board.check_move(opt, current_color) {
			println!("Invalid Move!");
			continue;
		} else {
			game_board = game_board.apply_move_type(opt.unwrap());
		}
		
		//End of turn logic
		current_color = current_color.other();
	}
}