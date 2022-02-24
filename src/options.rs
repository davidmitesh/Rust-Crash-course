fn divide(dividend:i32,divisor:i32) -> Option<i32>{
    if dividend % divisor != 0 {
        None
    }else{
        Some(dividend/divisor)
    }
}


pub fn run(){
    let divide1 = divide(4, 2);
    let divide2 = divide(2, 3);

    println!("{:?} unwraps to {}",divide1,divide1.unwrap()); // Successfully executes

    println!("{:?} unwraps to {}",divide2,divide2.unwrap()); //This panics because unwrap panics when it encounters none

}