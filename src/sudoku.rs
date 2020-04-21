use std::sync::atomic::{AtomicU8, Ordering};

pub const COLUMS: usize = 9;
pub const ROWS: usize = 9;

#[derive(Clone, Debug, Default)]
pub struct Sudoku(pub [[Cell; COLUMS]; ROWS]);

#[derive(Debug)]
pub enum Cell {
    Fixed(u8),
    Variable(AtomicU8),
}

pub struct SudokuIter<'a, T: 'a + Iterator<Item=&'a Cell>> {
    inner: T,
}

impl Sudoku {
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
