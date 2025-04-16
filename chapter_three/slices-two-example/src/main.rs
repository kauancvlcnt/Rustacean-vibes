fn main() {

    let s = String::from("Texto de teste");

    // um slice  pega o range de uma palavra 

    let texto = &s[0..s.len()]; //pega do zero até o final
    let longo = &s[5..]; //pega do 5 até o final da string
    let _mais_um_texto = &s[..]; //pega do começo até o final do range
    let slice_num = [1,2,3,4];
    let slice = &slice_num[2..];
    println!{"{}, {}, {:?}", texto, longo, slice};
}
