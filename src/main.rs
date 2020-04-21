mod sudoku;

use sudoku::Sudoku;

use iced::{Column, Element, Row, Sandbox, Text};

#[derive(Default)]
struct SudokuView {
    sudoku: Sudoku,
}

#[derive(Clone, Copy, Debug)]
pub enum Message {
    Placeholder,
}

impl Sandbox for SudokuView {
    type Message = Message;

    fn new() -> SudokuView {
        SudokuView {
            sudoku: Sudoku::default(),
        }
    }

    fn title(&self) -> String {
        "Sandbox Title".to_string()
    }

    fn view(&mut self) -> Element<Message> {
        //Text::new(self.text).size(50).into()
        let mut grid = Column::new();
        let mut iter = self.sudoku.iter();
        for _ in 0..sudoku::ROWS {
            let mut row = Row::new();
            for _ in 0..sudoku::COLUMS {
                let num = iter.next().unwrap().read();
                row = row.push(Text::new(num.to_string()));
            }
            grid = grid.push(row);
        };
        grid.into()
    }

    fn update(&mut self, message: Message) {
        match message {
            _ => {}
        }
    }
}

fn main() {
    SudokuView::run(Default::default());
}
