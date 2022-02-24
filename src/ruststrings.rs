

//Rust strings are :
//1. UTF-8 encoded
//2. Non-Null-byte Terminated
//3. Not collection of chars

pub fn run(){
    let text = "hello World \n Good morning";
    let upper = text.to_uppercase();
    println!("{}",first_line(&upper));

}

fn first_line(string:&str)-> &str {
    string.lines().next().unwrap()
}