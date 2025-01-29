struct Solution;


impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut index;
        for element in nums{
            if element!=value{
                nums[index]=nums[i];
                index=index+1;
            }
        }
        return index;
    }
}

fn main() {
    let mut nums = vec![3, 2, 2, 3, 4, 2];
    let val = 3;
    let result = Solution::remove_element(&mut nums, val);
    println!("New length: {}", result);
    println!("Modified array: {:?}", &nums[0..result as usize]);
}