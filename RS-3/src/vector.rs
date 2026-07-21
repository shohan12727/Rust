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