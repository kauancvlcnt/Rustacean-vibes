fn main() {
    let length1 = 50;
    let width1 = 30;


    println!("The area of the rectangle is {} square pixels", area(length1, width1))
}

fn area(length: i32, width: i32) -> i32 {
    length * width
}