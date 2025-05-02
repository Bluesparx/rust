#[derive(Debug)]
pub struct Asparagus {}

// even methods need to be public to be
impl Asparagus {
    pub fn print_name(&self){
        println!("Asparagus module");
    }
}