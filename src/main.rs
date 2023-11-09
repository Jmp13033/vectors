mod helpers; 
use helpers::{filter_even_numbers, find_shortest_string, find_max, find_unique_elements, filter_strings_by_length, find_common_elements, multiply_vectors, absolute_difference};


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

    let arr1 = vec![0,8,7,8,5,4]; 
    let arr2 = vec![9, 8,5,3,2]; 
    let elem = find_unique_elements(arr1, arr2);
    let mut arr3: Vec<_> = vec![];
    arr3.push("Dinner".to_string());
    arr3.push("hello".to_string());
    
    let filtered_strings = filter_strings_by_length(arr3, 4); 


    println!("{:?}", filtered_strings); 



    println!("{:?}", elem); 

    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = vec![3, 4, 5, 6, 7];

    let common_elements = find_common_elements(vec1, vec2);

    println!("{:?}", common_elements); 
    

    let vector1 = vec![9,8,0,7,4]; 
    let vector2 = vec![9,9,7,6,5];
    let vector3 = vec![9,8,0,7,4]; 
    let vector4 = vec![9,9,7,6,5]; 

    let multiply = multiply_vectors(vector1, vector2); 
    let abs_diff = absolute_difference(vector3, vector4); 

    println!("{:?}", multiply); 

    println!("{:?}", abs_diff); 


    






}


