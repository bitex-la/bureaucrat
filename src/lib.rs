#![feature(custom_derive, plugin, macro_reexport)] 
#![recursion_limit = "1024"] // for ErrorChain recursion.

#[macro_use]
extern crate error_chain;
#[macro_use]
pub mod spec_helper;
pub mod errors;
mod common;
pub mod cbu;
pub mod cuit;
pub mod json_api;

