use crate::game::tile::{ActionResult, Tile};
use maplit::hashset;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid {
    pub grid: Vec<Vec<Tile>>,
    _solution_bulbs: HashSet<(usize, usize)>,
}

impl Grid {
    pub fn new_empty_3x3() -> Self {
        Self {
            grid: vec![vec![Tile::blank(); 3]; 3],
            _solution_bulbs: Default::default(),
        }
    }

    // hardoded 5x5 from BrainBashers daily (17/10/22 - easy)
    pub fn new_hardcoded() -> Self {
        let _solution_bulbs = hashset! {
            (0, 1), (0, 3),
            (1, 2), (1, 4),
            (2, 3),
            (3, 1),
            (4, 0),
        };

        Self {
            _solution_bulbs,
            grid: vec![
                vec![
                    Tile::blank(),
                    Tile::blank(),
                    Tile::Number(3),
                    Tile::blank(), // bulb
                    Tile::blank(),
                ],
                vec![
                    Tile::blank(),
                    Tile::blank(),
                    Tile::blank(), // bulb
                    Tile::Number(4),
                    Tile::blank(), // bulb
                ],
                vec![
                    Tile::Wall,
                    Tile::Number(1),
                    Tile::blank(),
                    Tile::blank(), // bulb
                    Tile::blank(),
                ],
                vec![
                    Tile::blank(),
                    Tile::blank(), // bulb
                    Tile::blank(),
                    Tile::blank(),
                    Tile::blank(),
                ],
                vec![
                    Tile::blank(), // bulb
                    Tile::blank(),
                    Tile::blank(),
                    Tile::Wall,
                    Tile::blank(),
                ],
            ],
        }
    }

    pub fn toggle(&mut self, row: usize, col: usize) {
        println!("COLS: {:?}", self.grid.first().map(|c| c.len()));
        println!("ROWS: {:?}", self.grid.len());

        let action = self.grid[row][col].toggle();
        if let ActionResult::Nothing = action {
            return;
        }

        let Tile::Togglable(after)  = &mut self.grid[row][col]
         else {
            unreachable!("Inconsistent before -> after state")
        };

        match action {
            // empty -> bulb
            ActionResult::BulbInserted => {
                after.times_lit += 1;

                let horizontal = self.horizontal_neighbours(row, col);
                println!("Horizontal: {horizontal:?}");

                let vertical = self.vertical_neighbours(row, col);
                println!("Vertical: {vertical:?}");

                for (r, c) in self.affected_neighbours(row, col) {
                    self.grid[r][c].light_up();

                }

            }
            // bulb -> cross
            ActionResult::CrossInserted => {
                after.times_lit -= 1;

                for (r, c) in self.affected_neighbours(row, col) {
                    self.grid[r][c].light_down();
                }
            }
            // cross -> empty
            ActionResult::TileCleared |
            // non-interactive tile
            ActionResult::Nothing => {}
        }
    }

    fn affected_neighbours(&self, row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
        self.horizontal_neighbours(row, col)
            .into_iter()
            .chain(self.vertical_neighbours(row, col))
    }

    // TODO: rewrite this crap
    fn horizontal_neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        dbg!(row, col);
        debug_assert!(row < self.grid.len());
        debug_assert!(Some(col) < self.grid.first().map(|col| col.len()));

        let mut neighbours = Vec::new();

        for i in (col + 1)..self.grid[row].len() {
            let tile = &self.grid[row][i];

            match tile {
                Tile::Togglable(_) => neighbours.push((row, i)),
                _ => break,
            }
        }

        for i in (0..col).rev() {
            let tile = &self.grid[row][i];
            match tile {
                Tile::Togglable(_) => neighbours.push((row, i)),
                _ => break,
            }
        }

        neighbours
    }

    fn vertical_neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();

        for i in (row + 1)..self.grid.len() {
            let tile = &self.grid[i][col];
            match tile {
                Tile::Togglable(_) => neighbours.push((i, col)),
                Tile::Wall | Tile::Number(_) => break,
            }
        }

        for i in (0..row).rev() {
            let tile = &self.grid[i][col];
            match tile {
                Tile::Togglable(_) => neighbours.push((i, col)),
                Tile::Wall | Tile::Number(_) => break,
            }
        }

        neighbours
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_each_tile(expected: &Grid, actual: &Grid) {
        let expected = &expected.grid;
        let actual = &actual.grid;

        assert_eq!(expected.len(), actual.len());
        assert_eq!(expected[0].len(), actual[0].len());

        for row in 0..expected.len() {
            for col in 0..expected.len() {
                assert_eq!(
                    expected[row][col], actual[row][col],
                    "Tile mismatch at [{row}][{col}]"
                );
            }
        }
    }

    #[test]
    fn inserting_lightbulb_lights_neighbours_in_line() {
        let mut grid = Grid {
            grid: vec![vec![Tile::blank(); 3]; 3],
            _solution_bulbs: Default::default(),
        };

        let expected = Grid {
            grid: vec![
                vec![Tile::blank(), Tile::lit_empty(1), Tile::blank()],
                vec![Tile::lit_empty(1), Tile::bulb(1), Tile::lit_empty(1)],
                vec![Tile::blank(), Tile::lit_empty(1), Tile::blank()],
            ],
            _solution_bulbs: Default::default(),
        };

        grid.toggle(1, 1);

        for x in 0..grid.grid.len() {
            for y in 0..grid.grid[0].len() {
                assert_eq!(
                    expected.grid[x][y], grid.grid[x][y],
                    "Tile mismatch at [{x}][{y}]"
                );
            }
        }
    }

    #[test]
    fn lights_in_two_corners() {
        let mut grid = Grid {
            grid: vec![vec![Tile::blank(); 3]; 3],
            _solution_bulbs: Default::default(),
        };
        let expected = Grid {
            grid: vec![
                vec![Tile::bulb(1), Tile::lit_empty(1), Tile::lit_empty(2)],
                vec![Tile::lit_empty(1), Tile::blank(), Tile::lit_empty(1)],
                vec![Tile::lit_empty(2), Tile::lit_empty(1), Tile::bulb(1)],
            ],
            _solution_bulbs: Default::default(),
        };

        grid.toggle(0, 0);
        grid.toggle(2, 2);

        for x in 0..grid.grid.len() {
            for y in 0..grid.grid[0].len() {
                assert_eq!(
                    expected.grid[x][y], grid.grid[x][y],
                    "Tile mismatch at [{x}][{y}]"
                );
            }
        }
    }

    #[test]
    fn lights_shining_at_each_other_in_all_corners() {
        let mut grid = Grid {
            grid: vec![vec![Tile::blank(); 3]; 4],
            _solution_bulbs: Default::default(),
        };
        let expected = Grid {
            grid: vec![
                vec![Tile::bulb(3), Tile::lit_empty(2), Tile::bulb(3)],
                vec![Tile::lit_empty(2), Tile::lit_empty(0), Tile::lit_empty(2)],
                vec![Tile::lit_empty(2), Tile::lit_empty(0), Tile::lit_empty(2)],
                vec![Tile::bulb(3), Tile::lit_empty(2), Tile::bulb(3)],
            ],
            _solution_bulbs: Default::default(),
        };

        grid.toggle(0, 0);
        grid.toggle(0, 2);
        grid.toggle(3, 0);
        grid.toggle(3, 2);

        for x in 0..grid.grid.len() {
            for y in 0..grid.grid[0].len() {
                assert_eq!(
                    expected.grid[x][y], grid.grid[x][y],
                    "Tile mismatch at [{x}][{y}]"
                );
            }
        }
    }
}
