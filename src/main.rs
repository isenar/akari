use cursive::direction::Direction;
use cursive::event::EventResult;
use cursive::theme::{BaseColor, Color, ColorStyle};
use cursive::traits::Resizable;
use cursive::view::CannotFocus;
use cursive::views::{Button, Dialog, LinearLayout, Panel};
use cursive::{Cursive, Printer, Vec2};
use maplit::hashset;
use std::collections::HashSet;
use std::fmt::Formatter;

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Tile>>,
    bulbs: HashSet<(usize, usize)>,
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
            bulbs,
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

#[derive(Debug)]
enum Tile {
    Togglable(TogglableTile),
    Wall,
    Zero,
    One,
    Two,
    Three,
    Four,
}

impl Tile {
    pub const fn blank() -> Self {
        Self::Togglable(TogglableTile::empty())
    }
}

impl Tile {
    fn toggle(&mut self) -> ActionResult {
        if let Self::Togglable(togglable) = self {
            togglable.toggle()
        } else {
            ActionResult::Nothing
        }
    }
}

#[derive(Debug)]
struct TogglableTile {
    times_lit: u8,
    content: TileContent,
}

impl TogglableTile {
    const fn empty() -> Self {
        Self {
            times_lit: 0,
            content: TileContent::Nothing,
        }
    }

    fn toggle(&mut self) -> ActionResult {
        let (next, action) = match self.content {
            TileContent::Nothing => (TileContent::Bulb, ActionResult::BulbInserted),
            TileContent::Bulb => (TileContent::Cross, ActionResult::CrossInserted),
            TileContent::Cross => (TileContent::Nothing, ActionResult::TileCleared),
        };

        self.content = next;

        action
    }
}

enum ActionResult {
    BulbInserted,
    CrossInserted,
    TileCleared,
    Nothing,
}

#[derive(Debug)]
enum TileContent {
    Nothing,
    Bulb,
    Cross,
}

impl cursive::view::View for Grid {
    fn draw(&self, printer: &Printer) {
        for (x, row) in self.grid.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                let (color, text) = match tile {
                    Tile::Togglable(TogglableTile {
                        content: TileContent::Bulb,
                        ..
                    }) => (Color::Dark(BaseColor::Black), "B"),
                    Tile::Togglable(TogglableTile {
                        content: TileContent::Nothing,
                        ..
                    }) => (Color::Dark(BaseColor::White), " "),
                    Tile::Togglable(TogglableTile {
                        content: TileContent::Cross,
                        ..
                    }) => (Color::Dark(BaseColor::Red), "X"),
                    Tile::Wall => (Color::Dark(BaseColor::Black), " "),
                    Tile::Zero => (Color::Dark(BaseColor::White), "0"),
                    Tile::One => (Color::Dark(BaseColor::White), "1"),
                    Tile::Two => (Color::Dark(BaseColor::White), "2"),
                    Tile::Three => (Color::Dark(BaseColor::White), "3"),
                    Tile::Four => (Color::Dark(BaseColor::White), "4"),
                };

                let bg_color = match tile {
                    Tile::Togglable(TogglableTile { times_lit, .. }) => {
                        if *times_lit > 0 {
                            Color::Light(BaseColor::Yellow)
                        } else {
                            Color::Light(BaseColor::White)
                        }
                    }
                    Tile::Wall | Tile::Zero | Tile::One | Tile::Two | Tile::Three | Tile::Four => {
                        Color::Dark(BaseColor::Black)
                    }
                };

                printer.with_color(ColorStyle::new(color, bg_color), |printer| {
                    printer.print((x, y), text)
                });
            }
        }
    }
    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        let x = self.grid.len();
        let y = self.grid[0].len();

        (x, y).into()
    }

    fn take_focus(&mut self, _source: Direction) -> Result<EventResult, CannotFocus> {
        Ok(EventResult::Consumed(None))
    }
}

fn show_board(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Akari")
            .content(LinearLayout::horizontal().child(Panel::new(Grid::new_hardcoded())))
            .padding_lrtb(4, 5, 1, 1)
            .button("Quit game", |s| {
                s.pop_layer();
            }),
    );
}

fn main() {
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::new().title("Akari").content(
            LinearLayout::vertical()
                .child(Button::new_raw("  New game   ", show_board))
                .child(Button::new_raw("    Exit     ", Cursive::quit)),
        ),
    );
    siv.add_global_callback('q', Cursive::quit);

    siv.run();
}
