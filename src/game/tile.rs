use std::borrow::Cow;

#[derive(Debug)]
pub enum Tile {
    Togglable(TogglableTile),
    Wall,
    Number(u8),
}

impl Tile {
    pub const fn blank() -> Self {
        Self::Togglable(TogglableTile::empty())
    }
}

impl Tile {
    pub fn toggle(&mut self) -> ActionResult {
        if let Self::Togglable(togglable) = self {
            togglable.toggle()
        } else {
            ActionResult::Nothing
        }
    }

    pub fn symbol(&self) -> Cow<str> {
        match self {
            Tile::Togglable(TogglableTile { content, .. }) => match content {
                TileContent::Nothing => Cow::Borrowed(" "),
                TileContent::Bulb => Cow::Borrowed("B"),
                TileContent::Cross => Cow::Borrowed("X"),
            },
            Tile::Wall => Cow::Borrowed(" "),
            Tile::Number(n) => Cow::Owned(n.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct TogglableTile {
    pub times_lit: u8,
    pub content: TileContent,
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
#[derive(Debug)]
pub enum ActionResult {
    BulbInserted,
    CrossInserted,
    TileCleared,
    Nothing,
}

#[derive(Debug)]
pub enum TileContent {
    Nothing,
    Bulb,
    Cross,
}
