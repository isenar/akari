use crate::game::tile::{BulbActionResult, Tile};
use crate::Vec2;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid {
    pub size: Vec2,
    pub grid: Vec<Vec<Tile>>,
}

impl Grid {
    // hardoded 5x5 from BrainBashers daily (17/10/22 - easy)
    pub fn new_hardcoded() -> Self {
        // Solution:
        //     (0, 1), (0, 3),
        //     (1, 2), (1, 4),
        //     (2, 3),
        //     (3, 1),
        //     (4, 0),

        Self {
            size: Vec2::new(5, 5),
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
        let action = self.grid[row][col].toggle();
        self.handle_toggle(row, col, action);
    }

    pub fn toggle_back(&mut self, row: usize, col: usize) {
        let action = self.grid[row][col].toggle_back();

        self.handle_toggle(row, col, action);
    }

    fn handle_toggle(&mut self, row: usize, col: usize, action: BulbActionResult) {
        if let BulbActionResult::Nothing = action {
            return;
        }

        let Tile::Togglable(after)  = &mut self.grid[row][col] else {
            unreachable!("Inconsistent before -> after state")
        };

        match action {
            BulbActionResult::BulbInserted => {
                after.light_level += 1;

                for (r, c) in self.affected_neighbours(row, col) {
                    self.grid[r][c].increase_light_level();
                }
            }
            BulbActionResult::BulbRemoved => {
                after.light_level -= 1;

                for (r, c) in self.affected_neighbours(row, col) {
                    self.grid[r][c].decrease_light_level();
                }
            }
            _ => {}
        }
    }

    fn affected_neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        debug_assert!(row < self.grid.len());
        debug_assert!(Some(col) < self.grid.first().map(|col| col.len()));

        self.horizontal_neighbours(row, col)
            .into_iter()
            .chain(self.vertical_neighbours(row, col))
            .collect()
    }

    fn horizontal_neighbours(
        &self,
        row: usize,
        col: usize,
    ) -> impl IntoIterator<Item = (usize, usize)> + '_ {
        let columns = self.grid[row].len();
        let to_left = self.grid[row].iter().enumerate().rev().skip(columns - col);
        let to_right = self.grid[row].iter().enumerate().skip(col + 1);

        to_left
            .chain(to_right)
            .take_while(|(_, tile)| matches!(tile, Tile::Togglable(_)))
            .map(move |(i, _)| (row, i))
    }

    fn vertical_neighbours(
        &self,
        row: usize,
        col: usize,
    ) -> impl IntoIterator<Item = (usize, usize)> + '_ {
        let above = self
            .grid
            .iter()
            .enumerate()
            .rev()
            .skip(self.grid.len() - row)
            .take_while(move |(_, tile)| matches!(tile[col], Tile::Togglable(_)))
            .map(move |(i, _)| (i, col));

        let below = self
            .grid
            .iter()
            .enumerate()
            .skip(row + 1)
            .take_while(move |(_, tile)| matches!(tile[col], Tile::Togglable(_)))
            .map(move |(i, _)| (i, col));

        above.chain(below)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_grid(expected: &Grid, actual: &Grid) {
        let expected = &expected.grid;
        let actual = &actual.grid;

        assert_eq!(expected.len(), actual.len(), "Rows mismatch");
        assert_eq!(expected[0].len(), actual[0].len(), "Columns mismatch");

        for row in 0..expected.len() {
            for col in 0..expected[0].len() {
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
            size: Vec2::new(3, 3),
            grid: vec![vec![Tile::blank(); 3]; 3],
        };

        let expected = Grid {
            size: Vec2::new(3, 3),

            grid: vec![
                vec![Tile::blank(), Tile::lit_empty(1), Tile::blank()],
                vec![Tile::lit_empty(1), Tile::bulb(1), Tile::lit_empty(1)],
                vec![Tile::blank(), Tile::lit_empty(1), Tile::blank()],
            ],
        };

        grid.toggle(1, 1);

        assert_grid(&expected, &grid);
    }

    #[test]
    fn lights_in_two_corners() {
        let mut grid = Grid {
            size: Vec2::new(3, 3),
            grid: vec![vec![Tile::blank(); 3]; 3],
        };
        let expected = Grid {
            size: Vec2::new(3, 3),
            grid: vec![
                vec![Tile::bulb(1), Tile::lit_empty(1), Tile::lit_empty(2)],
                vec![Tile::lit_empty(1), Tile::blank(), Tile::lit_empty(1)],
                vec![Tile::lit_empty(2), Tile::lit_empty(1), Tile::bulb(1)],
            ],
        };

        grid.toggle(0, 0);
        grid.toggle(2, 2);

        assert_grid(&expected, &grid);
    }

    #[test]
    fn lights_shining_at_each_other_in_all_corners() {
        let mut grid = Grid {
            size: Vec2::new(4, 3),
            grid: vec![vec![Tile::blank(); 3]; 4],
        };
        let expected = Grid {
            size: Vec2::new(4, 3),
            grid: vec![
                vec![Tile::bulb(3), Tile::lit_empty(2), Tile::bulb(3)],
                vec![Tile::lit_empty(2), Tile::lit_empty(0), Tile::lit_empty(2)],
                vec![Tile::lit_empty(2), Tile::lit_empty(0), Tile::lit_empty(2)],
                vec![Tile::bulb(3), Tile::lit_empty(2), Tile::bulb(3)],
            ],
        };

        grid.toggle(0, 0);
        grid.toggle(0, 2);
        grid.toggle(3, 0);
        grid.toggle(3, 2);

        assert_grid(&expected, &grid);
    }
}
