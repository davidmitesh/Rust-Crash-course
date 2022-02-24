pub fn run(){
    let mut hello = String::from("Hello ");
    hello.push_str("World");
    // println!("Length: {} ",hello.len());
    // hello.push('a');

    // println!("{} ",hello);

    //loop through string by whitespace

    for word in hello.split_whitespace(){
        println!("{}",word);
    }

    //Create string with capacity

    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('b');

    //assertion testing

    assert_eq!(2,s.len());
    assert_eq!(10,s.capacity());
    println!("{}",s);
}