use std::io;

fn main(){
    let mut input= String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

        let n: i32 = input.trim().parse().expect("Please enter a valid integer!");

    if n<=2 || (n>2 && n%2!=0){
        println!("NO");
    }
    else{
        println!("YES");
    }
}