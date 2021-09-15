use crate::direction::{E, S};
use std::fmt;
use std::iter;
use std::iter::FromIterator;

pub struct Maze {
    pub grid: Vec<Vec<u8>>,
    pub size: (u64, u64),
}

impl Maze {
    pub fn new(width: u64, height: u64) -> Self {
        let grid = vec![vec![0; width as usize]; height as usize];
        Self {
            grid,
            size: (width, height),
        }
    }

    fn sz(&self) -> (usize, usize) {
        (self.size.0 as usize, self.size.1 as usize)
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sz = self.sz();

        let s = String::from_iter(iter::repeat('_').take(sz.0 * 2 + 1));
        writeln!(f, "{}", s)?;

        for (y, row) in self.grid.iter().enumerate() {
            write!(f, "|")?;
            for (x, &cell) in row.iter().enumerate() {
                if cell == 0 && (y + 1) < self.grid.len() && self.grid[y + 1][x] == 0 {
                    write!(f, " ")?;
                } else {
                    if cell & S != 0 {
                        write!(f, " ")?;
                    } else {
                        write!(f, "_")?;
                    }
                }

                if cell == 0 && (x + 1) < row.len() && row[x + 1] == 0 {
                    if (y + 1) < self.grid.len()
                        && (self.grid[y + 1][x] == 0 || self.grid[y + 1][x + 1] == 0)
                    {
                        write!(f, " ")?;
                    } else {
                        write!(f, "_")?;
                    }
                } else if cell & E != 0 {
                    if (cell | row[x + 1]) & S != 0 {
                        write!(f, " ")?;
                    } else {
                        write!(f, "_")?;
                    }
                } else {
                    write!(f, "|")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
