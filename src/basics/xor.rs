//swap code


fn main(){
    let mut num_1 = 33;
    let mut num_2 = 77;

    num_1 = num_1 ^ num_2;
    num_2 = num_1 ^ num_2;
    num_1 = num_1 ^ num_2;

    println!("num1 {}, num2 {}", num_1, num_2);
}