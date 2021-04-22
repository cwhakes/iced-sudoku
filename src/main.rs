#![allow(dead_code)]
#![windows_subsystem = "windows"]

mod save;
mod style;
mod sudoku;
mod sudoku_view;

use save::SaveButtons;
use sudoku_view::SudokuView;

use iced::{Align, Column, Container, Element, Length, Sandbox, Space, Text};

use bincode;

const TITLE: &'static str = "SUDOKU!";

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
        if let Some(path) = std::env::args().skip(1).next() {
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
        if let Message::FileOp(op) = message {
            match op {
                FileOp::Load => {
                    if let Some(save_file) = self.save_buttons.load() {
                        self.game.sudoku = bincode::deserialize(&save_file).unwrap();
                    }
                }
                FileOp::Save => {
                    let save_file = bincode::serialize(&self.game.sudoku).unwrap();
                    self.save_buttons.save(save_file)
                }
            }
        }
    }
}

fn main() {
    SudokuApp::run(Default::default()).unwrap();
}
