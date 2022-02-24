//Arrays : Fixed list where elements are the same data types

use std::mem;
pub fn run(){
    let mut numbers:[i32;5] = [1,2,3,4,5];//fixed size of number 5

    println!("{:?} ",numbers);


    numbers[4] = 10;

    numbers.sort();

    //Get single value

    println!("Single value :{}",numbers[4]);

    //Arrays are stack allocated
    println!("Array occupies {} bytes",mem::size_of_val(&numbers));

    //get slice

    let slice:&[i32] = &numbers[0..4];

    println!("Slice : {:?}",slice);

}