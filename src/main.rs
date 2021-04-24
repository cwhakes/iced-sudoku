#![allow(dead_code)]
#![windows_subsystem = "windows"]

mod save;
mod style;
mod sudoku;
mod sudoku_view;

use save::SaveButtons;
use sudoku_view::SudokuView;

use iced::{Align, Column, Container, Element, Length, Sandbox, Space, Text};

const TITLE: &str = "SUDOKU!";

struct SudokuApp {
	game: SudokuView,
	save_buttons: SaveButtons,
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Message {
	ChangedCell {
		new_value: String,
		cell_index: (usize, usize),
	},
	FileOp(FileOp),
	Regenerate,
}

#[derive(Clone, Debug)]
pub enum FileOp {
	Save,
	Load,
}

impl Sandbox for SudokuApp {
	type Message = Message;

	fn new() -> SudokuApp {
		let mut app = SudokuApp {
			game: SudokuView::new(),
			save_buttons: SaveButtons::new(),
		};
		#[cfg(not(target_arch = "wasm32"))]
		if let Some(path) = std::env::args().nth(1) {
			let save_file = app.save_buttons.load_from_path(path).unwrap();
			app.game.sudoku = bincode::deserialize(&save_file).unwrap();
		}
		app
	}

	fn title(&self) -> String {
		TITLE.to_string()
	}

	fn view(&mut self) -> Element<Message> {
		let column = Column::new()
			.align_items(Align::Center)
			.push(Text::new(TITLE).size(32))
			.push(Space::with_height(Length::Units(20)))
			.push(self.game.view())
			.push(self.save_buttons.view());

		Container::new(column)
			.width(Length::Fill)
			.center_x()
			.height(Length::Fill)
			.center_y()
			.into()
	}

	fn update(&mut self, message: Message) {
		self.game.update(message.clone());
		match message {
			Message::FileOp(op) =>
			{
				#[cfg(not(target_arch = "wasm32"))]
				match op {
					FileOp::Load => {
						if let Some(save_file) = self.save_buttons.load() {
							self.game =
								SudokuView::new_from(bincode::deserialize(&save_file).unwrap());
						}
					}
					FileOp::Save => {
						let save_file = bincode::serialize(&self.game.sudoku).unwrap();
						self.save_buttons.save(save_file)
					}
				}
			}
			Message::Regenerate => {
				self.save_buttons.reset();
				self.game = SudokuView::new();
			}
			_ => {}
		}
	}
}

fn main() {
	#[cfg(target_arch = "wasm32")]
	{
		web_sys::console::log_1(&"Starting Sudoku...".into());
	}

	SudokuApp::run(Default::default()).unwrap();
}
