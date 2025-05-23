use std::num::ParseIntError;

fn main() {
    let teste = return_result();
    println!("{:?}", teste); //Err ParseInt
}

fn return_result() -> Result<i32, ParseIntError> {
    //ParseIntError é um Err que  parse()? retorna
    //Ok(i32)
    let my_str = String::from("OI");
    let my_str: i32 = my_str.trim().parse()?; //trata o err
    Ok(my_str)
}
