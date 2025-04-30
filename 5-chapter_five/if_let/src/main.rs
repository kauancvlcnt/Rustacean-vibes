fn main() {
    let algum_valor_u8 = Some(3u8); //u8 na frente indica que é um u8

    //trata None, ou Some ()
    //É obrigatório tratar possiveis None's e Some's
    match algum_valor_u8 {
        Some(3) => println!("três"),
        _ => (),
    }

        //"trata apenas um dos casos de uma vez", ou Some() ou None
    
        //tratando Some 
        //isso funciona da mesma forma que um match, só que sem verificação exaustiva
        if let Some(3) = algum_valor_u8 {
            println!("três");

        }else {
            //podemos usar o else do bloco if let como o _ do match
            println!("NÃO é 3");
        }
        // Tratando None
        if let None = algum_valor_u8  {
            println!("None");
        }

    let teste = 1u8; // 1 é do tipo u8 agora
    //funciona da mesma maneira que o match
    //Ele é usado para não fazer verificação exaustiva
    //E para diminuir o código, implemente caso não precise de varias  verificações
    //para casar
    if let 1 = teste {
        println!("SIM, É UM 1");
    }else {
        //podemos incluir um else em um if let, ele fica equivalente ao pega tudo _
        //do match
        println!("Não é um 1")    
    }        


}
