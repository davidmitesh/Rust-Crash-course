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