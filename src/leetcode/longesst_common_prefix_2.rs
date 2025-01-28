struct Solution;

impl Solution{
    pub fn longest_common_prefix(strs:Vec<String>)->String{
        if strs.is_empty(){
            return String::new();
        }
        let mut output = strs[0].clone();
        for single_string in strs.iter().skip(1){
            while !single_string.starts_with(&output){
                output.pop();
            }
        }
        return output.to_string();
    }
}

fn main() {
    // Example input
    let strs = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];

    // Call the function
    let result = Solution::longest_common_prefix(strs);

    // Print the result
    println!("The longest common prefix is: {}", result);

    // Test empty input
    let empty_strs = vec![];
    let empty_result = Solution::longest_common_prefix(empty_strs);
    println!("The longest common prefix for empty input is: '{}'", empty_result);
}