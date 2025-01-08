#[allow(unused_variables)]

fn main(){
    let string_slice : &str = "'Kashem'";
    let just_string : String = String::from("Kaua@a@");
    let last_string : String = String::from("Kader");
    let full_string : String = [&just_string," ",  &last_string].concat();// concating two string
    println!("{}", full_string);
    println!("{}", just_string);
    let sub_string : &str = &just_string[2..4];// creating a substring, not including the max value, it is up to value
    println!("{}", sub_string);
    
    // let mut char_by_index: char  = &just_string.chars().nth(0);
    // println!("nth char {}", char_by_index);

    // match char_by_index{
    //     Some (c) => println!("{}",c)
    //     None => {}
    // }
    if let Some (c) = just_string.chars().nth(2){
        println!("{}",c)
    }

    let convert_to_string = string_slice.to_string(); // convert str slice to string
    let convert_slice : &str = &convert_to_string; //convert string to str slice

    println!("{}", string_slice);
}