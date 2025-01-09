#[allow(unused_variables)]

fn main(){
    let returned_data = some_function(2.2,50);
    println!("returned data is {}", returned_data);
    some_procedure(5.0, 96);
}

fn some_procedure(param_a:f32, param_b: i8){
    println!("Procedure is a function without returning anything");
}

fn some_function(param_a:f32, param_b:i32) -> f32{
    println!("I am a function");
    if param_a<100.{
        let return_var =10.1 *  param_a + param_b as f32;

        return_var
    }
    else{
        -1.
    }

     //no semicolon means we want to return this data



}