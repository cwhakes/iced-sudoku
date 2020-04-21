use std::sync::atomic::{AtomicU8, Ordering};

use rand::{thread_rng, Rng};

pub const COLUMNS: usize = 9;
pub const ROWS: usize = 9;

#[derive(Clone, Debug, Default)]
pub struct Sudoku(pub [[Cell; COLUMNS]; ROWS]);

#[derive(Debug)]
pub enum Cell {
    Fixed(u8),
    Variable(AtomicU8),
}

pub struct SudokuIter<'a, T: 'a + Iterator<Item=&'a Cell>> {
    inner: T,
}

impl Sudoku {
    pub fn new() -> Sudoku {
        let mut sudoku = Sudoku::default();
        for row in sudoku.0.iter_mut() {
            for cell in row.iter_mut() {
                let num = thread_rng().gen_range(1, 10);
                if thread_rng().gen() {
                    *cell = Cell::Fixed(num);
                } else {
                    *cell = Cell::Variable(AtomicU8::new(0));
                }
            }
        }
        sudoku
    }

    pub fn iter(&self) -> impl '_ + Iterator<Item=&'_ Cell> {
        SudokuIter {
            inner: self.0.iter().flat_map(|i| i.into_iter())
        }
    }
}

impl Cell {
    pub fn read(&self) -> u8 {
        match self {
            Cell::Fixed(inner) => *inner,
            Cell::Variable(inner) => inner.load(Ordering::Relaxed)
        }
    }

    pub fn set(&mut self, new_value: u8) {
        if let Cell::Variable(inner) = self {
            inner.store(new_value, Ordering::Relaxed)
        }
    }
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        match self {
            Cell::Fixed(inner) => Cell::Fixed(*inner),
            Cell::Variable(inner) => {
                let num = inner.load(Ordering::Relaxed);
                Cell::Variable(AtomicU8::new(num))
            }
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Fixed(0)
    }
}

impl<'a, T: 'a + Iterator<Item=&'a Cell>> Iterator for SudokuIter<'a, T> {
    type Item = &'a Cell;

    fn next(&mut self) -> Option<&'a Cell> {
        self.inner.next()
    }
}
