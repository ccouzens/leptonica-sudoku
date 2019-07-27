extern crate leptonica_sys;
use leptonica_sys::{l_int32, sudokuCreate, sudokuDestroy, sudokuGenerate, sudokuSolve, L_SUDOKU};

/// Safe wrapper around Leptonica's L_SUDOKU struct.
pub struct Sudoku(*mut L_SUDOKU);

impl Drop for Sudoku {
    fn drop(&mut self) {
        unsafe {
            sudokuDestroy(&mut self.0 as *mut *mut L_SUDOKU);
        }
    }
}

impl Sudoku {
    /// Create a new Sudoku object from a flat array of digits.
    /// Use 0 to indicate a square is blank.
    pub fn new(raw_array: &[l_int32; 81]) -> Self {
        unsafe {
            let mut_ptr = (raw_array as *const l_int32) as *mut l_int32;
            Sudoku(sudokuCreate(mut_ptr))
        }
    }

    /// Get the initial grid as a flat array of digits.
    /// 0 represents the square is blank.
    pub fn initial_state(&self) -> &[l_int32; 81] {
        unsafe { &*((*self.0).init as *const [l_int32; 81]) }
    }

    /// Get the current state as a flat array of digits.
    /// If solve has been called, this will be the solution.
    pub fn state(&self) -> &[l_int32; 81] {
        unsafe { &*((*self.0).state as *const [l_int32; 81]) }
    }

    /// Solve the puzzle.
    /// If it can't be solved, it returns false and prints a message to stderr.
    pub fn solve(&mut self) -> bool {
        unsafe { sudokuSolve(self.0) == 1 }
    }

    /// Generate a new Sudoku puzzle.
    /// 
    /// Returns `Some(Sudoku)` on success and `None` on failure.
    /// Note that this will print messages to stderr on success and failure.
    /// 
    /// This generator works backwards. It starts with a solved sudoku and
    /// adds in blanks to create a puzzle.
    pub fn generate(
        mut complete_grid: [l_int32; 81],
        seed: l_int32,
        min_elements: l_int32,
        max_tries: l_int32,
    ) -> Option<Sudoku> {
        let sudoku_ptr =
            unsafe { sudokuGenerate(complete_grid.as_mut_ptr(), seed, min_elements, max_tries) };
        if sudoku_ptr.is_null() {
            None
        } else {
            Some(Sudoku(sudoku_ptr))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Sudoku;
    use leptonica_sys::l_int32;

    const PUZZLE_UNSOLVED: [l_int32; 81] = include!("test_sudoku.in");
    const PUZZLE_SOLVED: [l_int32; 81] = include!("solved_sudoku.in");

    #[test]
    fn new_works() {
        let sudoku = Sudoku::new(&PUZZLE_UNSOLVED);
        assert_eq!(PUZZLE_UNSOLVED[..], sudoku.initial_state()[..]);
    }

    #[test]
    fn solve_works() {
        let mut sudoku = Sudoku::new(&PUZZLE_UNSOLVED);
        assert!(sudoku.solve());
        assert_eq!(PUZZLE_UNSOLVED[..], sudoku.initial_state()[..]);
        assert_eq!(PUZZLE_SOLVED[..], sudoku.state()[..]);
    }

    #[test]
    fn generate_works() {
        let sudoku = Sudoku::generate(PUZZLE_SOLVED, 0, 50, 5).unwrap();
        assert_ne!(sudoku.initial_state()[..], PUZZLE_SOLVED[..]);
        assert_eq!(sudoku.state()[..], PUZZLE_SOLVED[..]);
        let blanks = sudoku.initial_state().iter().filter(|&&s| s == 0).count();
        assert!(blanks <= 81 - 17);
        assert!(blanks >= 30);
    }
}
