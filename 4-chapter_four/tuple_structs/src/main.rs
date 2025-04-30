#[derive(Debug)]
struct Color (i32,i32,i32);
#[derive(Debug)]
struct Point (i32,i32,i32);


struct Unit_like; //são strucs que não tem atributos definidos logo de cara


fn main() {
    // Tuple strucs são uma maneira de criar strucs sem nomear os campos
    // Usamos apenas os tipos 


    let black:Color = Color(0,0,0);
    let origin: Point = Point(0,0,0);
    println!("{:#?} origin: {:#?}", black, origin);
}