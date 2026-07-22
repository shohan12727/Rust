pub fn generic () {
  
    let number_list = vec![34,5,34,65,76,875,65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");


}


// fn largest(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];

//     for item in list{
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

use std::cmp::PartialOrd;

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}



pub fn generic_2 () {
    
    let number_list = vec! [1,2,3,4,5,6,7,89];
    let result = largest(&number_list);
    println!("The largest number is {result}");


    let number_list = vec![3,4,5];
    let result = largest(&number_list);
    println!("The largest number iss {result}");

}
