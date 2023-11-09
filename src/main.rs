pub fn adding_evens(vector: Vec<i32>) -> i32 {

    let mut sum = 0; 

    for i in vector {

        if i % 2 == 0 {

            sum += i; 


        }
    


    }
    sum 

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


fn find_shortest_string(strings: Vec<String>) -> String  {

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
    
    
    
    



    //let num = adding_evens(numbers);

    //array_number.push(num); 
    //println!("{:?}", array_number); 



}


