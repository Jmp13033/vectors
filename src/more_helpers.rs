pub fn zipped_array(list1: Vec<i32>, list2: Vec<String>) -> Vec<(i32, String)> {
    
    // zip types yields references 
    let zipped: Vec<(i32, String)> = list1.iter().zip(list2.iter()).map(|(&a, b)| (a, b.clone())).collect(); 
    zipped


}