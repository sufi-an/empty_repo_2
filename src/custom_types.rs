
// #![allow(dead_code)]

// #![derive(Debug)]
struct Person{
    name: String,
    age: u8,
}

pub fn custom_types() {
    let name:String = String::from("Sufian");
    let age: u8 = 26;

    let sufian = Person{name,age};

    println!("His name is: {}, his age is: {}",sufian.name,sufian.age);
}