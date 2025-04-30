fn main() {
    let key:bool =  true;
    let mut count: i32 = 0;

    // Usar o if let implica menos código pra digitar e menos indentação.
    //  Porém, perdemos a verificação exaustiva que é garantida pelo match.

    if let true = key {
        loop {
            println!("{}", count);
            // usando if let para parar o loop
            if let 5 = count {
                break;
            }
            count= count + 1;
        }
    }

}
