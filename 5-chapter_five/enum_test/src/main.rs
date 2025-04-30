#[derive(Debug)]

enum Hair {
    Curto,
    Comprido,
}
#[derive(Debug)]
enum Eyes {
    Claros,
    Castanhos,
    Verdes,
}
#[derive(Debug)]
enum Clothes {
    Shorts,
    Calca,
    Blusa,
    Camiseta,
}
#[derive(Debug)]
struct Woman {
    cabelo: Hair,
    olhos: Eyes,
    roupa_cima: Clothes,
    roupa_baixo: Clothes,
}
#[derive(Debug)]
struct Man {
    cabelo: Hair,
    olhos: Eyes,
    roupa_cima: Clothes,
    roupa_baixo: Clothes,
}
#[derive(Debug)]
enum Pessoa {
    Woman(Woman),
    Man(Man),
}

fn main() {
    let mulher = Pessoa::Woman(Woman {
        cabelo: Hair::Comprido,
        olhos: Eyes::Verdes,
        roupa_baixo: Clothes::Shorts,
        roupa_cima: Clothes::Camiseta,
    });
    let homem = Pessoa::Man(Man {
        cabelo: Hair::Curto,
        olhos: Eyes::Verdes,
        roupa_cima: Clothes::Camiseta,
        roupa_baixo: Clothes::Calca,
    });

    println!("{:#?}", homem);
}
