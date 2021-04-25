use crate::style;
use crate::sudoku::{Cell, Sudoku};
use crate::Message;

use iced::text_input::{State, TextInput};
use iced::{Column, Container, Element, Length, Row, Text};

pub struct SudokuView {
	pub sudoku: Sudoku,
	states: Vec<State>,
}

impl SudokuView {
	pub fn new() -> Self {
		Self::new_from(Sudoku::generate_unsolved(3, 3, 50))
	}

	pub fn new_from(sudoku: Sudoku) -> Self {
		let state_len = sudoku.area();
		Self {
			sudoku,
			states: vec![State::new(); state_len],
		}
	}

	pub fn view(&mut self) -> Element<Message> {
		let subregion_columns = self.sudoku.subregion_columns();
		let subregion_rows = self.sudoku.subregion_rows();
		//Text::new(self.text).size(50).into()
		let mut grid = Column::new();
		let mut states = self.states.iter_mut();
		for major_i in 0..subregion_columns {
			let mut major_row = Row::new();
			for major_j in 0..subregion_rows {
				let mut subregion = Column::new();
				for minor_i in 0..subregion_rows {
					let mut minor_row = Row::new();
					for minor_j in 0..subregion_columns {
						let i = major_i * subregion_rows + minor_i;
						let j = major_j * subregion_columns + minor_j;
						let state = states.next().unwrap();
						let cell = &self.sudoku[(i, j)];
						let is_valid = self.sudoku.validate_cell((i, j));

						minor_row =
							minor_row.push(element_from_cell((i, j), cell, state, is_valid));
					}
					subregion = subregion.push(minor_row);
				}
				major_row = major_row.push(Container::new(subregion).style(style::SubregionBorder));
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
				self.sudoku[cell_index].set(0)
			// Set the value if value successfully parses to u8 between 1 and 9
			} else if let Ok(val) = new_value.parse() {
				if 1 <= val && val <= max_value {
					self.sudoku[cell_index].set(val);
				}
			}
		}
	}
}

fn element_from_cell<'a>(
	index: (usize, usize),
	cell: &'a Cell,
	state: &'a mut State,
	is_valid: bool,
) -> Element<'a, Message> {
	let inner: Element<_> = match cell {
		Cell::Fixed(inner) => Text::new(inner.to_string()).into(),
		Cell::Variable(_) => {
				TextInput::new(state, "", &cell.text(), move |val| Message::ChangedCell {
					new_value: val,
					cell_index: index,
				})
				.style(style::CellInput::new(is_valid))
				.width(Length::Units(20))
				.into()
		}
	};

	Container::new(inner)
		.width(Length::Units(32))
		.center_x()
		.height(Length::Units(32))
		.center_y()
		.style(style::CellBorder)
		.into()
}
