#![cfg(feature = "secret")]
use std::u128;

use secret_toolkit::storage::{Item, Keymap};

use crate::state::{Config, Epoch};

pub const EPOCHS: Keymap<u128, Epoch> = Keymap::new("epochs".as_bytes());
pub const CONFIG: Item<Config> = Item::new("config".as_bytes());
