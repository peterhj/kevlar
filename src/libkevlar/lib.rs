#![crate_id = "kevlar#0.1"]
#![crate_type = "lib"]

#![allow(dead_code)]
#![allow(unused_variable)]
#![allow(visible_private_types)]

#![feature(default_type_params)]

extern crate collections;

pub mod cast;
pub mod client;
pub mod server;

mod internal;
