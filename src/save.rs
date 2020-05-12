use crate::{Message, FileOp};

use std::fs;

use iced::{Element, Row, Text};
use iced::button::{Button, State};
use tinyfiledialogs::{open_file_dialog, save_file_dialog_with_filter};

pub struct SaveButtons {
    file_path: Option<String>,
    save: State,
    load: State,
}

impl SaveButtons {
    pub fn new() -> SaveButtons {
        SaveButtons {
            file_path: None,
            save: State::new(),
            load: State::new(),
        }
    }

    pub fn view(&mut self) -> Element<Message> {
        Row::new()
            .push(
                Button::new(&mut self.save, Text::new("Save"))
                    .on_press(Message::FileOp(FileOp::Save))
            )
            .push(
                Button::new(&mut self.load, Text::new("Load"))
                    .on_press(Message::FileOp(FileOp::Load))
            )
            .into()
    }

    pub fn save(&mut self, save_file: Vec<u8>) {
        if self.file_path.is_none() {
            self.file_path = save_file_dialog_with_filter(
                "Sudoku! Save File",
                "save_file1.sudoku",
                &["*.sudoku"],
                "Sudoku! Files",
            )
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
        if let Some(file_path) = open_file_dialog(
            "Sudoku! Load File",
            "",
            Some((&["*.sudoku"], "Sudoku! Files")),
        ) {
            self.load_from_path(file_path)
        } else {None}
    }
}
