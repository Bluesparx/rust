use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main(){
    println!("guess game!");  // using an ! means we r calling a macro. not a function
    let secret_num = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("input a number b/w 1 and 100 or type quit to exit");
        let mut guess = String::new(); // muti s for mutable variables, string is growable
        // let guess = String::new() ; this will be immutable , = in rust will beind the var with the data type or value?
        io::stdin()
            .read_line(&mut guess) // here the pass by reference also needs to be mutable,
            .expect("failed"); // in proper project we would use error handling properly
        // expect will only crash the program , otherwise the readline will return the value it read
        print!("your guess: {}", guess); // {} is a placeholder similar to ${} in js.

        // for random function we dont have inbuilt func but it is there in crate rand
        // crate rand is a executable which means it cnnot be run on its own
        // we will add this as a dependency in our project by adding it to cargo.toml
        
        // we need to convert guess to a number type to match secret and be able to compare
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => break,
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("too low"),
            Ordering::Greater => println!("too high"),
            Ordering::Equal => println!("Correct! Correct number: {}", secret_num),
        }   
    }
    println!("\nGame end!");
}