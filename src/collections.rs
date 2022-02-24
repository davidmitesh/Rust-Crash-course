pub fn run(){
    let mut students = vec![Student{name:
        "Ryan"
    }];

    students.push(Student{
        name:"Lisa"
    });
    
    assert_eq!(&students[0],)
}

#[derive(PartialEq,Eq)]
struct Student{
    name:String,
}