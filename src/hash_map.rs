// #[derive(Debug)]
use std::collections::HashMap;

pub fn run(){
    let mut map: HashMap<&str,&str> = HashMap::new();
    // map.insert("mitesh", (maths: 80,"science":90));
    // map.insert("ganesh",{"maths":90,"science":100});
    // map.insert("himal",{"maths":70,"science":99});
    map.insert("mitesh","very kind");
    map.insert("ganesh","very handsome");
    map.insert("hari","very compassionate");

    get_map_values("ganesh", &map);
    let key = "himal";
    let value = "some dummy";
    println!("{:?}",insert_new(key,& mut map,value));
    // println!("{:?}",map);
}

fn get_map_values(key : &str, mapping :  &HashMap<&str,&str>){
    match mapping.get(key){
        Some(result) => println!("{:?}",result),
        None => println!("The passed key doesnot exist in the mapping")
    }
}

fn insert_new<'a>(key : &'a str,mapping :&'a mut HashMap<&'a str,&'a str>,value:&'a str)-> &'a mut HashMap<&'a str,&'a str>{
    mapping.insert(key, value);
    mapping
}