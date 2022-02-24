
//This is important because data is passed in solana in form of slices of input.
pub fn run(){
    let arr = [0,1,2,3,4,5];
    let slice = &arr[0 .. 4];
    //result is [0,1,2,3]
    
    // println!("{:?}",slice);
    borrowing_slice(arr,slice);
}


fn borrowing_slice(arr:[u8;6],slice:&[u8]){
    println!("{:?}",arr);
    println!("length:{:?}",slice);
}