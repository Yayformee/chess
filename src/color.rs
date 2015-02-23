#[derive(Copy, PartialEq)]
pub enum Color{
	White,
	Black,
}

impl Color{
	pub fn other(&self) -> Color{
		match *self{
			Color::White => Color::Black,
			Color::Black => Color::White,
		}
	}
	
	pub fn display(&self) -> &str{
		match *self{
			Color::White => "white",
			Color::Black => "black",
		}
	}
}