//Tuples group together values of different types
//Max 12 elements
//i8 is a 8-bit number 0-255
pub fn run(){
    let person: (&str,&str,i8,f32) = ("Mitesh","Brad",12,2.1);

    //we can destructure the tuple as well
    let (a,b,c,d) = person;

    println!("{} is from {} and is {} ",person.0,person.1,person.2);
    println!("Printing the whole tuple : {:?}",person);
    println!("{},{},{},{}",a,b,c,d);
}