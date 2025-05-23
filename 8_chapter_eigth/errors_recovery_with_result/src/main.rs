// Result é uma enum

// enum Result {
//     Ok(T), T é um parâmetro genérico
//      T representa o tipo do valor que será retornado dentro da variante Ok
//     Err(E), E é um parâmetro genérico
//      E representa o tipo de erro que será retornado dentro variante Err em caso de falha
// }

use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn main() {
    //como saber se a função retorna um Result? podemos dar uma anotação de tipo
    //que sabemos não ser o tipo retornado pela função, e tentar compilar o código
    //o compilador dirá que os tipos não casam. A mensagem de erro vai então nos dizer
    //qual é, de fato, o tipo de f.
    // ex: demos u32 e ele encontrou uma enum(que no caso é a Result)
    // let f: u32 = File::open("./hello.txt");
    let f = File::open("hello.txt");
    let filee = File::open("hello.txt").unwrap(); //unwrap é um método que já faz o match e retorna Err, ou Ok
    let filee1 = File::open("hello.txt").expect("Falhou ao abrir hello.txt"); //expect funciona da mesma maneira do
    //unwrap, a diferença é que podemos deixar uma mensagem caso o Result seja um Err

    let f = match f {
        //tratando um Result com match
        //tratando com diferentes erros
        Ok(file) => file,
        // o if é um match guard, é uma condição extra que posteriormente refina o braço match
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            // Err retorna uma struct que tem dentro dela um kind, o tipo dele é ErrorKind,
            // uma Enum que tem vários tipos de opções de erro que podem acontecer em uma operação de io
            Ok(file_create) => file_create,
            Err(error) => {
                panic!("Tentou criar o arquivo e houve um problema: {:?}", error)
            }
        },
        Err(error) => {
            panic!("Houve um problema ao abrir o arquivo {:?}", error);
        }
    };

    println!("f: {:?} filee: {:?}", f, filee);
    println!("{:?}", read_username_from_file());
}

fn _read_username_from_file() -> Result<String, io::Error> {
    // quando escrevemos uma função e essa função chama algo que pode falhar, em vez de tratar o erro dentro dessa
    // função, você pode retornar o erro ao código que chamou de forma que ele possa decidir o que fazer
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // String
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e), //Retorna io::Error
    } //Retorna um Erro de io caso exista

    //vai ser retornado uma String ou um io::Error
    // Escolhemos io::Error como o tipo de retorno dessa função porque é este o tipo
    // de erro retornado pelas duas operações que estamos chamando no corpo dessa função
    //  que podem falhar: a função File::open e o método read_to_string.
}

fn read_username_from_file() -> Result<String, io::Error> {
    // Esse padrão de propagação de erros é tão comum em Rust que a linguagem disponibiliza
    //  o operador de interrogação ? para tornar isso mais fácil.
    // ?  funciona como um match que retorna um Result Ok ou Err
    let mut f = File::open("hello.txt")?; // ? Retorna uma String ou um io::Error
    //também podemos encadear métodos usando "?"
    // File::open("hello.txt")?.read_to_string(&mut f)?;

    let mut s = String::new();
    f.read_to_string(&mut s)?; // ? Retorna uma String ou um io::Error
    Ok(s) //retorna quando ambos os métodos open() e read_to_string() dão certo

    // ? só pode ser usado em funções que tem um tipo de retorno Result, porque está definido
    // a funcionar da mesma maneira que a expressão match que chama handlers com o return Err(e)

    // "?" retorna um Result, então não podemos utilizar ele em lugares onde retorna outro tipo
    //Se usarmos ele na função main, retornará um erro, já que main() retorna ()
}
