use crate::game::grid::Grid;
use crate::game::tile::{Tile, TileContent, TogglableTile};
use cursive::direction::Direction;
use cursive::event::EventResult;
use cursive::theme::{BaseColor, Color, ColorStyle};
use cursive::view::CannotFocus;
use cursive::views::{Button, Dialog, LinearLayout, Panel};
use cursive::{Cursive, Printer, Vec2};

mod game;

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
