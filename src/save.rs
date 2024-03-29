use crate::{FileOp, Message};

use std::fs;

use iced::widget::{Button, Row, Text};
use iced::Element;

pub struct SaveButtons {
	file_path: Option<String>,
}

impl SaveButtons {
	pub fn new() -> Self {
		Self { file_path: None }
	}

	pub fn view(&self) -> Element<Message> {
		Row::new()
			.push(Button::new(Text::new("New")).on_press(Message::Regenerate))
			.push(Button::new(Text::new("Save")).on_press(Message::FileOp(FileOp::Save)))
			.push(Button::new(Text::new("Load")).on_press(Message::FileOp(FileOp::Load)))
			.into()
	}
}

#[cfg(target_arch = "wasm32")]
impl SaveButtons {
	pub fn reset(&mut self) {
		self.file_path = None;
	}

	pub fn save(&mut self, _save_file: Vec<u8>) {}

	pub fn load_from_path(&mut self, _path: String) -> Option<Vec<u8>> {
		None
	}

	pub fn load(&mut self) -> Option<Vec<u8>> {
		None
	}
}

#[cfg(not(target_arch = "wasm32"))]
impl SaveButtons {
	pub fn reset(&mut self) {
		self.file_path = None;
	}

	pub fn save(&mut self, save_file: Vec<u8>) {
		use tinyfiledialogs::save_file_dialog_with_filter;

		if self.file_path.is_none() {
			self.file_path = save_file_dialog_with_filter(
				"Sudoku! Save File",
				"save_file1.sudoku",
				&["*.sudoku"],
				"Sudoku! Files",
			);
		}
		if let Some(save_file_path) = &self.file_path {
			fs::write(save_file_path, save_file).unwrap();
		}
	}

	pub fn load_from_path(&mut self, path: String) -> Option<Vec<u8>> {
		let file_contents = fs::read(&path).ok();
		self.file_path = Some(path);
		file_contents
	}

	pub fn load(&mut self) -> Option<Vec<u8>> {
		use tinyfiledialogs::open_file_dialog;

		open_file_dialog(
			"Sudoku! Load File",
			"",
			Some((&["*.sudoku"], "Sudoku! Files")),
		)
		.and_then(|file_path| self.load_from_path(file_path))
	}
}
