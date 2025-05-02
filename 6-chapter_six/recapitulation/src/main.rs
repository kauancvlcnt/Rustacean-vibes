// enum serve para onde temos mais de uma opção
// option<T> serve para substituir o Null

fn main() {
    // age do tipo Option<i32> recebe None se assemelha com "inválido"
    let mut age: Option<i32> = None;

    //Como dissemos que o Option era do tipo i32, devemos passar um i32 no Some
    //que é um Option
    //só passamos valores para o option pelo Some
    age = Some(12);

    match age {
        Some(age) => {
            if age >= 10 {
                println!("maior ou igual",)
            } else {
                println!("menor")
            }
        }
        None => println!("menor"),
    }
}
