use std::collections::HashMap;

pub fn hashmap () {
    let mut capital_city = HashMap::new();
    

    capital_city.insert("Bangladesh","Dhaka");
    capital_city.insert("India","Dheli");
    capital_city.insert("Usa","washintong");

    println!("{:?}",capital_city);

}