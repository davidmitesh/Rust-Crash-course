use std::fs::File;
pub fn run(){
    //Illutrations throeing panic

    // panic!("Farewell");
    // let v = vec![0,1,2,3];
    // println!("{}",v[6]);

    // //option use that returns the some or none
    // let fruits = vec!["banana","apple","mango"];

    // let first = fruits.get(0);
    // println!("{:?}",first);

    // let third = fruits.get(2);
    // println!("{:?}",third);

    // let non_existent = fruits.get(3);
    // println!("{:?}",non_existent);


    //Now implementing the result error handling
    // let f = File::open("hello.txt") ;

    // let f = match f {   
    //     Ok(file)=>file,
    //     Err(error)=>panic!("can't open the file: {:?}",error),
        
    // };

    //The above implementation can also be done using:

    let f = File::open("hello.txt").expect("error occured while opening the hello.txt");


}