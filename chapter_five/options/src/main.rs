enum Cor {
    Preto,
    Branco,
    Amarelo,
}

enum Olhos {
    Azuis,
    Castanhos,
    Verdes,
}


enum Gato {
    Cor(Cor),
    Olhos
}




fn main() {
    let cat1 = Gato::Cor(Cor::Preto);
    let is_black = e_gato_preto(&cat1);
    if is_black.is_some() {
        println!("Miaaaooouu de gato preto")
    }else {
        println!("Não sou um gato preto")
    }

}

fn e_gato_preto(cat: &Gato) -> Option<bool>{

    match  cat {
        Gato::Cor(Cor::Preto) => Some(true),
        _ => None,

    }
}
