use rusqlite::Connection;
use std::sync::Mutex;

use crate::models::{GameSettings, WorldState};

pub struct GameState(pub Mutex<WorldState>);
pub struct DbState(pub Mutex<Connection>);
pub struct SettingsState(pub Mutex<GameSettings>);
