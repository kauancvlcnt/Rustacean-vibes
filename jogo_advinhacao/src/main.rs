extern crate rand;
//importa a dependencia rand
use std::cmp::Ordering;
use std::io;
// traz a trait Rng para o escopo do programa
use rand::Rng;
fn main() {
    println!("Adivinhe o número    !");
    let numero_secreto = rand::thread_rng().gen_range(1, 101); //inclui o inferior e exclui o superior
    println!("Digite o seu palpite:");  //  Macros são escritas com ! no final.
    // se fosse uma função não teria !, teria apenas func()
    loop {
        let mut palpite = String::new();
        // String é um tipo fornecido pela biblioteca padrão que representa
        // uma cadeia expansível de caracteres codificados em utf8
        // :: indica que new() é uma função associada do tipo String
        // uma função associada implementa sobre um tipo
        // String::new() cria uma nova String vazia

        // Para resumir, a linha let mut palpite = String::new();
        // criou uma variável mutável que está atualmente vinculada
        //  a uma nova instância vazia de uma String.
        io::stdin()
            .read_line(&mut palpite) // & segnifica que é referência, elas também são imutáveis por padrão, por isso o &mut
            .expect("Falha ao conectar"); //retorna um Result : Ok ou Err; Se o retorno for Err, encerra o programa com a mensagem deixada ao expect
                                          // palpite sombreado (shadowing); geralmente usado quando convertemos um valor em outro
                                          // parse converte uma String para um tipo numérico
        let palpite: u32 = match palpite.trim().parse() { //o match  é  "tipo um if"
            Ok(num) => num, //O método parse retorna um valor do tipo Result, se o Result for Ok, ele só retornará o número
            Err(_) => continue, //Caso seja um Err ele só vai ignorar e seguir para a a próxima interação do loop. o (_) é um valor "pega tudo", funciona basicamente igual um else

            }; //quando vamos fazer o parse devemos definir o tipo manualmente, nesse caso u32
            // se a String tiver caracteres que não podem ser convertidos para número, o parse() retorna um Result Err()

        // match é para comparar todos os elementos do bloco
        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto"),
            Ordering::Equal => {
                println!("Muito bem, você acertou");
                break;
            }
        }
    }
    // Sai do programa e faz o dropp
}
