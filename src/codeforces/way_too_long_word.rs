use std::io;

fn main(){
    let mut word = String::new();

    // let mut too_long_word= String::new();

    io::stdin()
        .read_line(&mut word)
        .expect("failed to read line");


    // first = let cur = *sym.get(&s_bytes[i]).unwrap();


if word.len()>=10{
    println!("{}",word);
    let length= word.len();
    let first = word.chars().nth(0).unwrap();
    let w_last = word.chars().last().unwrap();
    let middle_length= word.len()-2;

    println!("{}, {}, {}",&first, middle_length, w_last);
    word = format!("{}{}{}",first, middle_length, w_last);
}

    

println!("{}",word);
}