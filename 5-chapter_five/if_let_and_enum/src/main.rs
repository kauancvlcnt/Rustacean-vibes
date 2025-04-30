use std::f32::DIGITS;


#[derive(Debug)]
enum Estado {
    Alabama,
    California,
}

enum Moeda {
    Dolar(Estado),
}

fn main() {
    let dol = Moeda::Dolar(Estado::California);

    match &dol {
        // O valor agregado em Dolar(Estado::), pode ser comparado ou feito como parãmetro
        Moeda::Dolar(estado) => println!("{:#?}", estado),
        _ => println!("Alabama"),
    }

    // exemplo com if let, se comporta da mesma maniera do 
    // o valor que deve ser feito a comparação deve ficar direito do 
    //se ficar do lado esquerdo, ele terá outro comportamento
    if let Moeda::Dolar(Estado::California) = &dol {
        println!("Califórnia")
    }else {
        println!("Alabama")
    }


    //Tratativa apenas do caso Alabama 
    if let Moeda::Dolar(Estado::Alabama) = &dol {
        println!("É do alabama");
        
    }
}
