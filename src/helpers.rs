use std::collections::HashSet;



pub fn absolute_difference(vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32> {
    if vec1.len() != vec2.len() {

        panic!("different length are not ok"); 
    }



    let mut vector = Vec::with_capacity(vec1.len()); 
    for (num1, num2) in vec1.iter().zip(vec2.iter()) {

        vector.push((num1 - num2).abs())


    }

    vector


}



pub fn multiply_vectors(vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32> {
    if vec1.len() != vec2.len() {
        panic!("length MUST be the same"); 

    }


    let mut new_vec = Vec::with_capacity(vec1.len()); 
    
    for (elem1, elem2) in vec1.iter().zip(vec2.iter()) {

        new_vec.push(elem1 * elem2); 
    }

    new_vec



}

pub fn find_common_elements(vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32> {

    let hashmap1: HashSet<_> = vec1.into_iter().collect(); 
    let hashmap2: HashSet<_> = vec2.into_iter().collect();

    let common_elements: HashSet<_> = hashmap1.intersection(&hashmap2).cloned().collect();

    common_elements.into_iter().collect()

}



pub fn adding_evens(vector: Vec<i32>) -> i32 {

    let mut sum = 0; 

    for i in vector {

        if i % 2 == 0 {

            sum += i; 


        }
    
    }
    sum 
}

pub fn filter_even_numbers(input: Vec<i32>) -> Vec<i32> {

    input.into_iter().filter(|&x| x % 2 == 0).collect()
}


pub fn filter_strings_by_length(strings: Vec<String>, threshold:usize) -> Vec<String> {

    if strings.is_empty() {
        panic!("there are no strings in this list "); 



    }
    strings.into_iter().filter(|x| x.len() > threshold).collect()


}

pub fn find_max(vector: Vec<i32>) -> i32 {
    
    let mut max = vector[0]; 

    for num in vector {

        if max < num {

            max = num
        }


    }

    max

}

pub fn find_shortest_string(strings: Vec<String>) -> String  {

    if strings.is_empty() {

        panic!("There are no strings in this vector "); 



    }

    let mut shortest = &strings[0]; 
    
    for string in &strings {

        if shortest.len() > string.len() {

            shortest = &string; 


        }
        

       
    }

    shortest.to_string()
}



pub fn find_unique_elements(mut arr1: Vec<i32>, arr2: Vec<i32>) -> HashSet<i32> {
    arr1.extend(arr2); 
    let unique_elements: HashSet<_> = arr1.into_iter().collect();

    unique_elements
}
