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



fn main() {
    slicing();
    // reference();

   let x = true;
        //   read(x);

}
