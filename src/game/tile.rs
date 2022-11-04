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
            light_level: level,
            content: TileContent::Nothing,
        })
    }

    pub const fn bulb(level: u8) -> Self {
        Self::Togglable(TogglableTile {
            light_level: level,
            content: TileContent::Bulb,
        })
    }
}

impl Tile {
    pub fn toggle(&mut self) -> BulbActionResult {
        if let Self::Togglable(togglable) = self {
            togglable.toggle()
        } else {
            BulbActionResult::Nothing
        }
    }

    pub fn toggle_back(&mut self) -> BulbActionResult {
        if let Self::Togglable(togglable) = self {
            togglable.toggle_back()
        } else {
            BulbActionResult::Nothing
        }
    }

    pub fn increase_light_level(&mut self) {
        if let Self::Togglable(togglable) = self {
            togglable.light_level += 1;
        }
    }

    pub fn decrease_light_level(&mut self) {
        if let Self::Togglable(togglable) = self {
            togglable.light_level -= 1;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TogglableTile {
    pub light_level: u8,
    pub content: TileContent,
}

impl TogglableTile {
    fn toggle(&mut self) -> BulbActionResult {
        let (next, action) = match self.content {
            TileContent::Nothing => (TileContent::Bulb, BulbActionResult::BulbInserted),
            TileContent::Bulb => (TileContent::Cross, BulbActionResult::BulbRemoved),
            TileContent::Cross => (TileContent::Nothing, BulbActionResult::Nothing),
        };

        self.content = next;

        action
    }

    fn toggle_back(&mut self) -> BulbActionResult {
        let (next, action) = match self.content {
            TileContent::Nothing => (TileContent::Cross, BulbActionResult::Nothing),
            TileContent::Bulb => (TileContent::Nothing, BulbActionResult::BulbRemoved),
            TileContent::Cross => (TileContent::Bulb, BulbActionResult::BulbInserted),
        };

        self.content = next;

        action
    }
}
#[derive(Debug)]
pub enum BulbActionResult {
    BulbInserted,
    BulbRemoved,
    Nothing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileContent {
    Nothing,
    Bulb,
    Cross,
}
