use std::any::Any;

#[derive(Debug)]
struct Dog {
    age: f64,
    name: String,
    bark: bool,
}

impl Dog {
    fn new_dog(age: f64, name: String, bark: bool) -> Dog {
        Dog { age, name, bark }
    }
    fn bark_now(&mut self) {
        if self.bark == false {
            self.bark = true;
        } else {
            println!("{} Já está latindo", self.bark);
        }
    }
}

fn main() {
    // Dog::new_dog é uma função associada a Dog, um método é relacionado as instâncias
    let mut amancio = Dog::new_dog(0.2, String::from("Amâncio"), false);
    amancio.bark_now();
    println!("{:#?}", amancio);

    let dogs: [String; 4] = [
        String::from("Amâncio"),
        String::from("Maccalan"),
        String::from("Jack"),
        String::from("Biu"),
    ];

    let mut vect:Vec<Dog> = Vec::new(); 
    for i in dogs {
        let mut dog = Dog::new_dog(0.0, i, false);
        vect.push(dog);
    }

    println!("{:#?}", vect);

    let tes = &vect[1..2];

    println!("{:#?}", tes);
}
