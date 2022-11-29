use crate::game::tile::{BulbAction, Tile, Wall};
use crate::Vec2;
use log::debug;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grid {
    pub grid: Vec<Vec<Tile>>,
    pub solution: Vec<(usize, usize)>,
}

impl Grid {
    // hardcoded 5x5 from BrainBashers daily (17/10/22 - easy) for testing
    pub fn new_hardcoded() -> Self {
        Self {
            grid: vec![
                vec![
                    Tile::blank(),
                    Tile::blank(),
                    Tile::Wall(Wall::Three),
                    Tile::blank(), // bulb
                    Tile::blank(),
                ],
                vec![
                    Tile::blank(),
                    Tile::blank(),
                    Tile::blank(), // bulb
                    Tile::Wall(Wall::Four),
                    Tile::blank(), // bulb
                ],
                vec![
                    Tile::Wall(Wall::Clear),
                    Tile::Wall(Wall::One),
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
                    Tile::wall(),
                    Tile::blank(),
                ],
            ],
            // solution: vec![(0, 1), (0, 3), (1, 2), (1, 4), (2, 3), (3, 1), (4, 0)],
            solution: vec![(0, 4), (1, 0), (1, 3), (2, 1), (3, 0), (3, 2), (4, 1)],
        }
    }

    pub fn size(&self) -> Vec2 {
        Vec2::new(self.grid[0].len(), self.grid.len())
    }

    pub fn toggle(&mut self, row: usize, col: usize) {
        debug!("Attempting to toggle [{row}][{col}]");

        let action = self.grid[row][col].toggle();
        self.handle_toggle(row, col, action);
    }

    pub fn toggle_back(&mut self, row: usize, col: usize) {
        let action = self.grid[row][col].toggle_back();

        self.handle_toggle(row, col, action);
    }

    fn handle_toggle(&mut self, row: usize, col: usize, action: BulbAction) {
        if let BulbAction::Nothing = action {
            debug!("Got 'nothing', skipped handling a toggle");
            return;
        }
        let affected = self.affected_neighbours(row, col);
        debug!("Affected neighbours of [{row}][{col}]: {affected:?}");

        let Tile::Togglable(after)  = &mut self.grid[row][col] else {
            unreachable!("Inconsistent before -> after state")
        };

        debug!("Action: {action:?}");

        match action {
            BulbAction::Inserted => {
                after.light_level += 1;

                for (r, c) in affected {
                    self.grid[r][c].increase_light_level();
                }
            }
            BulbAction::Removed => {
                after.light_level -= 1;

                for (r, c) in affected {
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
        let on_left = self.grid[row]
            .iter()
            .enumerate()
            .rev()
            .skip(columns - col)
            .map_while(move |(col, tile)| togglable_pos2(row, col, tile));
        let on_right = self.grid[row]
            .iter()
            .enumerate()
            .skip(col + 1)
            .map_while(move |(col, tile)| togglable_pos2(row, col, tile));

        on_left.chain(on_right)
    }

    fn vertical_neighbours(
        &self,
        row: usize,
        col: usize,
    ) -> impl IntoIterator<Item = (usize, usize)> + '_ {
        let rows = self.grid.len();
        let above = self
            .grid
            .iter()
            .enumerate()
            .rev()
            .skip(rows - row)
            .map_while(move |(row, row_tiles)| togglable_pos(row, col, row_tiles));

        let below = self
            .grid
            .iter()
            .enumerate()
            .skip(row + 1)
            .map_while(move |(row, row_tiles)| togglable_pos(row, col, row_tiles));

        above.chain(below)
    }
}

fn togglable_pos2(row: usize, col: usize, tile: &Tile) -> Option<(usize, usize)> {
    matches!(tile, Tile::Togglable(_)).then(|| (row, col))
}

fn togglable_pos(row: usize, col: usize, row_tiles: &[Tile]) -> Option<(usize, usize)> {
    matches!(row_tiles.get(col), Some(Tile::Togglable(_))).then(|| (row, col))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_grid(tiles: Vec<Vec<Tile>>) -> Grid {
        Grid {
            grid: tiles,
            solution: vec![],
        }
    }

    fn assert_grid(expected: &Grid, actual: &Grid) {
        let expected = &expected.grid;
        let actual = &actual.grid;

        assert_eq!(expected.len(), actual.len(), "Rows mismatch");
        assert_eq!(expected[0].len(), actual[0].len(), "Columns mismatch");

        let expected = expected.iter().flatten();
        let actual = actual.iter().flatten();
        let pairs = expected.zip(actual).enumerate();

        for (n, (expected_tile, actual_tile)) in pairs {
            assert_eq!(expected_tile, actual_tile, "Tile mismatch at [{n}]");
        }
    }

    #[test]
    fn inserting_lightbulb_lights_neighbours_in_line() {
        let mut grid = test_grid(vec![vec![Tile::blank(); 3]; 3]);

        let expected = test_grid(vec![
            vec![Tile::blank(), Tile::lit_empty(1), Tile::blank()],
            vec![Tile::lit_empty(1), Tile::bulb(1), Tile::lit_empty(1)],
            vec![Tile::blank(), Tile::lit_empty(1), Tile::blank()],
        ]);

        grid.toggle(1, 1);

        assert_grid(&expected, &grid);
    }

    #[test]
    fn lights_in_two_corners() {
        let mut grid = test_grid(vec![vec![Tile::blank(); 3]; 3]);
        let expected = test_grid(vec![
            vec![Tile::bulb(1), Tile::lit_empty(1), Tile::lit_empty(2)],
            vec![Tile::lit_empty(1), Tile::blank(), Tile::lit_empty(1)],
            vec![Tile::lit_empty(2), Tile::lit_empty(1), Tile::bulb(1)],
        ]);

        grid.toggle(0, 0);
        grid.toggle(2, 2);

        assert_grid(&expected, &grid);
    }

    #[test]
    fn lights_shining_at_each_other_in_all_corners() {
        let mut grid = test_grid(vec![vec![Tile::blank(); 3]; 4]);
        let expected = test_grid(vec![
            vec![Tile::bulb(3), Tile::lit_empty(2), Tile::bulb(3)],
            vec![Tile::lit_empty(2), Tile::lit_empty(0), Tile::lit_empty(2)],
            vec![Tile::lit_empty(2), Tile::lit_empty(0), Tile::lit_empty(2)],
            vec![Tile::bulb(3), Tile::lit_empty(2), Tile::bulb(3)],
        ]);

        grid.toggle(0, 0);
        grid.toggle(0, 2);
        grid.toggle(3, 0);
        grid.toggle(3, 2);

        assert_grid(&expected, &grid);
    }

    #[test]
    fn light_is_blocked_by_walls_minimal() {
        let mut grid = test_grid(vec![vec![
            Tile::blank(),
            Tile::wall(),
            Tile::blank(),
            Tile::blank(),
            Tile::wall(),
            Tile::blank(),
        ]]);
        let expected = test_grid(vec![vec![
            Tile::blank(),
            Tile::wall(),
            Tile::bulb(1),
            Tile::lit_empty(1),
            Tile::wall(),
            Tile::blank(),
        ]]);

        grid.toggle(0, 2);

        assert_grid(&expected, &grid);
    }
}
