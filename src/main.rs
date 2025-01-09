use std::array;

fn main(){
    let some_array = [1,2,3,4,5,6,7,8];
    let mut sum = 0;
    for &value in some_array.iter(){
        sum+=value;
    }
    println!("{:?}", some_array);
    println!("{ }",sum)


}