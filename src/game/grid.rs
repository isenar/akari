use crate::game::tile::{ActionResult, Tile};
use maplit::hashset;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid {
    pub grid: Vec<Vec<Tile>>,
    _solution_bulbs: HashSet<(usize, usize)>,
}

impl Grid {
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
        let action = self.grid[row][col].toggle();
        if let ActionResult::Nothing = action {
            return;
        }

        let Tile::Togglable(after)  = &mut self.grid[row][col] else {
            unreachable!("Inconsistent before -> after state")
        };

        match action {
            // empty -> bulb
            ActionResult::BulbInserted => {
                after.times_lit += 1;

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

    fn affected_neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
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
        debug_assert!(row < self.grid.len());
        debug_assert!(Some(col) < self.grid.first().map(|col| col.len()));

        let columns = self.grid[row].len();

        let to_left = self.grid[row]
            .iter()
            .enumerate()
            .rev()
            .skip(columns - col)
            .take_while(|(_, tile)| matches!(tile, Tile::Togglable(_)))
            .map(move |(i, _)| (row, i));

        let to_right = self.grid[row]
            .iter()
            .enumerate()
            .skip(col + 1)
            .take_while(|(_, tile)| matches!(tile, Tile::Togglable(_)))
            .map(move |(i, _)| (row, i));

        to_left.chain(to_right)
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

        assert_grid(&expected, &grid);
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

        assert_grid(&expected, &grid);
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

        assert_grid(&expected, &grid);
    }
}
