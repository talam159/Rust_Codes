use std::collections::HashMap;


fn main(){
 let nums: &[i32]=&[5,6,3,2,0,1,8];
 let target :i32 = 14;
 let mut result = two_sum(nums, target);
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>{
    let mut track_map: HashMap<i32, i32> = HashMap::new();
    for i in 0..nums.len(){
       track_map.insert(nums, i as i32);
    }
    for i in 0..nums.len(){
        let mut complement = target - nums[i];
        if track_map.values().any(|&v| v == &complement) && track_map.get (&complement)!= Some(&i) {
            [i, track_map.get]
        } else{
            []
        }
    }
    
}