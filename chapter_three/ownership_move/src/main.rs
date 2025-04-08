// Variáveis e interações 
fn main() {
   let x = 5;
   let y = x; // y é uma cópia de x
   //5 é um número inteiro simples e conhecido,então os dois são colocados na stack
   println!("{y}");


   let s1 = String::from("texto");
   let _s2 = s1; //s1 foi movida para s2 (move)
// Dados alocados na heap recebem um nome, ponteiro,tamanho e capacidade. Essas
//  informações são guardadas na stack. Quando s2 recebe s1, Rust torna s1 uma
//  referência inválida, e passa as informações de s1 para s2, a memória onde os
//  dados estão permanece inalterada e não é feita nenhuma cópia.

// isso evita das referencias apontarem para o mesmo local de memória e causarem o double free

// Copiar o ponteiro sem copiar os dados é chamado de shallow copy (cópia rasa)
// se queremos fazer uma cópia profunda (deep copy) devemos usar o método clone

    let s3 = String::from("texto");
    let s4 = s3.clone(); // s3 foi clonado para s4, os dois apontam para locais na memória


    println!("{s3} {s4}");


}
