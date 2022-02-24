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
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click{x:i64,y:i64}
}

enum Option<T>{
    Some(T),//Here T is generic type parameter
    None
}

fn main(){
    let quit:WebEvent = WebEvent::KeyPress('q');
    let click = WebEvent::Click{2,3};

    let something = Some(1);
}

 