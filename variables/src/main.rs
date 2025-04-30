use std::io;

fn main() {
    // variables, immutable by default
    let mut x = 5;
    println!("x = {}", x);
    x = 6;
    println!("x = {}", x);

    // constant values r bound to a name and not allowed to change
    // cannot use mut. they're always immutable
    const PI: f32 = 3.142; // type of value needs to be given
    println!("pi= {}", PI);

    // shadow -- without using mut we r now creating a new var with same name in same scope
    let x = 10; // earlier x was a mut var with val 6
    // in different scope (eg, loop, inner scope)
    {
        let x = "hii, world!";
        println!("x inside = {}", x);
    }
    println!("x outside = {}", x);

    // DATA types
    // scalar: integer, flating-points, booleans, characters
    
    // integer: i8, u8, i16, u16 .... i128, u128, isize, usize
    // last 2 depend on architecture of ur computer (32 or 64)
    // in case of overflow error: 
    // 1. in debug mode: PANIC
    // 2. in release mode: 2's complement wrapping

    // floating points: f32, f64 (both are signed)
    let quotient = 43.00/ 3.00; // will return float val because operands are also float
    let rem = 43 % 3;
    println!("quotient: {} ", quotient);
    println!("rem: {} ", rem);

    // compound types: tuple, arrays
    // tuples: fixed size, can be different types
    let tup = (1, 2.44, 3000);
    let (x, y, z) = tup; // pattern match destructuring
    println!("x={}. y={}, z={}", x, y, z);
    // index match
    println!("0th={}, 1st={}, 2nd={}", tup.0, tup.1, tup.2);
    // empty tuple = unit
    // let tup: () = ();

    // Array: same type, stack, fixed length
    // Vector: dynamic size array, heap
    let arr: [i32; 5] = [1, 2, 3 ,4, 5];
    // let arr2 = [5;3]; // [5,5,5]
    let first = arr[0];
    let second = arr[1];
    println!("first = {}, second = {}", first, second);
    // array length
    println!("length of arr: {}", arr.len());

    let mut index = String::new();
    println!("input index: ");

    io::stdin()
        .read_line( &mut index)
        .expect("failed");
    let index: usize = index.trim().parse()
        .expect("not a number");
    let element = arr[index];
    println!("element at index {} = {}", index, element);

    // statments and expressions
    // statements DO NOT returan a value
    // let x = (let y=5) ; this will give error
    // while in c we can use things like x=y=5; assignment also returns a value
    
    // expressions:  call a function, marco, code block {}
    // expressions do not end with semi-colon
    let y = {
        let x = 5;
        x
    };
    println!("y={y}");
    let mine = my_num();
    println!("my number = {mine}");
    let ans = add_one(&mine);
    println!("mine = {}, old={}", ans, mine);

    // 3 arms if block
    if ans>=mine {
        println!("ans is greater");
    } else if ans == 11 {
        println!("ans is 11");
    } else {
        println!("else line");
    }

    let mut cnt = 0;
    let result: bool = loop {
        cnt += 1;
        if cnt == 5 {
            break true;
        }
    };
    // loop labels can be used to distinguish multiple loops
    // allows using break and contninue much better
    println!("result: {}", result);
    
    let mut outer_cnt = 0;
    'outer: loop {
        println!("outer loop");
        outer_cnt = outer_cnt+ 1;
        let mut cnt = 0;
        'inner: loop {
            println!("inner loop");
            cnt = cnt+1;
            if cnt ==3 { break 'inner; }; // can use it for outer also
        }
        if outer_cnt == 2 { break 'outer; }; // if o label specefied then it uses closest scope
    }

    // while loop is slower than for loop 
    // for will just increment to the end
    // while adds a check->evaluates true or false to move ahead
    
    // using range 
    for i in 1..10 {
        print!("{} ", i);
    } 
    println!();
    // .rev for reverse
    for i in (1..10).rev() {
        print!("{} ", i);
    }
    println!();
}

fn my_num() -> i32 {
    10
}
fn add_one(x: &i32) -> i32 {
    x+1
}