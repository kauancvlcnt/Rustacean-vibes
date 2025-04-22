
struct Rectangle {
    width: i32,
    heigth: i32,
}

fn main() {
    let rect1 = Rectangle {
        width: 50,
        heigth: 30,
    };

    println!("The area of the rectangle is {} square pixels", area(&rect1)); //passada uma referência de rectangle



}

fn area(rectangle: &Rectangle) -> i32 { //na assinatura também deve ser explícito que é uma referência do tipo Rectangle
    rectangle.width * rectangle.heigth
}