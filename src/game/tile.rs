#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tile {
    Togglable(TogglableTile),
    Wall,
    Number(u8),
}

impl Tile {
    pub const fn blank() -> Self {
        Self::lit_empty(0)
    }

    pub const fn lit_empty(level: u8) -> Self {
        Self::Togglable(TogglableTile {
            times_lit: level,
            content: TileContent::Nothing,
        })
    }

    pub const fn bulb(level: u8) -> Self {
        Self::Togglable(TogglableTile {
            times_lit: level,
            content: TileContent::Bulb,
        })
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

    pub fn light_up(&mut self) {
        if let Self::Togglable(togglable) = self {
            togglable.times_lit += 1;
        }
    }

    pub fn light_down(&mut self) {
        if let Self::Togglable(togglable) = self {
            togglable.times_lit -= 1;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileContent {
    Nothing,
    Bulb,
    Cross,
}
