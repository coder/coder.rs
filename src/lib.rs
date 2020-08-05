// #![feature(trace_macros)]
// trace_macros!(true);
//
// #![allow(dead_code)]
// #![allow(unused_macros)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]

#[macro_use]
extern crate paste;

#[macro_use]
mod macros;

pub mod client;
pub mod headers;
pub mod models;
pub mod users;

mod util;
