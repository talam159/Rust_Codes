use std::collections::HashMap;
struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut sym = HashMap::new();
        sym.insert(b'I', 1);
        sym.insert(b'V', 5);
        sym.insert(b'X', 10);
        sym.insert(b'L', 50);
        sym.insert(b'C', 100);
        sym.insert(b'D', 500);
        sym.insert(b'M', 1000);

        let mut result = 0;
        let s_bytes = s.as_bytes(); // Convert to byte slice for indexing

        for i in 0..s_bytes.len() {
            let cur = *sym.get(&s_bytes[i]).unwrap();//unwrap extracts values safely, it's like picking one by one
            if i < s_bytes.len() - 1 {//checking so that we do not address out of array or s_byte
                let next = *sym.get(&s_bytes[i + 1]).unwrap();
                if cur < next {
                    result -= cur; // Subtractive case
                } else {
                    result += cur;
                }
            } else {
                result += cur; // Add the last character value
            }
        }
        
        result
    }
}
fn main() {
    let result = Solution::roman_to_int("MCMXCIV".to_string());
    println!("{}", result);  // Output: 1994
}
