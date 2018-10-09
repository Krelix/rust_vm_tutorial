#![warn(clippy)]
#[macro_use]
extern crate nom;

pub mod vm;
pub mod instruction;
pub mod repl;
pub mod assembler;

use vm::VM;
use repl::REPL;

fn main() {
    let mut repl_instance = REPL::new();
    repl_instance.run();
}
