use crate::style;
use crate::sudoku::{Cell, Sudoku};
use crate::Message;

use iced::widget::{Column, Container, Row, Text, TextInput};
use iced::{theme, Element, Length};

pub struct SudokuView {
	pub sudoku: Sudoku,
}

impl SudokuView {
	pub fn new() -> Self {
		Self::new_from(Sudoku::generate_unsolved(3, 3, 50))
	}

	pub fn new_from(sudoku: Sudoku) -> Self {
		Self { sudoku }
	}

	pub fn view(&self) -> Element<Message> {
		let subregion_columns = self.sudoku.subregion_columns();
		let subregion_rows = self.sudoku.subregion_rows();
		let mut grid = Column::new();
		for major_i in 0..subregion_columns {
			let mut major_row = Row::new();
			for major_j in 0..subregion_rows {
				let mut subregion = Column::new();
				for minor_i in 0..subregion_rows {
					let mut minor_row = Row::new();
					for minor_j in 0..subregion_columns {
						let i = major_i * subregion_rows + minor_i;
						let j = major_j * subregion_columns + minor_j;
						let cell = &self.sudoku[(i, j)];
						let is_valid = self.sudoku.validate_cell((i, j));

						minor_row = minor_row.push(element_from_cell((i, j), cell, is_valid));
					}
					subregion = subregion.push(minor_row);
				}
				major_row = major_row.push(
					Container::new(subregion)
						.style(theme::Container::Custom(Box::new(style::SubregionBorder))),
				);
			}
			grid = grid.push(major_row);
		}
		grid.into()
	}

	pub fn update(&mut self, message: Message) {
		if let Message::ChangedCell {
			new_value,
			cell_index,
		} = message
		{
			let max_value = self.sudoku.length_u8();
			if new_value.is_empty() {
				self.sudoku[cell_index].set(0);
			// Set the value if value successfully parses to u8 between 1 and 9
			} else if let Ok(val) = new_value.parse() {
				if 1 <= val && val <= max_value {
					self.sudoku[cell_index].set(val);
				}
			}
		}
	}
}

fn element_from_cell(index: (usize, usize), cell: &Cell, is_valid: bool) -> Element<Message> {
	let inner: Element<_> = match cell {
		Cell::Fixed(inner) => Text::new(inner.to_string()).into(),
		Cell::Variable(_) => {
			TextInput::new(
				/*state, */ "",
				&cell.text(),
				move |val| Message::ChangedCell {
					new_value: val,
					cell_index: index,
				},
			)
			.style(theme::TextInput::Custom(Box::new(style::CellInput::new(
				is_valid,
			))))
			.padding(3)
			.width(Length::Units(20))
			.into()
		},
	};

	Container::new(inner)
		.width(Length::Units(32))
		.center_x()
		.height(Length::Units(32))
		.center_y()
		.style(theme::Container::Custom(Box::new(style::CellBorder)))
		.into()
}
