
fn main() {
    println!("Hello, world!");
    // Ownership
    {
        let _s: &str = "hello"; // string literal
        let _s2 : String = String::from("hello"); // heap
        // underscore is used for unused variables
        // do stuff with s
    } // s goes out of scope here, so s is not valid anymore

    // interaction
    // for literals this copy will work
    let x = 5;
    let y = x; // Copy x into y 
    // x is still valid because this is a local variable (in stack)
    println!("x: {}, y:{}", x, y); 
    // but for heap allocated data this will Move
    let s1 = String::from("hello");
    let s2 = s1; // move s1 into s2
    println!("s2: {}", s2); // this will give error
    // for deep copy we have a clone method
    let s3 = s2.clone(); // both s2 and s3 are diff same variables
    println!("s3: {}", s3);

    let s_own = String::from("string string");
    takes_ownership(s_own);
    // println!("s_own: {}", s_own); 
    // this will give error, because we're giving ownership of s_own to the function

    let x = 10;
    makes_copy(x);
    println!("s: {}", x);

    let s_new = gives_ownership(); // s_new is in scope now and has value from function return
    println!("s_new: {}", s_new);

    let s2 =String::from("this one");
    let s3 = takes_and_gives_ownership(s2);
    // s2 is no longer in scope. 
    println!("s3: {}", s3);

    // what if i want to use string after pasing it to function
    // we can use tuple to return multiple values
    let s4 = String::from("hi, my name is nazia");
    let (s5, len) = calculate_length(s4); 
    // but after this s4 is till not valid , goes out of scope
    // for that we will use references.
    println!("s5: {}, len: {}", s5, len);

    // REFRENCES
    let s1 = String::from("hii this is nazia");
    let length = calculate_length_ref(&s1);
    println!("s1: {s1}, length: {length}");
    
    let mut s2 = String::from("hello");
    change(&mut s2);
    change_again(&mut s2); 
    // these two functions are called in a chain, not simulataneously
    // if we try to use more than 1 mutable reference at the same time it wont work
    // more than 1 immutable is allowed 
    let r1 = & s2;
    let r2 = & s2;
    println!("r1 = {r1}, r2 = {r2}"); // we would get error here
    // after scope of r1, r2 is over, we can have mutable reference now
    let r3 = &mut s2; // this is OK
    println!("r3: {r3}");
    println!("s2: {s2}");

    // dangling references 
    // let refer_to_nothing = dangle(); 

    // Slices let us reference a sequence of elements in a collection 
    let mut s = String::from("hello, world my name is nazia");
    println!("{s}");
    let word = first_word(&s);
    let slice = &s[0..2];
    println!("slice: {slice}, first word: {word}"); 
    s.clear(); // empty the string making it equal to ""
    // now, word still has the value even though s is empty
    // so instead of returning the word we will return a part of reference to the string
    // so when s goes out of scope or is changed. word will also get changed.

    // this will give error because it is out of scope now
    // println!("slice: {slice}, first word: {word}"); 

    let a = [1, 2, 3, 4, 5, 6];
    let slice = &a[2..5];
    for i in slice {
        println!("{}", i);
    }
    

}

fn takes_ownership(some_string: String){ 
    // some_string is valid,in scope inside the function only
    println!("{}", some_string);
} // some_string goes out of scope , drop is called , memory is freed

fn makes_copy(some_int: i32){ // some_int is a copy of s.
    println!("{}", some_int);  // some_int is in scope
}// some_int goes out of scope, but nothign happens. its just a copy.

fn gives_ownership() -> String {
    let some_string = String::from("take it");
    some_string
} // this returns some_string, so the ownership is moved to the caller

// next function takes a string and returns a string
fn takes_and_gives_ownership(some_string: String) -> String {
    some_string
}

fn calculate_length(s: String) -> (String, usize){
    let len = s.len();
    (s, len)
} // giving ownership of s and len to the caller


// passing in parameters as reference is called Borrowing
fn calculate_length_ref(s: &String) -> usize { // s borrows ownership of the string  
    let len = s.len();
    // s.push_str(", hehe"); // this will not even work in rust
    len 
} // s goes out of scope, but nothing happens 
// string is still valid outside this function scope.

// mutable reference allows us to modify in the borrowed value
// but the borrwed value must be mutable
fn change(some_string: &mut String) {
    some_string.push_str(", world");
    change_again(some_string);
    
} // no need to return anything, we just had ot change the string.

fn change_again(some_string: &mut String) {
    some_string.push_str(", again");
}

// fn dangle() -> String {
//     s = String::from("current");
//     // function will return reference to this string
//     &s
// } 
// s will go out of scope here, however the caller will need to get return value 
// return value is a dangling reference so it will not compiler we get an errro
// we can fix this by using lifetimes

fn first_word(s: &String) -> &str {
    let bytes  = s.as_bytes();

    for (i, &it) in bytes.iter().enumerate() {
        if it == b' ' { // byte literal 
            return &s[0..i];
        }
    }  
    &s[..]
}