pub fn closure () {

    let add_one = |x:i32| -> i32 {
        x + 1 
    };

    println!("{}", add_one(100));

}