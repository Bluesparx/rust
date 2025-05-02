// this is library crate
// we can only have one library crate 


// use -> to bring into scope
// how will we allow other external files to use the function add_waitlist
// we will use pub keyword with use to re-export
pub use crate::front_of_house::hosting;

// use rand::Rng; // we're using an external lib crate
// use only applies in the same scope it is in.
// nested path {}, we can also import all public items using * (glob operator)
use rand::{Rng, CryptoRng, ErrorKind::Transient};
use std::io::*;

// this library is for a restaurant 
mod front_of_house {
    // these two modules are siblings that means they're defined in same module
    pub mod hosting {
        pub fn add_waitlist() {}
        pub fn seat_at_table() {}
        // we can even have structs, enums, constants etc here
        const TOTAL_TABLES: i32 = 10;
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn make_payment() {}
    }
}
// this pub fn is basically like api endpoint
pub fn eat_at_restaurant() {
    // absoulte path
    // why crate? -> this is refering to lib crate which is the first crate in the package
    crate::front_of_house::hosting::add_waitlist();
    // relative path 
    front_of_house::hosting::seat_at_table();
    // using the use scope
    hosting::add_waitlist();
    // order a summer bf
    let mut order = back_of_house::Breakfast::summer("rye");
    order.toast = String::from("wheat");
    // we can print the toast because it is set to public
    println!("i'd like {} please", order.toast);
    // if we try to access seasonal_fruits we cant, because it is private
    println!("{:?}", order);

}

// using super keyword
fn serve_order() {}

mod back_of_house {
    fn fix_order() {
        cook_order();
        // we cant use use here because its outside the socope of this code block
        // reference the parent module of this mod using super keyword
        super::serve_order();
    }
    fn cook_order() {}
    // struct
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruits: String, // this field is still private even tho struct is public
    }
    impl Breakfast{
        // because we have a private field we need to construct a constructor by ourselves otherwise it wont work
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruits: String::from("mangoes"), 
            }
        }
    } 
    // enum -> if we make an enum public, all its types becomes public
    
}

// usually we will bring into scope the function we want to use directly by using use but
//  if we have same name functions we cant do that 
// then we would just bring it's parent
// use std::fmt;
// use std::io;

// we can also rename it
use std::fmt::Result;
use std::io::Result as ioResult;

fn func1() -> Result {
    Ok(())
}

fn func2() -> ioResult<()> {
    Ok(())
}

