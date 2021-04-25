use iced::{container, text_input};
use iced::{Background, Color};

const GOOD_INPUT: text_input::Style = text_input::Style {
	background: Background::Color(Color::WHITE),
	border_radius: 4.0,
	border_width: 0.5,
	border_color: Color::BLACK,
};

const BAD_INPUT: text_input::Style = text_input::Style {
	background: Background::Color(Color {
		r: 1.0,
		g: 0.75,
		b: 0.75,
		a: 1.0,
	}),
	border_radius: 4.0,
	border_width: 0.5,
	border_color: Color::BLACK,
};

pub struct CellInput {
	cell_is_valid: bool,
}

pub struct CellBorder;
pub struct SubregionBorder;

impl CellInput {
	pub fn new(cell_is_valid: bool) -> CellInput {
		CellInput { cell_is_valid }
	}

	fn style(&self) -> text_input::Style {
		match self.cell_is_valid {
			true => GOOD_INPUT,
			false => BAD_INPUT,
		}
	}
}

impl text_input::StyleSheet for CellInput {
	fn active(&self) -> text_input::Style {
		self.style()
	}
	fn focused(&self) -> text_input::Style {
		self.style()
	}
	fn placeholder_color(&self) -> Color {
		Color::WHITE
	}
	fn value_color(&self) -> Color {
		Color::BLACK
	}
	fn selection_color(&self) -> Color {
		Color::from_rgb8(200, 200, 255)
	}
}

impl container::StyleSheet for CellBorder {
	fn style(&self) -> container::Style {
		container::Style {
			text_color: None,
			background: None,
			border_radius: 0.0,
			border_width: 1.0,
			border_color: Color::BLACK,
		}
	}
}

impl container::StyleSheet for SubregionBorder {
	fn style(&self) -> container::Style {
		container::Style {
			text_color: None,
			background: None,
			border_radius: 0.0,
			border_width: 2.0,
			border_color: Color::BLACK,
		}
	}
}
