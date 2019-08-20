#![feature(plugin, proc_macro_hygiene, decl_macro, custom_attribute)]

//#[macro_use] //this crate has macros, currently unused
pub extern crate failure;
#[macro_use]
pub extern crate log;
pub extern crate stderrlog;
#[macro_use]
pub extern crate clap;
#[macro_use]
pub extern crate rocket;
pub extern crate rocket_contrib;
#[macro_use]
pub extern crate serde_derive;
pub extern crate serde;
pub extern crate serde_json;
#[macro_use]
pub extern crate diesel;
#[macro_use]
pub extern crate diesel_migrations;
#[macro_use]
pub extern crate diesel_derive_enum;
pub extern crate chrono;
pub extern crate exitfailure;
#[macro_use]
extern crate validator_derive;
extern crate validator;

pub mod database;
pub mod models;
pub mod pool;
pub mod routes;
pub mod schema;
pub mod utils;
