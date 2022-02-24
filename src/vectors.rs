//Vectors : They are resizeable arrays


use std::mem;
pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];//fixed size of number 5

    println!("{:?} ",numbers);


    numbers[4] = 10;

    
    numbers.push(40);
    numbers.push(20);



    numbers.sort();
    //Get single value

    println!("Single value :{}",numbers[4]);

    //Vectors are stack allocated
    println!("Vector occupies {} bytes",mem::size_of_val(&numbers));

    //get slice

    // let slice:&[i32] = &numbers[0..];


    //looping through vector values

    for x in numbers.iter(){
        println!("Number : {}",x);
    }

    //Loop and mutate values

    for x in numbers.iter_mut(){
        *x *= 2;
    }

    // println!("Slice : {:?}",slice);

    println!("Numbers vec: {:?} ",numbers);

}