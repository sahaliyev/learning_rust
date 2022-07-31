#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    gender: String,
    surname: String
}

// implement Person struct to have methods on it
impl Person {
    fn find_bd_year(&self) -> u32{
        let curr_year: u32 = 2022;
        curr_year - dbg!(self.age) // dbg! to debug inline
    }

    fn const_full_name(&self) -> String {
        let mut full_name = String::from("");
        full_name.push_str(&self.name);
        full_name.push_str(" ");
        full_name.push_str(&self.surname);

        full_name
    }
    
}

fn main() {
    let sahil = Person{
        name: String::from("Sahil"),
        age: 27,
        gender: String::from("male"),
        surname: String::from("Aliyev")
    };
    
    let age = &sahil.age;
    let name = &sahil.name;
    let gender = &sahil.gender;
    println!("{:?}", sahil);
    
    println!("Person name is {} age is {} gender is {}", name, age, gender);
    println!("db year is {}", sahil.find_bd_year());

    // update age

    let sahil_new = Person{
        age: 26,
        ..sahil
    };

    println!("Person name is {} age is {} gender is {}", 
    &sahil_new.name, &sahil_new.age, &sahil_new.gender);
    println!("db year is {}", sahil_new.find_bd_year());
    println!("full name is {}", sahil_new.const_full_name());


    
}