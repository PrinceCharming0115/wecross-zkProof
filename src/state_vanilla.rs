#![cfg(feature = "vanilla")]

use crate::state::{Config, Epoch};
use cosmwasm_std::{Order, StdResult, Storage};
use cw_storage_plus::{Item, Map};

pub const EPOCHS: Map<u128, Epoch> = Map::new("epochs");
pub const CONFIG: Item<Config> = Item::new("config");

pub fn get_all_epochs(storage: &dyn Storage) -> StdResult<Vec<u128>> {
    EPOCHS.keys(storage, None, None, Order::Ascending).collect()
}
