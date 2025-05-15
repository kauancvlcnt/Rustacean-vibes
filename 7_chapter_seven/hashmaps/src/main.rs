// Vector, String, hashmaps são tipos de coleções comuns

//Assim como os vetores, HashMaps são armazenados na
//Como vetores, os hash maps são homogêneos: todas as chaves
//ter o mesmo tipo e todos os valores devem ter o mesmo tipo.

use std::collections::HashMap; //não está no prelúdio

fn main() {
    let mut hashs = HashMap::new();
    // hashs.insert(k, v)onde k é a key, e v é o value
    hashs.insert("blue".to_owned(), 1); //define a chave e o valor para a tabela hash

    let teste = hashs["blue"]; // podemos acessar a chave pelo nome usando [], porém Rust entra em um panic se a chave não existir
    let teste2 = hashs.get("blue"); //acessando usando get(), que retorna um Some/None
    println!("{:?}", teste);
    println!("{:?}", teste2);

    let test23 = match teste2 {
        Some(1) => "sim, é 1".to_owned(),
        _ => "não".to_owned(),
    };

    println!("{}", test23);

    other_hashmap();
    iterator();
    owership_hashmaps();
}

fn other_hashmap() {
    //Outra maneira de fazer um hashMap
    let teams = vec![String::from("Blue"), String::from("Yelow")];
    let initial_scores = vec![10, 20];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores.get(&"Blue".to_string()));
}

fn iterator() {
    let teste = String::from("blu");
    let for_it = teste.as_bytes().iter();
    let for_it1 = teste.as_bytes().iter();
    //Um vetor de <_> (_ permite que Rust faça a inferência do tipo) formado por tuplas de dois elementos
    //esses dois elementos juntos na tupla vem do método zip
    //collect fará com que isso seja um Vetor
    let my: Vec<_> = for_it.zip(for_it1).collect();

    println!("{:?}", my)
}

fn owership_hashmaps() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let field_name1 = String::from("Favorite color 2");
    let field_value1 = String::from("Yelow");
    let mut my_hash = HashMap::new();
    my_hash.insert(field_name1, field_value1);
    my_hash.insert(field_name, field_value); //hashMaps só podem ter chaves unique
    //field_name  e field_value sáo inválidos, o ownership é do Hashmap my_hash
    println!("{:?}", my_hash);

    //podemos iterar sobre HashMaps
    for (key, value) in my_hash {
        println!("{}: {}", key, value)
    }
}
