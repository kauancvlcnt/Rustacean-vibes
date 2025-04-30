fn main() {
    // É meio irritante saber que qualquer coisa que passemos a uma função também
    //  precisa ser passado de volta se quisermos usá-lo novamente, além de algum
    //  possível resultado proveniente do corpo da função que também queremos retornar.

    // É possível retornar múltiplos valores usando uma tupla, da seguinte forma:
    let s1 = String::from("texto"); //s1 será movida para a função calc tamanho na próxima linha //1
    let (s2, tamanho) = calcula_tamanho(s1); //destructuring do retorno da função calc tamanho //2, 5
    println!("{s2}, tamanho:{tamanho}");

}

fn calcula_tamanho(s:String) -> (String, usize) { //3
    let tamanho = s.len();
    (s, tamanho) //tupla que vai ser retornada //4
}
