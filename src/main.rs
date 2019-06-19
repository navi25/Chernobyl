
extern crate num;
#[macro_use]
extern crate num_derive;

pub mod instructions;
pub mod vm;
pub mod repl;

fn main() {
    let mut repl = repl::REPL::new();
    repl.run();
}
