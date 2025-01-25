#[allow(unused_variables)]


fn main(){
    let some_bool = true;
    let num = 1700;

    if (num%4==0 || num%400==0) && num%100 != 0 {
        println!("{} is leapyear", num);
    } else {
        println!("not a leapyear");
    }
    let just_int = 2;

    match just_int {
        0 => println!("zero"),
        1 => println!("one"),
        None => println!("None"),

    } 

}