#[derive(Debug)]
struct User {
    username: String,
    emails: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {
    let mut user1 = User {
        username: String::from("joão"),
        emails: String::from("jõaocandido@gmail.com"),
        sign_in_count: 3,
        active: true,
    };

    user1.username = String::from("juquinha");

    println!("{}", user1.username);
    println!("{}", user1.emails);
    println!("{}", user1.sign_in_count);
    println!(
        "{}
  ",
        user1.active
    );

    println!("{:#?}", user1);

    // você pode usar o field init shorthand ((inicialização abreviada do campo)
    // Se você tiver as variáveis com os mesmos nomes dos campos da struct 



    // struct update  sintax
    //podemos atualizar uma struct usando a sintax .. (funciona como o operador rest)

    let user2 = User {
        emails:String::from("justwait@gmail.com"),
        ..user1 //isso "copiará" os outros campos da struct, atualizando apenas o "emails"
    };





}
