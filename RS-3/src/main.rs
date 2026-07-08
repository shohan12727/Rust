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



fn main() {

    reference();

   let x = true;
        //   read(x);

}
