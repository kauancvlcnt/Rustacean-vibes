//Rust não deixa retornar  ponteiros soltos

fn main() {
    // let _referencia_para_o_nada = soltar();
}



// fn soltar() -> &String {
//     let s = String::from("Texto"); 
//     &s  aqui retornariamos uma referência a uma String, s 
// }  Aqui é feito o drop de s e sua memória é devolvida 
//  Perigo!
//  Se Rust deixasse, essa referencia que tentamos retornar estaria apontando para um
//  local de memória inválido! Isso não é bom. Rust não nos deixa fazer isso

/*Vamos recapitular o que discutimos sobre referências:

    Em um dado momento, você pode ter um ou outro, mas não os dois:

    Uma referência mutável.
    Qualquer número de referências imutáveis.

    Referências devem ser válidas sempre.

*/