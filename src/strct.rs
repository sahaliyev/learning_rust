#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    gender: String
}

fn main() {
    let sahil = Person{
        name: String::from("Sahil"),
        age: 27,
        gender: String::from("male")
    };
    
    let age = &sahil.age;
    let name = &sahil.name;
    let gender = &sahil.gender;
    println!("Person name is {} age is {} gender is {}", name, age, gender);

    // update age

    let sahil_new = Person{
        age: 26,
        ..sahil
    };

    println!("Person name is {} age is {} gender is {}", 
    sahil_new.name, sahil_new.age, sahil_new.gender);

    
}