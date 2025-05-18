// Result é uma enum

// enum Result {
//     Ok(T), T é um parâmetro genérico
//      T representa o tipo do valor que será retornado dentro da variante Ok
//     Err(E), E é um parâmtro genérico
//      E representa o tipo de erro que será retornado dentro variante Err e caso de falha
// }

use std::{
    fs::File,
    io::{ErrorKind, Read},
};

fn main() {
    //como saber se a função retorna um Result? podemos dar uma anotação de tipo
    //que sabemos não ser o tipo retornado pela função, e tentar compilar o código
    //o compilador dirá que os tipos não casam. A mensagem de erro vai então nos dizer
    //qual é, de fato, o tipo de f.
    // ex: demos u32 e ele encontrou uma enum(que no caso é a Result)
    // let f: u32 = File::open("./hello.txt");
    let f = File::open("hello.txt");

    let f = match f {
        //tratando um Result com match
        Ok(file) => file,
        // o if é um match guard, é uma condição extra que posteriormente refina o braço match
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            // Err retorna uma struct que tem dentro dela um kind, o tipo dele é ErrorKind,
            // uma Enum que tem vários tipos de
            Ok(file_create) => file_create,
            Err(error) => {
                panic!("Tentou criar o arquivo e houve um problema: {:?}", error)
            }
        },
        Err(error) => {
            panic!("Houve um problema ao abrir o arquivo {:?}", error);
        }
    };

    println!("{:?}", f);
}
