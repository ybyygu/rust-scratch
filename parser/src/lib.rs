// lib.rs
// :PROPERTIES:
// :header-args: :tangle src/lib.rs
// :END:

// [[file:~/Workspace/Programming/rust-scratch/parser/parser.note::*lib.rs][lib.rs:1]]
#![allow(dead_code)]

#[macro_use] extern crate nom;
#[macro_use] extern crate quicli;

// for tests only
#[cfg(test)]
#[macro_use] extern crate approx;

#[macro_use]
mod nom_parser;
mod parser;
mod pdb;
mod cif;

pub use self::nom_parser::*;
pub use self::parser::*;
// lib.rs:1 ends here
