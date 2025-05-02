// binary crate

// we will create a public module for garden using pub mod
// this tells the compiler to look for src/garden.rs
pub use crate::garden::vegetables::Asparagus;

mod garden;

fn main() {
    println!("Hello, world!");
    let plant = Asparagus {};
    plant.print_name();
    println!("{plant:?}");
}
 