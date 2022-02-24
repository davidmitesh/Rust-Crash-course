pub fn run(){
    greeting("hello", "Mitesh");

    //bind function value to variable

    let get_sum = add(23, 42);
    
    println!("Sum : {}",get_sum);

    //Closure
    //nice and compact and we can also use outside variables
    let n3 =50;
    let add_nums = |n1:i32,n2:i32| n1+n2+n3+10;
    println!("Closure sum is : {}",add_nums(3,3));
}


fn greeting(greet:&str,name:&str){
    println!("{} {},nice to meet you!",greet,name);
}

fn add (n1:i32,n2:i32)-> i32 {
    n1+n2
}