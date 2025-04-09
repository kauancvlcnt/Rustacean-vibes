// retornar tranferindo posse

fn main() {
   // Retornar também pode trasferir a posse dee um valor
    let s1 = entrega_valor();
    println!("{}", s1);

    let s2 = String::from("transfere a posse pela função");

    let s3 = pega_e_entrega_valor(s2);
    println!("{}", s3)

}//drop memory RAII

fn entrega_valor() -> String {
    //retorna essa string passando a posse para quem recebe
    let uma_string = String::from("entrega posse s1"); 
    uma_string
}

fn pega_e_entrega_valor(uma_string: String) -> String {
    // a posse de uma_string será transferida para s3
    uma_string
}