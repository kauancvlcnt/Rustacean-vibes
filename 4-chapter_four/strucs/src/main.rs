#[derive(Debug)]
struct User {
    name: String,
    lastname: String,
}

//uma função associada :: retorna o tipo que implementa
impl User {
    //método que mostra o nome completo do usuário
    fn fullname(&self) -> String {
        let name = String::from(&self.name);
        let lastname = " ".to_owned() + &self.lastname;
        name + &lastname
    }
}
fn main() {
    let user1 = User {
        name: String::from("Lua"),
        lastname: String::from("Land"),
    };

    let user2 = User {
        name: String::from("Jhonson"),
        lastname: String::from("Boris"),
    };
    
    println!("nome completo dos usuários: {:#?}", (user1. fullname(), user2.fullname()));
}
