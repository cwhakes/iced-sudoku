use iced::widget::{container, text_input};
use iced::Theme;
use iced::{Background, Color};

const GOOD_INPUT: text_input::Appearance = text_input::Appearance {
	background: Background::Color(Color::WHITE),
	border_radius: 4.0,
	border_width: 1.0,
	border_color: Color::BLACK,
};

const BAD_INPUT: text_input::Appearance = text_input::Appearance {
	background: Background::Color(Color {
		r: 1.0,
		g: 0.75,
		b: 0.75,
		a: 1.0,
	}),
	border_radius: 4.0,
	border_width: 1.0,
	border_color: Color::BLACK,
};

pub struct CellInput {
	cell_is_valid: bool,
}

pub struct CellBorder;
pub struct SubregionBorder;

impl CellInput {
	pub fn new(cell_is_valid: bool) -> Self {
		Self { cell_is_valid }
	}

	fn appearance(&self) -> text_input::Appearance {
		match self.cell_is_valid {
			true => GOOD_INPUT,
			false => BAD_INPUT,
		}
	}
}

impl text_input::StyleSheet for CellInput {
	type Style = Theme;

	fn active(&self, _style: &Self::Style) -> text_input::Appearance {
		self.appearance()
	}
	fn focused(&self, _style: &Self::Style) -> text_input::Appearance {
		self.appearance()
	}
	fn placeholder_color(&self, _style: &Self::Style) -> Color {
		Color::WHITE
	}
	fn value_color(&self, _style: &Self::Style) -> Color {
		Color::BLACK
	}
	fn selection_color(&self, _style: &Self::Style) -> Color {
		Color::from_rgb8(200, 200, 255)
	}
}

impl container::StyleSheet for CellBorder {
	type Style = Theme;

	fn appearance(&self, _style: &Self::Style) -> container::Appearance {
		container::Appearance {
			text_color: None,
			background: None,
			border_radius: 0.0,
			border_width: 1.0,
			border_color: Color::BLACK,
		}
	}
}

impl container::StyleSheet for SubregionBorder {
	type Style = Theme;

	fn appearance(&self, _style: &Self::Style) -> container::Appearance {
		container::Appearance {
			text_color: None,
			background: None,
			border_radius: 0.0,
			border_width: 2.0,
			border_color: Color::BLACK,
		}
	}
}
