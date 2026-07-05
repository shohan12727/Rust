fn sum () {
    let a = 5;
    let b = 10;
    let total = a + b;
    println!("Total of two number is: {total}");
}

fn scope (){
    let x = 5;
    let x = x +1;
    {
        let x = x * 2;
        println!("The value of of x is the inner scope is: {x}");
    }
    println!("The value of x is: {x}")
}

fn spaces () {
    let spaces = "Brazil";
    let spaces = spaces.len();
    println!("{spaces}")
}


fn tuple(){
  let tup: (i32, f64, u8) = (23, 4.654, 96);
  println!("{:?}", tup)
}

fn array(){
    let numbers = [1,2,3,4,5,6,7,8,9];
    println!("{:?}", numbers)
}

fn another_function (x: i32){  // use of parameters
   println!("The value of X is {x}")
}


fn ownerShipMoved (){
    let name1 = String::from("King Shohan");
    let _name2 = name1.clone();
    println!("{name1}{}", "{name2}")
}



fn main() {
    // sum();
    // scope();
    // spaces();
    // tuple();
    // array();
    // another_function(100); // use of parameters
    ownerShipMoved();
}
