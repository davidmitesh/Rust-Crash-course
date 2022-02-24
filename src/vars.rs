//Variables hold primitive data or references to data
//Variables are immutable by default
//Rust is a block-scoped language

pub fn run(){
    let name = "mitesh";
    println!("my name is {}",name);

    //defining multiple variables

    let (my_name,my_age ) = ("mitesh",23);
    println!("{} is {} years old. ",my_name,my_age);
}