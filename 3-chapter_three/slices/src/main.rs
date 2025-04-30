// Outro tipo de dado que não tem ownership é a slice. Slice lhe permitem referenciar
// uma sequência contígua de elementos em uma coleção em vez de referenciar a coleção
//inteira. 
fn main() {
  let s = String::from("TEXTO UM DOIS");
  println!("{}",primeira_palavra(&s));

}

// função  que procura por um espaço na string, se não encrontrar um espaço vazio
//retorna a string inteira
fn primeira_palavra(s: &String) -> usize { //não queremos tomar posse dessa String
  // podemos retornar o índice final de uma String 
  let bytes = s.as_bytes(); //Tranforma a string em um array de bytes: as_bytes()
  for (i, &item) in bytes.iter().enumerate() {
    //iter() cria um iterador sobre o array de bytes
    //iter() é um método que retorna cada elemento de uma coleção
    //enumerate() encapsula o resultado do iter() e retorna cada elemento como parte 
    //como parte de uma tupla. O primeiro elemento da tupla é o índice, o segundo
    // é uma referencia ao valor. isto é pouco mais conveniente do que calcular o
    //índice nós mesmos.
    //O método enumerate retorna uma tupla, então podemos usar padrões para desestruturar
    // esta tupla, assim como qualquer outra coisa em Rust. Então, no for, especificamos um
    // padrão que tem i para o índice na tupla e &item para o byte. Como pegamos uma
    // referência ao elemento através do iter().enumerate(), usamos um & neste padrão
    if item == b' ' {
      return i;
    }
  };

  s.len()
}