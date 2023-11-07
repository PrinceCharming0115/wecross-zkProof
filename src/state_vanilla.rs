#![cfg(feature = "vanilla")]

use crate::state::{Config, Epoch};
use cw_storage_plus::{Item, Map};

pub const EPOCHS: Map<u128, Epoch> = Map::new("epochs");
pub const CONFIG: Item<Config> = Item::new("config");
