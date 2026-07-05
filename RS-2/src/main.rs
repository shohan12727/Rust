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


fn main() {
    // sum();
    // scope();
    // spaces();
    // tuple();
}
