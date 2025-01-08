use std::i8;

fn main() {
    let x : i8 = 30;
    let x: i8 = x+5;
    {
        let x = x*(2);
        println!("value of x is {}", x);
    }

    println!("value of x is {}", x);
}
