mod sudoku;

use sudoku::{Cell, Sudoku};

use iced::{Column, Container, Element, Length, Row, Sandbox, Text};
use iced::text_input::{State, TextInput};

struct SudokuView {
    sudoku: Sudoku,
    states: Vec<State>,
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Message {
    ChangedCell{
        new_value: String,
        cell_index: usize,
    }
}

impl Sandbox for SudokuView {
    type Message = Message;

    fn new() -> SudokuView {
        let mut sudoku = Sudoku::default();
        //dbg!(sudoku.row(5).collect::<Vec<_>>());
        //dbg!(sudoku.column(5).collect::<Vec<_>>());
        sudoku.solve().fix();
        //dbg!(sudoku.subregion(5).collect::<Vec<_>>());
        SudokuView {
            sudoku: sudoku,
            states: vec![State::new(); sudoku::SUDOKU_AREA],
        }
    }

    fn title(&self) -> String {
        "Sandbox Title".to_string()
    }

    fn view(&mut self) -> Element<Message> {
        //Text::new(self.text).size(50).into()
        let mut grid = Column::new();
        let mut iter = self.sudoku.iter().zip(self.states.iter_mut()).enumerate();
        for _ in 0..sudoku::SUDOKU_MAX {
            let mut row = Row::new();
            for _ in 0..sudoku::SUDOKU_MAX {
                let (index, (cell, state)) = iter.next().unwrap();
                row = row.push(element_from_cell(index, cell, state));
            }
            grid = grid.push(row);
        };
        grid.into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ChangedCell{new_value, cell_index} => {
                if new_value == "" {
                    self.sudoku[cell_index].set(0)
                // Set the value if value successfully parses to u8 between 1 and 9
                } else if let Ok(val @ 1..=sudoku::SUDOKU_MAX_U8) = new_value.parse() {
                    self.sudoku[cell_index].set(val);
                }
            }
            _ => {}
        }
    }
}

fn element_from_cell<'a>(index: usize, cell: &'a Cell, state: &'a mut State) -> Element<'a, Message> {
    let inner: Element<_> = match cell {
        Cell::Fixed(inner) => Text::new(inner.to_string()).into(),
        Cell::Variable(_) => {
            let value = match cell.read() {
                0 => "".to_string(),
                // We
                num @ _ => num.to_string(),
            };
            TextInput::new(
                state,
                "",
                &value,
                move |val| Message::ChangedCell {
                    new_value: val.to_owned(),
                    cell_index: index,
                },
            ).into()
        }
    };
    Container::new(inner)
        .width(Length::Units(32))
        .height(Length::Units(32))
        .into()
}

fn main() {
    SudokuView::run(Default::default());
}
