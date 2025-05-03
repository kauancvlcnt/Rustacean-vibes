extern crate comunication;
pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

// podemos fazer a mesma coisa com uma enum
enum TraficLigth {
    //namespace TraficLigth
    Red,
    Yelow,
    Green,
}

enum Dia {
    Noite,
    Manha,
    Tarde,
}
use Dia::*; //traz tudo que tiver no namespace da enum para esse escopo, esse * é um Glob
use TraficLigth::{Red, Yelow}; //Traz Red e Green para esse escopo
use a::series::of; //use pode ser usado para encurtar o caminho
fn main() {
    of::nested_modules();
    let red = Red;
    let yelow = Yelow;
    let green = TraficLigth::Green; //especificamos o namespace pq green não está no use
}
