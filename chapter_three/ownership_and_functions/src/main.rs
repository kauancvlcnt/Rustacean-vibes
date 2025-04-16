// Ownership e funções
// Demostrando como é traferir a posse de valores Move para funções

fn main() {
    let s = String::from("Sou um dado"); //s entra no escopo.
    toma_posse(s); //s agora foi movido para dentro da função
                   //s não é mais válido no escopo dessa função

    // println!("{s}"); não funcionaria, pois o valor já foi movido
    
    let x: u32 = 5; //x entra no escopo
    faz_uma_copia(x); //Se x fosse Move, seria movido para dentro da
                      //função, porém ele é Copy, ele será apenas copiado sem
                      // ser invalidado no escopo dessa função.

    println!("{x} é um Copy. Não foi movido, apenas copiado :)");
}
fn toma_posse(uma_string: String) {
    println!("{uma_string} Move, agora essa função é meu owner");
}

fn faz_uma_copia(um_inteiro: u32) {
    println!("Copy dentro da função {um_inteiro}")
}
