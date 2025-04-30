// enum Estado {
//     Alabama,
//     Alaska,
//     Michigan,
// }


// enum Moeda {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(Estado),
// }

// match É parecido a uma expressão utilizada com if, mas tem uma grande diferença:
//  com if, a expressão precisa retornar um valor booleano. Aqui, pode ser de
//  qualquer tipo. O tipo da variável moeda, neste exemplo, é a enum Moeda,
// fn main() {
// //     let moeda = Moeda::Quarter;
// //    let moeda = match moeda {
// //         Moeda::Penny => 1, //braço 1 todos os braços são expressões 
// //         Moeda::Nickel => 5, //braço 2
// //         Moeda::Dime => 10, //braço 3
// //         Moeda::Quarter(estado) => {
// //             println!("{}", estado);
// //         //Se o braço for longo e tiver mais de 1 linha de cod, podemos usar
// //         //chaves para delimitar
// //         //podemos fazer o que quisermos aqui                                                        
// //             qua(1)
// //         }, //braço 4
// //     };

// //     println!("moeda value {}", moeda);



// }

fn main() {
#[derive(Debug)]
enum Estado {
   Alabama,
   Alaska,
}

enum Moeda {
   Penny,
   Nickel,
   Dime,
   Quarter(Estado),
}


fn valor_em_cents(moeda: Moeda) -> u32 {
    match moeda {
        Moeda::Penny => 1,
        Moeda::Nickel => 5,
        Moeda::Dime => 10,
        Moeda::Quarter(estado) => { //estado é o Estado::Alabama associado a
            //Quarter
            println!("Quarter do estado {:?}!", estado);
            25
        },
    }
}


valor_em_cents(Moeda::Quarter(Estado::Alabama));
}



fn qua(quarter: i32) -> i32 {
    12  * 2 + quarter
}