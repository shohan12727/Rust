pub fn learn_vector() {

    enum spreed_sheet_cell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        spreed_sheet_cell::Int(50),
        spreed_sheet_cell::Float(70.56),
        spreed_sheet_cell::Text(String::from("Shohan King")),
    ];


    match &row[2]{
        spreed_sheet_cell::Int(i) => println!("{}",i),
        _ => println!("Not a integer"),
    }

}


pub fn learn_vector_2 () {
    // println!("from vector 2");

    let mut v = vec![1,2,3,4,5,6,7,8,9];
    let fifth = v[5];

    println!("{}",fifth);

    for i in &v {
        println!("{i}")
    }

    let fifth = v.get(5);
    match fifth {
        Some(fifth) => println!("The fifth element is {fifth}"),
        None => println!("There is no fifth element"),
    }

}