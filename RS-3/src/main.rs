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




fn main() {

    User1();

    let now = Local::now();
    // println!("current time is {}", now);

    




    // slicing();
    // reference();

   let x = true;
        //   read(x);

}
