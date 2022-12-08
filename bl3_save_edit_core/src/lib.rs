// Load I18n macro, for allow you use `tfl!` macro in anywhere.
#[macro_use]
extern crate bl3_save_edit_i18n;

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
