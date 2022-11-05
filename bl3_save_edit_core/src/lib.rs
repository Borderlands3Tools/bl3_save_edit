// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;

// Init translations for current crate.
i18n!("locales");

pub mod arbitrary_bits;
pub mod bl3_item;
pub mod bl3_profile;
pub mod bl3_save;
pub mod error;
pub mod file_helper;
pub mod game_data;
pub mod models;
pub mod parser;
pub mod protos;
pub mod resources;
pub mod vehicle_data;
