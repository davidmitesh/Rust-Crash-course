//An example of dangling pointer

// pub fn run(){
//     let r:&i32;
//     {
//         let x:i32 =5 ;
//         r = &x;//In this block, x is borrowed to r, but it is dropped to this block, but is referenced outside the block. So, r is dangling pointer outside the scope and rust tries to eliminate such.
//     }
//     println!("r:{}",r);
// }


//another example of lifetime in rust

pub fn run(){
    let string1:String = String::from("mitesh");
    let result: &str ;
    {
        let string2: String = String::from("pandey");
        result = longest(&string1, &string2);

        //When result is used in this scope, it works perfectly fine.
        println!("The longest string is:{}",result);
    }
    //Using the result here gives the error because, string2 only lives inside the block, so result is only valid inside the block.
    println!("The longest string is:{}",result);
}

fn longest<'a>(x: &'a str,y: &'a str) -> &'a str{ //What this generic lifetime annotations is denoting is that this function will return the lifetime of the shortest argument passed as the parameter.
    //It is just defining the ambiguious behavior, not to be confused with changing the lifetime.
    if x.len() > y.len() {
        x
    }else{
        y
    }
}