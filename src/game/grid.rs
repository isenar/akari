use crate::game::tile::{ActionResult, Tile, TileContent, TogglableTile};
use maplit::hashset;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Grid {
    pub grid: Vec<Vec<Tile>>,
    _bulbs: HashSet<(usize, usize)>,
}

impl Grid {
    // hardoded 5x5 from BrainBashers daily (17/10/22 - easy)
    pub fn new_hardcoded() -> Self {
        let bulbs = hashset! {
            (0, 1), (0, 3),
            (1, 2), (1, 4),
            (2, 3),
            (3, 1),
            (4, 0),
        };

        Self {
            _bulbs: bulbs,
            grid: vec![
                vec![
                    Tile::blank(),
                    Tile::Togglable(TogglableTile {
                        times_lit: 1,
                        content: TileContent::Bulb,
                    }), // bulb
                    Tile::Three,
                    Tile::blank(), // bulb
                    Tile::blank(),
                ],
                vec![
                    Tile::blank(),
                    Tile::blank(),
                    Tile::blank(), // bulb
                    Tile::Four,
                    Tile::blank(), // bulb
                ],
                vec![
                    Tile::Wall,
                    Tile::One,
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

    pub fn toggle(&mut self, x: usize, y: usize) {
        let action = self.grid[x][y].toggle();

        match action {
            ActionResult::BulbInserted => {}
            ActionResult::CrossInserted => {}
            ActionResult::TileCleared => {}
            ActionResult::Nothing => {}
        }

        // if action == lit -> light all remain

        // else if action == unlit?
    }
}
