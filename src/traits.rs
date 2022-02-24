pub fn run(){

    let capt_marvel = Film{
        title: String::from("Captain Marvel"),
        director: String::from("Anna Boden and Ryan Flek"),
        studio : String::from("Marvel")
    };

    capt_marvel.describe();

    let harry_porter = Book{
        title:String::from("Harry Porter"),
        author: String::from("JK Rowling"),
        publisher:String::from("Oxford")
    };
    harry_porter.describe();


    let let_it_be = Album{
        title:"let it be".to_string(),
        artist : "Sugam pokhrel".to_string(),
        label:"Pop".to_string()
    };

    let_it_be.describe();
}

struct Film{
    title:String,
    director: String,
    studio: String
}

struct Book{
    title: String,
    author : String,
    publisher : String
}

struct Album{
    title:String,
    artist : String,
    label : String
}

trait Catalog {
    //here implementing the default function for the catalog
    fn describe(&self){
        println!("We need more information about this type of media.");
    }
}

impl Catalog for Film{
    fn describe(&self){
        println!("{} was directed by {} through {} studios.",
        self.title,
        self.director,
        self.studio
    );
    }
}


impl Catalog for Book{
    fn describe(&self){
        println!("{} was written by {} and published by {}",self.title,self.author,self.publisher);
    }
}

impl Catalog for Album{

}

