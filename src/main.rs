
extern crate num;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate nom;

pub mod instructions;
pub mod vm;
pub mod repl;
pub mod assembler;

fn main() {
    let mut repl = repl::REPL::new();
    repl.run();
}
