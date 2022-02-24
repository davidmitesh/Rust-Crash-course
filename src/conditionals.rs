pub fn run(){
    //normal if/else

    let age =21;

    if age >22  && 5>2 {
        println!("You got it baby");
    }else {
        println!("Sorry my friend");
    }

    //shorthand if/else

    let is_of_age = if age >=21 {true} else {false};

    println!("Is of age :{}",is_of_age);
}


fn match_use(){
    let i =5;
    match i{
        0 => println!("0"),
        1 | 2 => println!("1,2"),
        3..=4 => println!("3,4"),
        _ => println!("default")
    }
}