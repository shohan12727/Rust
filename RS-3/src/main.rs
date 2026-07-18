fn read(y: bool){
    if y {
        println!("y is true!");
    }
}


fn reference(){
    let name = String::from("King Shohan");
    let name_reference = &name;

    println!("{}", name_reference)
}

fn slicing(){
    let array = [1,2,3,4,5,6,7,8,9];
    let slice = &array[1..5];
    println!("{:?}", slice);
}


use chrono::{Local, Utc};


struct User {   // struct is basically a object like in other language ... 
    active: bool,
    userName: String,
    email: String,
    sign_in_count: u32,
}

fn User1 (){
    let user1 = User {
        active:true,
        userName: "King Shohan".to_string(),
        email: "king.shohan@gmail.com".to_string(),
        sign_in_count: 1, 
    };

    println!("{}", user1.userName);
}

fn learn_enum() {
#[derive(Debug)]
enum IpAddrKind{
    V4(String),
    V6(String),
}
let four = IpAddrKind::V4(String::from("this is fucking ip 4 address"));
let six = IpAddrKind::V6(String::from("this is fucking ip 6 address"));
println!("{:?}", four);
println!("{:?}", six);
}

fn learn_enum_2() {

    enum Direction{
        Up,
        Down,
        Left,
        Right,
    }

    let my_direction = Direction::Right;

    match my_direction {
        Direction::Up => println!("Going up"),
        Direction::Down => println!("Going down"),
        Direction::Left => println!("Going Left"),
        Direction::Right => println!("Going RIght"),
    }

}


fn enum_coin() {

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents (coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

}

fn learn_some () {
   let fruit = Some("Apple");
   println!("{:?}", fruit);
}



fn learn_some_2 (){
    // let age = Some(20);
    let age: Option<i32> = None;
    
    match age {
        Some(value) => {
            println!("The age is {}", value);
        }

        None => {
            println!("There is no age.");
        }
    }


}











fn main() {








learn_some_2();
// learn_some();
// enum_coin();
// learn_enum();
// learn_enum_2();

    // User1();

    let now = Local::now();
    // println!("current time is {}", now);

    




    // slicing();
    // reference();

   let x = true;
        //   read(x);

}
