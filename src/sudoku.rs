use std::ops;
use std::sync::atomic::{AtomicU8, Ordering};

use rand::prelude::*;
use rand::thread_rng;

pub const SUBREGION_COLUMNS: usize = 3;
pub const SUBREGION_ROWS: usize = 3;
pub const SUDOKU_MAX: usize = SUBREGION_COLUMNS * SUBREGION_ROWS;
pub const SUDOKU_MAX_U8: u8 = SUDOKU_MAX as u8;
pub const SUDOKU_AREA: usize = SUDOKU_MAX * SUDOKU_MAX;

#[derive(Clone, Debug, Default)]
pub struct Sudoku(pub [[Cell; SUDOKU_MAX]; SUDOKU_MAX]);

#[derive(Debug)]
pub enum Cell {
    Fixed(u8),
    Variable(AtomicU8),
}

fn get_subregion(index: (usize, usize)) -> usize {
    assert!(index.0 < SUDOKU_MAX && index.1 < SUDOKU_MAX);
    SUBREGION_ROWS * (index.0/SUBREGION_ROWS) + (index.1 / SUBREGION_COLUMNS)
}

pub fn is_valid_subregion<'a>(iter: impl Iterator<Item=&'a Cell>) -> bool {
    let mut vec: Vec<_> = iter.map(|c| c.read()).filter(|i| *i != 0).collect();
    vec.sort();
    let len = vec.len();
    vec.dedup();
    len == vec.len()
}

impl Sudoku {
    pub fn generate() -> Sudoku {
        // Create empty Sudoku
        let sudoku = Sudoku::default();
        // Create a vec of all possible to be filled
        let mut numbers = (1..(SUDOKU_MAX_U8+1)).collect::<Vec<u8>>();
        for (i, row) in sudoku.0.iter().enumerate() {
            // Shuffle the numbers once per row
            numbers.shuffle(&mut thread_rng());
            'cells: for (j, cell) in row.iter().enumerate() {
                for number in numbers.iter() {
                    cell.set(*number);
                    // Fast fail validation
                    if sudoku.validate_cell((i, j)) {
                        // Be sure we can still solve the puzzle
                        if sudoku.has_solution() {
                            continue 'cells;
                        }
                    }
                }
                // Since the puzzle never has no solution, cells loop will always continue
                unreachable!();
            }
        }
        sudoku
    }

    pub fn iter(&self) -> impl Iterator<Item=&'_ Cell> {
        self.0.iter().flatten()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=&'_ mut Cell> {
        self.0.iter_mut().flatten()
    }

    pub fn _fixeds_iter(&self) ->  impl Iterator<Item=&'_ Cell> {
        self.iter().filter(|cell| {
            match cell {
                Cell::Fixed(_) => true,
                Cell::Variable(_) => false,
            }
        })
    }

    pub fn _variables_iter(&self) -> impl Iterator<Item=&'_ Cell> {
        self.iter().filter(|cell| {
            match cell {
                Cell::Fixed(_) => false,
                Cell::Variable(_) => true,
            }
        })
    }

    pub fn row(&self, row: usize) -> impl Iterator<Item=&'_ Cell> {
        assert!(row < SUDOKU_MAX);
        self.0[row].iter()
    }

    pub fn column(&self, column: usize) -> impl Iterator<Item=&'_ Cell> {
        assert!(column < SUDOKU_MAX);
        self.0.iter().flat_map(move |row| row.iter().nth(column))
    }

    pub fn subregion(&self, subregion: usize) -> impl Iterator<Item=&'_ Cell> {
        assert!(subregion < SUDOKU_MAX);
        self.0.iter().skip(SUBREGION_ROWS * (subregion / SUBREGION_ROWS))
            .take(SUBREGION_ROWS)
            .flat_map(move |row| {
                row.iter().skip(SUBREGION_COLUMNS * (subregion % SUBREGION_ROWS))
                    .take(SUBREGION_COLUMNS)
            })
    }

    pub fn validate_cell(&self, index: (usize, usize)) -> bool {
        assert!(index.0 < SUDOKU_MAX && index.1 < SUDOKU_MAX);
        0 == self[index].read() ||
        is_valid_subregion(self.row(index.0)) &&
        is_valid_subregion(self.column(index.1)) &&
        is_valid_subregion(self.subregion(get_subregion(index)))
    }

    /// Verifies that sudoku has *at least* one solution.
    /// TODO: Change algorithm from brute force.
    pub fn has_solution(&self) -> bool {
        let mut sudoku = self.clone();
        sudoku.fix();
        let stack = sudoku.make_solve_stack();
        // If sudoku is complete, there is only 1 solution
        if 0 == stack.len() {return true;}
        
        // if not zero, there's a solution
        0 != sudoku.solve_inner(&stack, 0)
    }
    
    /// Verifies that sudoku has *exactly* one solution.
    /// TODO: Change algorithm from brute force.
    pub fn has_unique_solution(&self) -> bool {
        let mut sudoku = self.clone();
        // Set all variable cells to 0
        sudoku.iter_mut().for_each(|cell| cell.set(0));
        let stack = sudoku.make_solve_stack();
        // If sudoku is complete, there is only 1 solution
        if 0 == stack.len() {return true;}

        let cursor = sudoku.solve_inner(&stack, 0);
        // If this can't solve, there are no unique solutions
        if 0 == cursor {return false;}

        // back off from the end
        let cursor = cursor - 1;
        // Go beyond the previously found solution
        // If the number is too big, we set to 0 when we solve inner
        stack[cursor].1.fetch_add(1, Ordering::Relaxed);
        sudoku.solve_inner(&stack, cursor);

        // If true, there are no addtional solutions
        0 == stack[0].1.load(Ordering::Relaxed)
    }

    pub fn solve(&mut self) -> &mut Self {
        let stack = self.make_solve_stack();
        self.solve_inner(&stack, 0);
        self
    }

    fn make_solve_stack(&self) -> Vec<((usize, usize), &AtomicU8)> {
        self.iter().enumerate().filter_map(|(index, cell)| {
            if let Cell::Variable(inner) = cell {
                Some(((index / SUDOKU_MAX, index % SUDOKU_MAX), inner))
            } else {
                None
            }
        }).collect()
    }

    fn solve_inner(&self, stack: &[((usize, usize), &AtomicU8)], mut cursor: usize) -> usize {
        while cursor < stack.len() {
            let (index, cell) = stack[cursor];
            if SUDOKU_MAX_U8 > cell.fetch_add(1, Ordering::Relaxed) {
                if self.validate_cell(index) {
                    cursor += 1;
                }
            } else {
                cell.store(0, Ordering::Relaxed);
                if cursor > 0 {
                    cursor -= 1;
                } else {
                    break;
                }
            }
        }
        cursor
    }

    pub fn fix(&mut self) -> &mut Self {
        for cell in self.iter_mut() {
            let num = cell.read();
            if num != 0 {
                *cell = Cell::Fixed(num);
            }
        }

        self
    }

    /// Removes fixed cells from a sudoku while ensuring a unique solution
    /// Removes up to `num_to_remove` cells. Stops early if it runs out of cells it can remove.
    pub fn prune(&mut self, num_to_remove: usize) -> &mut Self {
        assert!(self.has_unique_solution());

        let mut indices = self.iter().enumerate().filter_map(|(index, cell)| {
            match cell {
                Cell::Fixed(_) => Some(index),
                Cell::Variable(_) => None,
            }
        }).collect::<Vec<_>>();

        indices.shuffle(&mut thread_rng());

        let mut num_removed = 0;
        for index in indices {
            if num_removed < num_to_remove {
                let num = self[index].read();
                self[index] = Cell::Variable(AtomicU8::new(0));
                if self.has_unique_solution() {
                    num_removed +=1;
                } else {
                    self[index] = Cell::Fixed(num);
                }
            }
        }

        self
    }
}

impl Cell {
    pub fn read(&self) -> u8 {
        match self {
            Cell::Fixed(inner) => *inner,
            Cell::Variable(inner) => inner.load(Ordering::Relaxed)
        }
    }

    pub fn set(&self, new_value: u8) {
        if let Cell::Variable(inner) = self {
            inner.store(new_value, Ordering::Relaxed)
        }
    }
}

impl ops::Index<usize> for Sudoku {
    type Output = Cell;

    fn index(&self, idx: usize) -> &Self::Output {
        assert!(idx < SUDOKU_MAX * SUDOKU_MAX);
        &self.0[idx / SUDOKU_MAX][idx % SUDOKU_MAX]
    }
}

impl ops::IndexMut<usize> for Sudoku {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        assert!(idx < SUDOKU_AREA);
        &mut self.0[idx / SUDOKU_MAX][idx % SUDOKU_MAX]
    }
}

impl ops::Index<(usize, usize)> for Sudoku {
    type Output = Cell;

    fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
        assert!(row < SUDOKU_MAX && column < SUDOKU_MAX);
        &self.0[row][column]
    }
}

impl ops::IndexMut<(usize, usize)> for Sudoku {
    fn index_mut(&mut self, (row, column): (usize, usize)) -> &mut Self::Output {
        assert!(row < SUDOKU_MAX && column < SUDOKU_MAX);
        &mut self.0[row][column]
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
        Cell::Variable(AtomicU8::new(0))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solver() {
        let mut sudoku = Sudoku::default();
        assert!(!sudoku.has_unique_solution());
        sudoku.solve();
        sudoku.fix();
        assert!(sudoku.has_unique_solution());
        sudoku.prune(50);
        assert!(sudoku.has_unique_solution())
    }
}
