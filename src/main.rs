use crate::game::grid::Grid;
use crate::game::tile::{Tile, TileContent, TogglableTile};
use cursive::direction::Direction;
use cursive::event::{Event, EventResult};
use cursive::theme::{BaseColor, Color, ColorStyle};
use cursive::view::CannotFocus;
use cursive::views::{Button, Dialog, LinearLayout, Panel};
use cursive::{Cursive, Printer, Vec2};

mod game;

const WHITE: Color = Color::Light(BaseColor::White);
const BLACK: Color = Color::Dark(BaseColor::Black);
const RED: Color = Color::Dark(BaseColor::Red);
const YELLOW: Color = Color::Light(BaseColor::Yellow);

fn color_style(tile: &Tile) -> ColorStyle {
    let (font_color, bg_color) = match tile {
        Tile::Togglable(TogglableTile { times_lit, content }) => {
            let font_color = match content {
                TileContent::Nothing => WHITE,
                TileContent::Bulb => BLACK,
                TileContent::Cross => RED,
            };
            let bg_color = if *times_lit > 0 { YELLOW } else { WHITE };

            (font_color, bg_color)
        }
        Tile::Wall | Tile::Number(_) => (WHITE, BLACK),
    };

    ColorStyle::new(font_color, bg_color)
}

impl cursive::view::View for Grid {
    fn draw(&self, printer: &Printer) {
        for (x, row) in self.grid.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                let color_style = color_style(tile);

                printer.with_color(color_style, |printer| printer.print((x, y), &tile.symbol()));
            }
        }
    }
    fn required_size(&mut self, _constraint: Vec2) -> Vec2 {
        let x = self.grid.len();
        let y = self.grid[0].len();

        (x, y).into()
    }

    fn on_event(&mut self, _: Event) -> EventResult {
        EventResult::Ignored
    }

    fn take_focus(&mut self, _source: Direction) -> Result<EventResult, CannotFocus> {
        Ok(EventResult::Consumed(None))
    }
}

fn show_board(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::new()
            .title("Akari")
            .content(LinearLayout::vertical().child(Panel::new(Grid::new_hardcoded())))
            // .padding_lrtb(4, 5, 1, 1) // TODO
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
