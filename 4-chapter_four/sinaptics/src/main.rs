//o ultimo nome é o que sempre usamos lá embaixo para se referir a algum método
use rand;
use std::io::stdin;
#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    lastname: String,
    age: String,
    balance: u64,
    is_admin: bool,
}

impl User {
    fn make_user(name: String, lastname: String, age: String, is_admin: bool) -> User {
        if is_admin {

            User {
                id: rand::random_range(1000..1999),
                name,
                lastname,
                age,
                balance: 10000,
                is_admin,
            }
        }else {
            User {
                id: rand::random_range(1000..1999),
                name,
                lastname,
                age,
                balance: 0,
                is_admin,
            }
        }
    }
}
impl User {
    fn add_balance(&self, value: u64, user: &mut User) {
        //Quem vai colocar saldo, quantidade de saldo, usuário pra colocar saldo
        println!("{:?}", self);
        if self.is_admin {
            user.balance = value;
            
        }
    }
}

fn main() {
    let (mut name, mut lastname, mut age, mut is_admin) =
        (String::new(), String::new(), String::new(), String::new());
    print!("\x1B[2J\x1B[1;1H");
    println!("Your name");
    stdin().read_line(&mut name).expect("error");
    print!("\x1B[2J\x1B[1;1H");
    println!("Your lastname ");
    stdin().read_line(&mut lastname).expect("error");
    print!("\x1B[2J\x1B[1;1H");
    println!("Your age");
    stdin().read_line(&mut age).expect("erro");
    print!("\x1B[2J\x1B[1;1H");
    println!("You access for admin");
    stdin().read_line(&mut is_admin).expect("erro");

    let is_admin = if is_admin.trim() == "true" {
        true
    } else {
        false
    };

    let mut user1 = User::make_user(
        String::from(name.trim()),
        String::from(lastname.trim()),
        String::from(age.trim()),
        is_admin,
    ); //transferiu a posse de todas as strings para make_user

    let mut user2 = User::make_user(
        String::from("Lucas"),
        String::from("Julio"),
        String::from("31"),
        false,
    ); //transferiu a posse de todas as strings para make_user

    user1.add_balance(30, &mut user2);

    println!(
        "Olá {} {}  your  id {} and balance is {}",
        user2.name.to_uppercase(),
        user2.lastname.to_uppercase(),
        user2.id,
        user2.balance
    );

    println!("BALANCE ADMIN {}", user1.balance);
}
