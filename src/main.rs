pub mod vm;
pub mod instruction;
pub mod repl;

use vm::VM;
use repl::REPL;

fn main() {
    let mut repl_instance = REPL::new();
    repl_instance.run();
}
