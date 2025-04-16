#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// self é quem chama, é semelhante ao this
impl Rectangle {
    fn area(&self) -> u32 {
        self.width + self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

//função associada ::
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width:size,
            height: size,
        }
    }
}
fn main() {
    // uma struct é um conjunto de vários valores relacionados que compõem um grupo
    //  dadoss

    let rect = Rectangle {
        width: 90,
        height: 80,
    };

    let rect1 = Rectangle {
        width: 50,
        height: 80,
    };

    let rect2 = Rectangle {
        width: 100,
        height: 70,
    };

    let rect3 = Rectangle::square(30);

    println!("rect can hold rect1: {:#?}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {:#?}", rect.can_hold(&rect2));
    println!("rect width {} and height {}", rect.width, rect.height);
    //quando lá em cima é colocado self, a struct rect pega elamesma.width e
    //  elamesma.heigth
    println!("rect area {}", rect.area());
    println!("rect {:#?}", rect3); //podemos acessar o .width e o .height
}

// para funções associadas parecem não  usar o &self
