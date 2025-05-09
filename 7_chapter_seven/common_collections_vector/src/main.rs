// A maioria dos tipos de dados representa um valor específico,
//  mas coleções podem conter múltiplos valores. Diferente dos tipos
//  embutidos array e tupla, os dados que essas coleções apontam estão
//  guardados na heap, que significa que a quantidade de dados não precisa
//  ser conhecida em tempo de compilação e pode aumentar ou diminuir conforme
//  a execução do programa.

// Coleções mais utilizadas:
// Vetor -> possibilita guardar um número variável de valores um ao lado do outro
// String -> sequência de caracteres
// hash map -> permite associar um valor a uma chave em particular. e uma implementação
// particular da estrutura de dados mais geral chamada map

fn main() {
    // Vector
    // sintaxe let name Vec<T> = vec![1,2,3]; ou cria um vetor vazio usando Vec::new();
    //  T é o tipo do vetor
    // Vetores são homogêneos, todos os dados do vetor deve ser de  um só tipo
    {
        let v: Vec<i32> = vec![1, 2, 3, 4]; // vetor com valores já colocados
        let mut my_vec: Vec<i32> = Vec::new(); //vetor vazio e mutável para que possamos modificar

        //modificando um vetor
        my_vec.push(1); //podemos adicionar valores ao
        my_vec.push(2);
        my_vec.pop(); //podemos remover um elemento do vetor

        //usa as informações de my_vec
    } //após sair desse escopo, my_vec será descartado drop()

    {
        //Vetores são indexados por números, começando do 0
        let v: Vec<i32> = vec![1, 2, 3, 4];
        let position_1: &i32 = &v[1]; //Leitura por indexação
        let position_1: Option<&i32> = v.get(1); //Leitura pelo método get

        let first: &i32 = &v[2];
        // v.push(6); O vetor não pode ser modificado enquanto tem uma referência válida
        println!("{}", first);
        let does_not_exists = &v.get(100); //quando o index do vetor não existe, get() retorna None
        println!("{:#?}", does_not_exists);
        let does_not_exists = &v[100]; //Entra em panic!, já que o indice não existe
        println!("{:#?}", does_not_exists);
    }
}
