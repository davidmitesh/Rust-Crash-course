#[derive(Debug)]
enum MyError{
    Error1
}


//Result has two types ,
//Err, an enum that contains the error code
//Ok(value), A wrapper that contains the value
fn divide(dividend:i32,divisor:i32) -> Result<i32,MyError>{
    if dividend % divisor != 0 {
        Err(MyError::Error1)
    }else{
        Ok(dividend/divisor)
    }
}


pub fn run(){
    let divide1 = divide(4, 2);
    let divide2 = divide(2, 3);

    match &divide1{
        Ok(v) => println!("{}",v),
        Err(v) => println!("{:?}",v)
    }


    println!("{:?}",&divide2.unwrap());


    if divide1.is_ok(){
        println!("{}",divide1.unwrap())
    }
   
    //How to resolve this error. I need to find out.
    println!("{:?}",divide2.unwrap_or(4));

}