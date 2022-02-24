// //Enums are types which have a few definite values

// enum Movement{
//     //Variants
//     Up,
//     Down,
//     Left,
//     Right
// }

// fn move_avatar(m:Movement){
//     //Perform action depending on the info
//     match m{//similar to switch
//         Movement::Up => println!("Avatar moving up!"),
//         Movement::Down => println!("Avatar moving down!"),
//         Movement::Left => println!("Avatar moving left!"),
//         Movement::Right => println!("Avatar moving right!")
//     }
// }

// pub fn run(){
//     let avatar1 = Movement::Left;
//     let avatar2 = Movement::Up;

//     move_avatar(avatar1);
//     move_avatar(avatar2);
// }


enum WebEvent {
    // PageLoad,
    // PageUnload,
    KeyPress(char),
    // Paste(String),
    Click{x:i64,y:i64}
}

enum Option<T>{
    Some(T),//Here T is generic type parameter
    None
}

pub fn run(){
    let quit:WebEvent = WebEvent::KeyPress('q');
    let click = WebEvent::Click{x:2,y:3};

    let something = Some("mitesh");

    //destructuring the enum values
    if let WebEvent::KeyPress(b) = quit{
        println!("{}",b);
    }

    //destructuring the enum values
    if let WebEvent::Click{x,y} = click {
        println!("The click coordinates are : {} and {}",x,y);
    }

    match something {
        Some(x) => println!("{}",x),
        _ => println!("Nothing really matters")
    }


}

 