use cursive::direction::Direction;
use cursive::event::{Event, EventResult, MouseButton, MouseEvent};
use cursive::theme::{BaseColor, Color, ColorStyle};
use cursive::view::CannotFocus;
use cursive::views::{Button, Dialog, LinearLayout, Panel};
use cursive::{Cursive, Printer, Vec2};
use log::info;

use crate::game::grid::Grid;
use crate::game::tile::{Tile, TileContent, TogglableTile, Wall};

mod game;

const WHITE: Color = Color::Light(BaseColor::White);
const BLACK: Color = Color::Dark(BaseColor::Black);
const RED: Color = Color::Dark(BaseColor::Red);
const YELLOW: Color = Color::Light(BaseColor::Yellow);

fn tile_symbol(tile: &Tile) -> &str {
    match tile {
        Tile::Togglable(TogglableTile { content, .. }) => match content {
            TileContent::Nothing => "  ",
            TileContent::Bulb => "💡",
            TileContent::Cross => "❌",
        },
        Tile::Wall(Wall::Clear) => "  ",
        Tile::Wall(Wall::Zero) => " 0",
        Tile::Wall(Wall::One) => " 1",
        Tile::Wall(Wall::Two) => " 2",
        Tile::Wall(Wall::Three) => " 3",
        Tile::Wall(Wall::Four) => " 4",
    }
}

fn color_style(tile: &Tile) -> ColorStyle {
    let (font_color, bg_color) = match tile {
        Tile::Togglable(TogglableTile {
            light_level: times_lit,
            content,
        }) => {
            let font_color = match content {
                TileContent::Nothing => WHITE,
                TileContent::Bulb => BLACK,
                TileContent::Cross => RED,
            };
            let bg_color = if *times_lit > 0 { YELLOW } else { WHITE };

            (font_color, bg_color)
        }
        Tile::Wall(_) => (WHITE, BLACK),
    };

    ColorStyle::new(font_color, bg_color)
}

struct Game {
    grid: Grid,
    focused: Option<Vec2>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            grid: Grid::new_hardcoded(),
            focused: None,
        }
    }

    pub fn get_tile(&self, mouse_pos: Vec2, offset: Vec2) -> Option<Vec2> {
        mouse_pos
            .checked_sub(offset)
            .map(|pos| pos.map_x(|x| x / 2))
            .and_then(|pos| {
                if pos.fits_in(self.grid.size().map_x(|x| x - 1).map_y(|y| y - 1)) {
                    Some(pos)
                } else {
                    None
                }
            })
    }
}

impl cursive::view::View for Game {
    fn draw(&self, printer: &Printer) {
        for (x, row) in self.grid.grid.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                let color_style = color_style(tile);
                let tile_symbol = tile_symbol(tile);

                printer.with_color(color_style, |printer| {
                    printer.print((x * 2, y), tile_symbol)
                });
            }
        }
    }
    fn required_size(&mut self, _: Vec2) -> Vec2 {
        self.grid.size().map_x(|x| x * 2)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Mouse {
                offset,
                position,
                event: MouseEvent::Press(_),
            } => {
                if let Some(pos) = self.get_tile(position, offset) {
                    self.focused = Some(pos);
                    return EventResult::Consumed(None);
                }
            }
            Event::Mouse {
                offset,
                position,
                event: MouseEvent::Release(button),
            } => {
                if let Some(pos) = self.get_tile(position, offset) {
                    if self.focused == Some(pos) {
                        match button {
                            MouseButton::Left => {
                                self.grid.toggle(pos.x, pos.y);
                                return EventResult::Consumed(None);
                            }
                            MouseButton::Right => {
                                self.grid.toggle_back(pos.x, pos.y);
                                return EventResult::Consumed(None);
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }

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
            .content(LinearLayout::vertical().child(Panel::new(Game::new())))
            .padding_lrtb(4, 5, 1, 1)
            .button("Quit game", |s| {
                s.pop_layer();
            }),
    );
}

fn how_to_play(siv: &mut Cursive) {
    siv.add_layer(Dialog::info("Todo"));
}

fn main() {
    cursive::logger::init();

    info!("Starting game");

    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::new().title("Akari").content(
            LinearLayout::vertical()
                .child(Button::new_raw("  New game   ", show_board))
                .child(Button::new_raw(" How to play ", how_to_play))
                .child(Button::new_raw("    Exit     ", Cursive::quit)),
        ),
    );
    siv.add_global_callback('q', Cursive::quit);
    siv.add_global_callback('`', Cursive::toggle_debug_console);
    siv.run();
}
