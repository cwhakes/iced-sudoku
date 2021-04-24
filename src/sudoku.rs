use std::ops;
use std::sync::atomic::{AtomicU8, Ordering};

use rand::prelude::*;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Serialize, Deserialize))]
pub struct Sudoku {
	subregion_columns: u8,
	subregion_rows: u8,
	grid: Vec<Vec<Cell>>,
}

#[derive(Debug)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Serialize, Deserialize))]
pub enum Cell {
	Fixed(u8),
	Variable(AtomicU8),
}

pub fn is_valid_subregion<'a>(value: u8, iter: impl Iterator<Item = &'a Cell>) -> bool {
	let count = iter.map(Cell::read).filter(|i| *i == value).count();
	count <= 1
}

impl Sudoku {
	fn new(subregion_rows: u8, subregion_columns: u8) -> Self {
		let length = subregion_rows as usize * subregion_columns as usize;
		assert!(length <= u8::MAX as usize);
		let grid = vec![vec![Cell::default(); length]; length];
		Sudoku {
			subregion_columns,
			subregion_rows,
			grid,
		}
	}

	pub fn generate_solved(subregion_rows: u8, subregion_columns: u8) -> Self {
		// Create empty Sudoku
		let sudoku = Sudoku::new(subregion_rows, subregion_columns);
		// Create a vec of all possible to be filled
		let mut numbers = (1..(sudoku.length_u8() + 1)).collect::<Vec<u8>>();
		for (i, row) in sudoku.grid.iter().enumerate() {
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

	pub fn generate_unsolved(subregion_rows: u8, subregion_columns: u8, prune: usize) -> Self {
		let mut sudoku = Sudoku::generate_solved(subregion_rows, subregion_columns);
		//sudoku.solve().fix().prune().solve();
		sudoku.fix().prune(prune);
		sudoku
	}

	pub fn subregion_columns(&self) -> usize {
		self.subregion_columns as usize
	}

	pub fn subregion_rows(&self) -> usize {
		self.subregion_rows as usize
	}

	pub fn length(&self) -> usize {
		self.length_u8() as usize
	}

	pub fn length_u8(&self) -> u8 {
		self.subregion_columns * self.subregion_rows
	}

	pub fn area(&self) -> usize {
		self.length().pow(2)
	}

	pub fn iter(&self) -> impl Iterator<Item = &'_ Cell> {
		self.grid.iter().flatten()
	}

	pub fn iter_mut(&mut self) -> impl Iterator<Item = &'_ mut Cell> {
		self.grid.iter_mut().flatten()
	}

	pub fn fixeds_iter(&self) -> impl Iterator<Item = &'_ Cell> {
		self.iter().filter(|cell| match cell {
			Cell::Fixed(_) => true,
			Cell::Variable(_) => false,
		})
	}

	pub fn variables_iter(&self) -> impl Iterator<Item = &'_ Cell> {
		self.iter().filter(|cell| match cell {
			Cell::Fixed(_) => false,
			Cell::Variable(_) => true,
		})
	}

	pub fn row(&self, row: usize) -> impl Iterator<Item = &'_ Cell> {
		assert!(row < self.length());
		self.grid[row].iter()
	}

	pub fn column(&self, column: usize) -> impl Iterator<Item = &'_ Cell> {
		assert!(column < self.length());
		self.grid.iter().flat_map(move |row| row.get(column))
	}

	fn get_subregion_index(&self, index: (usize, usize)) -> usize {
		assert!(index.0 < self.length() && index.1 < self.length());
		self.subregion_rows() * (index.0 / self.subregion_rows())
			+ (index.1 / self.subregion_columns())
	}

	pub fn subregion(&self, subregion: usize) -> impl Iterator<Item = &'_ Cell> {
		assert!(subregion < self.length());
		self.grid
			.iter()
			.skip(self.subregion_rows() * (subregion / self.subregion_rows()))
			.take(self.subregion_rows())
			.flat_map(move |row| {
				row.iter()
					.skip(self.subregion_columns() * (subregion % self.subregion_rows()))
					.take(self.subregion_columns())
			})
	}

	pub fn validate_cell(&self, index: (usize, usize)) -> bool {
		assert!(index.0 < self.length() && index.1 < self.length());
		let value = self[index].read();
		0 == value
			|| is_valid_subregion(value, self.row(index.0))
				&& is_valid_subregion(value, self.column(index.1))
				&& is_valid_subregion(value, self.subregion(self.get_subregion_index(index)))
	}

	/// Verifies that sudoku has *at least* one solution.
	/// TODO: Change algorithm from brute force.
	pub fn has_solution(&self) -> bool {
		let mut sudoku = self.clone();
		sudoku.fix();
		let stack = sudoku.make_solve_stack();
		// If sudoku is complete, there is only 1 solution
		if stack.is_empty() {
			return true;
		}

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
		if stack.is_empty() {
			return true;
		}

		let cursor = sudoku.solve_inner(&stack, 0);
		// If this can't solve, there are no unique solutions
		if 0 == cursor {
			return false;
		}

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
		self.iter()
			.enumerate()
			.filter_map(|(index, cell)| {
				if let Cell::Variable(inner) = cell {
					Some(((index / self.length(), index % self.length()), inner))
				} else {
					None
				}
			})
			.collect()
	}

	fn solve_inner(&self, stack: &[((usize, usize), &AtomicU8)], mut cursor: usize) -> usize {
		while cursor < stack.len() {
			let (index, cell) = stack[cursor];
			if self.length_u8() > cell.fetch_add(1, Ordering::Relaxed) {
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

		let mut indices = self
			.iter()
			.enumerate()
			.filter_map(|(index, cell)| match cell {
				Cell::Fixed(_) => Some(index),
				Cell::Variable(_) => None,
			})
			.collect::<Vec<_>>();

		indices.shuffle(&mut thread_rng());

		let mut num_removed = 0;
		for index in indices {
			if num_removed < num_to_remove {
				let num = self[index].read();
				self[index] = Cell::Variable(AtomicU8::new(0));
				if self.has_unique_solution() {
					num_removed += 1;
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
			Cell::Variable(inner) => inner.load(Ordering::Relaxed),
		}
	}

	pub fn text(&self) -> std::borrow::Cow<'static, str> {
		match self.read() {
			0 => "".into(),
			num => num.to_string().into(),
		}
	}

	pub fn set(&self, new_value: u8) {
		if let Cell::Variable(inner) = self {
			inner.store(new_value, Ordering::Relaxed)
		}
	}
}

impl Default for Sudoku {
	fn default() -> Sudoku {
		Sudoku::new(3, 3)
	}
}

impl ops::Index<usize> for Sudoku {
	type Output = Cell;

	fn index(&self, idx: usize) -> &Self::Output {
		assert!(idx < self.area());
		&self.grid[idx / self.length()][idx % self.length()]
	}
}

impl ops::IndexMut<usize> for Sudoku {
	fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
		assert!(idx < self.area());
		let length = self.length();
		&mut self.grid[idx / length][idx % length]
	}
}

impl ops::Index<(usize, usize)> for Sudoku {
	type Output = Cell;

	fn index(&self, (row, column): (usize, usize)) -> &Self::Output {
		assert!(row < self.length() && column < self.length());
		&self.grid[row][column]
	}
}

impl ops::IndexMut<(usize, usize)> for Sudoku {
	fn index_mut(&mut self, (row, column): (usize, usize)) -> &mut Self::Output {
		assert!(row < self.length() && column < self.length());
		&mut self.grid[row][column]
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

	#[test]
	fn tiny() {
		let sudoku = Sudoku::generate_unsolved(1, 2, 50);
		assert_eq!(1, sudoku.fixeds_iter().count());
		assert_eq!(3, sudoku.variables_iter().count());
	}
}
