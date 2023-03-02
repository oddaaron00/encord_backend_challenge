pub struct Grid {
    grid: Vec<Vec<Option<()>>>,
}

// impl fmt::Debug for Grid {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let mut string = String::new();
//
//         let grid = self.get_grid();
//         for i in (0..grid.len()).rev() {
//             let mut row_string = String::new();
//             for j in &grid[i] {
//                 row_string.push_str(if j.is_some() { "O " } else { ". " });
//             }
//             string.push_str(format!("\n{}", row_string).as_str())
//         }
//         write!(f, "{}", string)
//     }
// }

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl Grid {
    pub fn new() -> Self {
        Self {
            // I didn't have time to refactor this into a 1-D array
            grid: vec![vec![None; 10]; 100],
        }
    }

    pub fn set_point(&mut self, x: usize, y: usize) {
        self.grid[y][x] = Some(());
    }

    pub fn remove_line(&mut self, i: usize) {
        self.grid.remove(i);
    }

    pub fn get_grid(&self) -> &[Vec<Option<()>>] {
        &self.grid
    }

    pub fn get_grid_row(&self, index: usize) -> &[Option<()>] {
        &self.grid[index]
    }

    pub fn get_height(&self) -> usize {
        for i in (0..self.grid.len()).rev() {
            if self.get_grid_row(i).iter().any(|cell| cell.is_some()) {
                return i;
            }
        }
        0
    }
}
