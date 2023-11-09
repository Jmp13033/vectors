mod helpers; 
use helpers::{filter_even_numbers, find_shortest_string, find_max };

fn main() {

    let numbers = vec![7,9,8,7,5]; 
    let array_number:  Vec<i32> = numbers.iter().map(|&num| num * 2).collect(); 
    let result = find_max(array_number); 
    println!("{}", result); 

    let string_array: Vec<String> = ["Hello", "GoodBye", "See", "Later", "Los"].iter().map(|&x| x.to_string()).collect();
    
    let result = find_shortest_string(string_array);

    println!("{0}", result);

    let mut array_of_strings: Vec<String> = ["Hi", "How", "Are"].iter().map(|&x| x.to_string()).collect(); 

    for (index, value) in array_of_strings.iter().enumerate() {

        println!("{0}, {1}", index, value);
    }

    let my_vec = vec![1, 2, 3, 4];


    let filtered = filter_even_numbers(my_vec); 

    println!("{:?}", filtered); 

    //let num = adding_evens(numbers);

    //array_number.push(num); 
    //println!("{:?}", array_number); 



}


