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
