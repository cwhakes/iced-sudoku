mod style;
mod sudoku;

use sudoku::{Cell, Sudoku, SUBREGION_COLUMNS, SUBREGION_ROWS};

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
        cell_index: (usize, usize),
    }
}

impl Sandbox for SudokuView {
    type Message = Message;

    fn new() -> SudokuView {
        let mut sudoku = Sudoku::generate();
        //sudoku.solve().fix().prune().solve();
        sudoku.fix().prune(50);
        SudokuView {
            sudoku: sudoku,
            states: vec![State::new(); sudoku::SUDOKU_AREA],
        }
    }

    fn title(&self) -> String {
        "Sudoku!".to_string()
    }

    fn view(&mut self) -> Element<Message> {
        //Text::new(self.text).size(50).into()
        let mut grid = Column::new();
        let mut states = self.states.iter_mut();
        for major_i in 0..SUBREGION_COLUMNS {
            let mut major_row = Row::new();
            for major_j in 0..SUBREGION_ROWS {
                let mut subregion = Column::new();
                for minor_i in 0..SUBREGION_ROWS {
                    let mut minor_row = Row::new();
                    for minor_j in 0..SUBREGION_COLUMNS {
                        let i = major_i * SUBREGION_ROWS + minor_i;
                        let j = major_j * SUBREGION_COLUMNS + minor_j;
                        let state = states.next().unwrap();
                        let cell = &self.sudoku[(i, j)];
                        let is_valid = self.sudoku.validate_cell((i, j));

                        minor_row = minor_row.push(element_from_cell((i, j), cell, state, is_valid));
                    }
                    subregion = subregion.push(minor_row);
                }
                major_row = major_row.push(Container::new(subregion).style(style::SubregionBorder));
            }
            grid = grid.push(major_row);
        }
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
        }
    }
}

fn element_from_cell<'a>(index: (usize, usize), cell: &'a Cell, state: &'a mut State, is_valid: bool) -> Element<'a, Message> {
    let inner: Element<_> = match cell {
        Cell::Fixed(inner) => Text::new(inner.to_string()).into(),
        Cell::Variable(_) => {
            let value = match cell.read() {
                0 => "".to_string(),
                num @ _ => num.to_string(),
            };
            let mut text = TextInput::new(
                state,
                "",
                &value,
                move |val| Message::ChangedCell {
                    new_value: val.to_owned(),
                    cell_index: index,
                },
            );
            if !is_valid {
                text = text.style(style::CellInput::new(is_valid));
            }
            text.width(Length::Units(20)).into()
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

fn main() {
    SudokuView::run(Default::default());
}
