struct Rectangle {
    width: u32,
    height: u32,
}
// Métodos são diferentes das funções, porque são definidos no contexto de uma
//  struct (ou um objeto enum ou uma trait, que nós cobrimos nos Capítulos 6 e 17,
//  respectivamente), o seu primeiro parâmetro é sempre self, que representa a 
// instância da struct do método que está a ser chamado.



//implementa pra tudo que é do tipo rectangle, todo rectangle poderá usar esse método
impl Rectangle {
    //deverá receber um rectangle
    fn area(&self) -> u32 {
    //self representa o rectangle que está sendo passado self.width é rectangle.width
    //Rust já sabe que no contexto da impl Rectangle self é o Tipo Rectangle
    //self é a mesma coisa que o parâmetro:( rectangle: &Rectangle )
    //area pega um empréstimo do rectangle passado e acessa os atributos
        println!("{}", self.width * self.height);
        self.width * self.height
    }
    // A principal vantagem do uso de métodos em vez de funções, além de usar a 
    // sintaxe do método e não ter de repetir o tipo de self em cada assinatura 
    // do método, é a organização. Nós colocamos todas as coisas que nós podemos
    //  fazer com uma instância de um tipo em um bloco impl em vez de fazer os
    //  futuros utilizadores do nosso código procurar as capacidades de Rectangle
    //  em vários lugares na biblioteca que fornecemos.
}

fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 30,
    };

    rect1.area();
}


