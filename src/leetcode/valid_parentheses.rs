use std::io;
struct Solution;

impl Solution{
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        
        for c in s.chars(){
            if c =='{' || c=='(' || c=='['{
                stack.push(c);
            }

            if c =='}' || c==')' || c==']'{
                

            }
            
            if c=='}'{
                if stack.last() ==Some(&'{'){
                    stack.pop();
                }
                else{
                return false;
                }
            } 
            if c==')'{
                if stack.last() ==Some(&'('){
                    stack.pop();
                }
                else{
                return false;
                }
            }
            if c==']'{
                if stack.last() ==Some(&'['){
                    stack.pop();
                }
                else{
                return false;
                }
            }
        }

        if stack.is_empty(){
            true
        }else{
            false
        }
    }
}

fn main(){
    let mut par:String = String::new();
    par = "()]";
    let mut result = is_valid(par);
    println!("{}", result);
}