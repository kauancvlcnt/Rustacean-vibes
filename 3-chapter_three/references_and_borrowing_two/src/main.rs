fn main() {
    let mut s = String::from("texto"); // tornando variável mutável para que a referência também possa ser
    modifica(&mut s); //tornando a referência mutável

    println!("{s}");
    ref_mutaveis_escopos_diferentes();
    ref_mutaveis_e_imutaveis_combinacao();
}

fn modifica(uma_string: &mut String) {
    //&mut aqui quer dizer que o parâmetro aceita referências mutáveis
    //referências também são imutáveis por padrão
    //devemos usar mut na variável e nas referência, para que ela possa ser alterada
    uma_string.push_str(" mais texto alterado");

    //observe que s1 foi alterada dentro do escopo dessa função, porém ela será alterada
    // no outro escopo, já que ela foi só alterada aqui, não  teve tranferência nenhuma de ownership
}

fn ref_mutaveis_escopos_diferentes() {
    /*     referencias mutáveis
         Referências mutáveis possuem uma grande restrição: você só pode ter uma
          referência mutável para um determinado dado em um determinado escopo. Este código vai dar erro

         let mut s_new = String::from("text");

         let r1 = &mut s_new;
         let r2 = &mut s_new; //segundo empréstimo mutável/imutável não pode

         println!("{} {}", r1, r2);

     isso permite que a mutabilidade seja feita de maneira controlada para impedir DATA RACES
    em tempo de compilação

     Um data race é parecido com uma condição de corrida, e acontece quando esses três fatores ocorrem:

        Dois ou mais ponteiros acessam o mesmo dado ao mesmo tempo.
        Ao menos um dos ponteiros é usado para escrever sobre o dado.
        Não há nenhum mecanismo sendo usado para sincronizar o acesso ao dado.


        Data races causam comportamento indefinido e pode ser difíceis de diagnosticar e corrigir quando você está tentando
        rastreá-los em tempo de execução. Rust previne este problema de acontecer porque não vai nem deixar compilar um código com data races!
    */

    // podemos usar chaves para criar um novo escopo, permitindo múltiplas referências de mutáveis
    // mas não simultâneas

    let mut s1 = String::from("OI, EU SOU UMA STRING:");

    // podemos agora ter multiplas referências mutáveis, mas elas devem estar em escopos diferentes
    {
        let ref1 = &mut s1;
        ref1.push_str(" push ref modificação 1;");
    } //ref1 faz drop, já podemos ter outra referência mutável
    let ref2 = &mut s1;
    ref2.push_str(" push ref modificação 2");

    println!("{}", s1);
}

fn ref_mutaveis_e_imutaveis_combinacao() {
    let mut s = String::from("TEXTOoooo");

    {
        let ref3 = &mut s; //PROBLEMA resolvido apenas colocando a referencia mutável em outro
                                        //escopo diferente das referencias imutáveis
        
        println!("{ref3}");
        
    }
    let ref1 = &s; //sem problema 
    let ref2 = &s; //sem problema 
    let ref4 = &s; //sem problema
    println!("{ref1}, {ref2}, {ref4}");
}


/*Vamos recapitular o que discutimos sobre referências:

    Em um dado momento, você pode ter um ou outro, mas não os dois:

    Uma referência mutável por escopo.
    Qualquer número de referências imutáveis por escopo.

    Referências devem ser válidas sempre.

*/