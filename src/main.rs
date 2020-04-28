mod style;
mod sudoku;
mod sudoku_view;

use sudoku_view::SudokuView;

use iced::{Align, Container, Column, Element, Length, Sandbox, Space, Text};

const TITLE: &'static str = "SUDOKU!";

struct SudokuApp {
    game: SudokuView,
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Message {
    ChangedCell{
        new_value: String,
        cell_index: (usize, usize),
    }
}

impl Sandbox for SudokuApp {
    type Message = Message;

    fn new() -> SudokuApp {
        SudokuApp {
            game: SudokuView::new(),
        }
    }

    fn title(&self) -> String {
        TITLE.to_string()
    }

    fn view(&mut self) -> Element<Message> {
        let column = Column::new()
            .align_items(Align::Center)
            .push(Text::new(TITLE)
                .size(32)
            )
            .push(Space::with_height(Length::Units(20)))
            .push(self.game.view());
        
        Container::new(column)
            .width(Length::Fill)
            .center_x()
            .height(Length::Fill)
            .center_y()
            .into()
    }

    fn update(&mut self, message: Message) {
        self.game.update(message);
    }
}

fn main() {
    SudokuApp::run(Default::default());
}
