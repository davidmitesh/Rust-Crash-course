//Ownership is related to how your program manages memory
//Rust stores the data in two differently structures part of the memory
//1. Stack (LIFO) - stores data with known fixed size
//2. Heap - for mutable data however like vectors in rust, the pointer to the values that are in heap is stored in stack.

//If any data is mutable means that it is stored both in stack and the heap.

//So what happens is if there are two pointers on stack that point to same memory location in heap, and if one of them is dropped then the other pointer 
//now becomes the dangling pointer because the memory location it is pointing to the heap is no more.So, this is handled by different programming
//languages in different ways.

//In languages like :

//1: c/c++ -- there is the concept where it is the programmer's responsiblity to manage the memory
//2: js/ruby/python -- there is the concept of garbage collector.cleans up references to the memory that doesnot exist

//But Rust takes different approach than this to manage memory.It introduces the concept of ownership. Here it is checked in the compile time.Set of rules.

//Rules of ownership:

//1. Each value in Rust has a variable that is the owner of the variable and called owner
//2. There can only be one owner of a value at a time
//3. When the owner goes out of scope, the value will be dropped.

//Benefits of this approach:
//1. Dangling pointer
//2. Double free - Trying to free the memory that is already been freed
//3. Memory leaks - Not freeing memory that should be freed


pub fn run(){
    let say = String::from("Cat");
    print_out(say);
    print!("Again:{}",say)//If we do this the we can error because we have passed the ownership to the print_out function of variable "say" and it is not
    //available
}

fn print_out(to_print:String){
    println!("{}",to_print);
}


