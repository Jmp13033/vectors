use std::error::Error;



pub fn smallest_element(numbers: &Vec<i32>) -> Result<i32, Box<dyn Error>> {

    if numbers.is_empty() {

       return Err("This is not correct".into()); 
    }

    let mut min_elem = numbers[0];

    for &elem in numbers.iter() {

        if elem < min_elem {


            min_elem = elem; 
        }

    

    }



    Ok(min_elem) 






}


pub fn zipped_array(list1: Vec<i32>, list2: Vec<String>) -> Vec<(i32, String)> {
    // zip types yields references 
    let zipped: Vec<(i32, String)> = list1.iter().zip(list2.iter()).map(|(&a, b)| (a, b.clone())).collect(); 
    zipped


}


pub fn sum_of_squares(numbers: Vec<i32>) -> Result<i32, Box<dyn Error>> {

    if numbers.is_empty() {

        return Err("this is my box type error".into())
    }

    let array = numbers.iter().filter(|&x| x % 2 == 0).map(|&x| x.pow(2)).sum(); 

    Ok(array)





}
