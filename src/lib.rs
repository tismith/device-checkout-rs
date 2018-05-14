#![feature(plugin, custom_derive, custom_attribute)]
#![plugin(rocket_codegen)]

//#[macro_use] //this crate has macros, currently unused
pub extern crate failure;
#[macro_use]
pub extern crate log;
pub extern crate stderrlog;
#[macro_use]
pub extern crate clap;
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

pub mod database;
pub mod models;
pub mod pool;
pub mod routes;
pub mod schema;
pub mod utils;
