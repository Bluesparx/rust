struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
// tuple structs, used to give a name to tuple
// struct Colour(u32, u32, u32); //no named attributes
struct Point(i32, i32, i32);

// Unit like struct with no fields at all
struct AlwaysEqual;

// need a debug trait to be able to print a struct
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut user1 = User {
        email: String::from("mymail@gmail.com"),
        username: String::from("nazia"),
        active: true,
        sign_in_count: 1
    };
    let name = user1.username;
    // we cant make a single attribute mutable, we need to make complete object mutable
    user1.username = String::from("myname12");
    println!("{name}, {}, {}, {}", user1.email, user1.active, user1.sign_in_count);

    let user2 = build_user(
        String::from("johndoe@gmail.com"), 
        String::from("johndoe123")
    );
    // creating new instance from existing instance
    // we are using an assignment so user2 will go out of scope after this
    // ownership is moved to user3
    let user3 = User {
        ..user2
    };
    // but if we had passed only stack data (literals) the it would still be useful
    // stack data has Copy trait which heap data doesnt
    let _user4 = User {
        email: String::from("john@gmail.com"), 
        username: String::from("john3"),
        ..user3   // only copying u64 data  
    }; 
    // let _black = Colour(0,0,0);
    let origin = Point(0,0,0);
    // when we destructure we need to give struct  name like
    let Point(x, y, z) = origin;
    println!("{},{},{}", x,y,z);

    let _subject = AlwaysEqual; // used when e want same instance no matter what

    let rect: (u32, u32) = (30, 20);
    println!("area of rectangle is: {}", area(rect));
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30*scale), 
        height: 20
    };
    // dbg!(&rect1);
    println!("rectangle dims: {:#?}", rect1);
    println!("area of rectangle is: {}", rect1.area());

    let rect2 = Rectangle {
        width: 10,
        height: 35
    };
    println!("can hold output: {} ", rect1.can_hold(&rect2));

    let a = 10;
    let sq = Rectangle::square(a);
    print!("area of square: {}", sq.area());
}

// constructor --> field init shorthand function we dont have to repeat the field names
fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        active: true, // default
        sign_in_count: 1,
    }
}

fn area(dims:(u32, u32)) -> u32 {
    dims.0 * dims.1
}

// fn area_struct(rect: &Rectangle) ->u32 {
//     rect.width * rect.height
// }
// implementation block of rectangle struct
impl Rectangle {
    // methods are tied to instances of the struct,
    // only an object can call these methods
    fn area(&self) -> u32 {
        self.height*self.width
    }
    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width>other.width && self.height>other.height
    }
   
}

// we can have more than one impl block
impl Rectangle {
    // associated functions dont need self parameter 
    // we can call this function directly by struct name suing scope operator ::
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}