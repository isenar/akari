#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tile {
    Togglable(TogglableTile),
    Wall(Wall),
}

impl Tile {
    pub const fn blank() -> Self {
        Self::lit_empty(0)
    }

    pub const fn wall() -> Self {
        Self::Wall(Wall::Clear)
    }

    pub const fn lit_empty(level: u8) -> Self {
        Self::Togglable(TogglableTile {
            light_level: level,
            content: TileContent::Nothing,
        })
    }

    #[cfg(test)]
    pub const fn bulb(level: u8) -> Self {
        Self::Togglable(TogglableTile {
            light_level: level,
            content: TileContent::Bulb,
        })
    }
}

impl Tile {
    pub fn toggle(&mut self) -> BulbAction {
        if let Self::Togglable(togglable) = self {
            togglable.toggle()
        } else {
            BulbAction::Nothing
        }
    }

    pub fn toggle_back(&mut self) -> BulbAction {
        if let Self::Togglable(togglable) = self {
            togglable.toggle_back()
        } else {
            BulbAction::Nothing
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

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Wall {
    Clear,
    Zero,
    One,
    Two,
    Three,
    Four,
}

impl TogglableTile {
    fn toggle(&mut self) -> BulbAction {
        let (new, action) = match self.content {
            TileContent::Nothing => (TileContent::Bulb, BulbAction::Inserted),
            TileContent::Bulb => (TileContent::Cross, BulbAction::Removed),
            TileContent::Cross => (TileContent::Nothing, BulbAction::Nothing),
        };

        self.content = new;

        action
    }

    fn toggle_back(&mut self) -> BulbAction {
        let (new, action) = match self.content {
            TileContent::Nothing => (TileContent::Cross, BulbAction::Nothing),
            TileContent::Bulb => (TileContent::Nothing, BulbAction::Removed),
            TileContent::Cross => (TileContent::Bulb, BulbAction::Inserted),
        };

        self.content = new;

        action
    }
}
#[derive(Debug)]
pub enum BulbAction {
    Inserted,
    Removed,
    Nothing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileContent {
    Nothing,
    Bulb,
    Cross,
}
