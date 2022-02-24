//Tuples group together values of different types
//Max 12 elements
//i8 is a 8-bit number 0-255
pub fn run(){
    let person: (&str,&str,i8) = ("Mitesh","Brad",12);

    println!("{} is from {} and is {} ",person.0,person.1,person.2);
}