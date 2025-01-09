use std::{collections::HashMap, vec};

fn main() {
    let numbers : Vec<i32> = vec![8,3,5,2,5,9,7];
    let target :i32 = 12;
    // println!("{:?}", numbers);
    // let result: Vec<i32> = two_sum(numbers, target);
    let kashem= two_sum_onepass(numbers, target);
    println!("{:?}", kashem);
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut track_map : HashMap<i32, i32> = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        track_map.insert(num, i as i32);
        
    }
    for(i, &num) in nums.iter().enumerate(){
        if let Some(&v) = track_map.get(&(target-num)){
            if i as i32 !=v{
                return vec![i as i32, v];
            }
               
            }
        
        }

    vec![]
    
}  


// Solution number 2
fn two_sum_onepass(nums:Vec<i32>, target: i32)-> Vec<i32>{

    let mut vec_map: HashMap<i32, i32> = HashMap::new(); //initializing a Hashmap

    for (i, &num) in nums.iter().enumerate(){
        println!(" i:{}", &i); //traversing the vector 
        let complement = target- num; // complementing the value, target - num
        if let Some(&v) = vec_map.get(&complement){
             //searching in the vector if any value in the vector is equal to the complement value
            return vec![v as i32, i as i32]; //if yes then it will send the value of the v and the i
        }
        println!("inext:{} and num{}", &i, &num );
        vec_map.insert(num, i as i32);
        println!("{:?}", &vec_map);
    }
    vec![]//returning empty vector if it does not find



}


