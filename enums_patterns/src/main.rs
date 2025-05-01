use std::fmt::format;

#[derive(Debug)]
enum IpAddType {
    V4,
    V6
}
// we can use enum type in structs also
// in struct kind must be of the same type always
// while in enum we can have different data types: 
// v4 could have 4 u8 and v6 could have a string
#[derive(Debug)]
struct IpAddr {
    kind: IpAddType,
    address: String,
}

// there is a standard library for IP address which uses structs for ipv4Addres and IPv6Addres
// then enum with these as parameters to the items
// struct Ipv4Addr {}
// struct Ipv6Addr {}
// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

// using enum only
#[derive(Debug)]
enum IpAddress {
    V4(()),
    V6(String),
}
// methods in enums are similar to struct
impl IpAddress {
    fn route(&self) ->u8 {
        5
    }
}
// this none enum is used to check if a value is present or not.
// they will return an instance of this enum which is Option<T>
// enum Option<T> { // this <T> is a generic type parameter
//     Some(T),     // so this T type can be ay DS
//     None  // for none we need to annotate the data type explicitly
// }

#[derive(Debug)]
enum States {
    Albama,
    Alaska,
}
impl States {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            States::Alaska => year>=1819,
            States::Albama => year>=1959,
        }
    }
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(States),
}
fn main() {
    let four = IpAddType::V4;
    let _six = IpAddType::V6;

    let home = IpAddr {
        kind: four,
        address: String::from("127.0.0.1"),
    };
    println!("using struct home: {:?}, {:?}", home. kind, home.address);
    let local = IpAddress::V4(());
    println!("Local: {:?}", local.route());

    let loopback = IpAddress::V6(String::from("::01"));
    println!("loopback: {:?}", loopback);
    let m = loopback.route();
    println!("{:?}", m);
    
    // to call Some and None we do not need scope operator. it is already included only
    let something = Some(10);  // type of this is Option<i32> rust will infer this by itself by looking at T
    let nothing: Option<i32> = None; // type is option<i32>
    // we cannot directly use these values like normal ds
    let num = 5;
    // this will give erorr
    // cannot add `Option<{integer}>` to `{integer}`
    // let sum = num+something; 
    // we need to convert it to a valid type before using it in program
    // so, everytime we see a value that isnt of type option we can safely assume that it will never be null
    // everytime we want to use a Option<T> value then we need to check it for all variants of some

    coin_value(Coin::Quarter(States::Alaska));
    // println!("{}", sum);
    
    let incr_something = plus_one(something);
    let incr_nothing= plus_one(nothing);
    println!("something incr : {:?}, nothing incr : {:?}", incr_something, incr_nothing);
    // we are still able to access this something because Option has Copy trait (stack)
    println!("someting: {:?}", something);
    
    // CATCH ALL = _ is used when we dont want to use the returned value for other patterns
    let dice_roll = 6;
    match dice_roll {
        3 => add_points(),
        7 => lucky_move(),
        // for all other values/ patterns
        // other => move_player(other), 
        _ => (), // we wont use this value ever. this empty tuple is for no code
    }

    // IF LET => we use this when we dont want to consider all possibilities but only one
    // so instead of writing pattern match code for all possible values we can just use one check using if
    // this if let statement will behave same if we wrote one pattern match code and rest _ => ()

    // in case we want to execute some code on _ also then we can add else
    let mut count = 0;
    if let Some(m) = something {
        println!(" this is something => {:?}", m);
    } else {
        count += 1;
    }
    println!("{count}");

    let cn = Coin::Quarter(States::Alaska);
    let descrip =  describe_state(cn);
    if let Some(s) = descrip {
        println!("{s:?}");
    } 

}

fn coin_value(coin: Coin) -> u8 {
    // a match expressions needs to have all arms
    // match arms can bind to a part of value that match pattern as well
    match coin {
        Coin::Penny => {
            println!("one penny is 1!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // when quarter is matched, a part of the value -> state will bind to the st in the arm
        Coin::Quarter(st) => {
            println!("this quarter is from {st:?}");
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn add_points() {}
fn lucky_move() {}
// fn move_player(num_ahead: u8) {}

fn describe_state(coin: Coin) -> Option<String> {
    // if let Coin::Quarter(state) = coin {
    //     if state.existed_in(1900) {
    //         Some(format!("{state:?} is pretty old."))
    //     } else {
    //         Some(format!("{state:?} is new."))
    //     }
    // } else {
    //     // we can return None because the return type is set to option
    //     None
    // }
    
    // simplify the above
    
    // let state = if let Coin::Quarter(st) = coin {
    //     st
    // } else {
    //     return None
    // };

    // to make this even simpler we have LET-ELSE
    let Coin::Quarter(state) = coin else {
        return None;
    }; 
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old."))
    } else {
        Some(format!("{state:?} is new."))
    }

}