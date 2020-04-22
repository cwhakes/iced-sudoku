mod sudoku;

use sudoku::{Cell, Sudoku};

use iced::{Background, Color, Column, Container, Element, Length, Row, Sandbox, Text};
use iced::text_input::{State, Style, StyleSheet, TextInput};

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

pub struct CellInputStyle {
    cell_is_valid: bool,
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
        let mut iter = self.sudoku.iter().zip(self.states.iter_mut()).enumerate();
        for i in 0..sudoku::SUDOKU_MAX {
            let mut row = Row::new();
            for j in 0..sudoku::SUDOKU_MAX {
                let (index, (cell, state)) = iter.next().unwrap();
                let is_valid = self.sudoku.validate_cell((i, j));

                row = row.push(element_from_cell(index, cell, state, is_valid));
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
        }
    }
}

fn element_from_cell<'a>(index: usize, cell: &'a Cell, state: &'a mut State, is_valid: bool) -> Element<'a, Message> {
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
                text = text.style(CellInputStyle::new(is_valid));
            }
            text.into()
        }
    };

    Container::new(inner)
        .width(Length::Units(32))
        .height(Length::Units(32))
        .into()
}

impl CellInputStyle {
    fn new(cell_is_valid: bool) -> CellInputStyle {
        CellInputStyle {
            cell_is_valid
        }
    }

    fn style(&self) -> Style {
        match self.cell_is_valid {
            true => GOOD_INPUT,
            false => BAD_INPUT,
        }
    }
}

impl StyleSheet for CellInputStyle {
    fn active(&self) -> Style { self.style() }
    fn focused(&self) -> Style { self.style() }
    fn placeholder_color(&self) -> Color { Color::WHITE }
    fn value_color(&self) -> Color { Color::BLACK }
    fn selection_color(&self) -> Color { Color::from_rgb8(200, 200, 255) }
}

const GOOD_INPUT: Style = Style {
    background: Background::Color(
        Color::WHITE,
    ),
    border_radius: 0,
    border_width: 2,
    border_color: Color::BLACK,
};

const BAD_INPUT: Style = Style {
    background: Background::Color(
        Color { r: 1.0, g: 0.75, b: 0.75, a: 1.0 },
    ),
    border_radius: 0,
    border_width: 2,
    border_color: Color::BLACK,
};

fn main() {
    SudokuView::run(Default::default());
}
