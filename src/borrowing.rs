//Rules of borrowing

//1. At any given time,you can have either:
////a. One mutable reference,
/// b. Any number of immutable references
//2. References must always be valid.

pub fn run(){
    let mut x_vec = vec![1,2,4];
    println!("Original Vector : {:?}",x_vec);
    add_to_vec(&mut x_vec);
    println!("Original Vector : {:?}",x_vec);
}

fn add_to_vec(a_vec: &mut Vec<i32>){
    a_vec.push(4);
}