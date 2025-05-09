#[derive(Debug)]
//definindo uma enum para guardar diferentes tipos em um vetor
enum SpreadsheetCell {
    Int(i32),
    Text(String),
    Float(f64),
}

use SpreadsheetCell::*;

fn main() {
    {
        //Vetores devem ter todos os dados de um tipo
        let my_vec: Vec<i32> = vec![1, 2, 3, 4, 5];
        println!("{}", my_vec[2]);
    }

    {
        //Como os vetores só podem ter dados de um tipo, podemos usar uma enum para
        //expandir as possibilidades
        let spread_vector = vec![Int(2), Text("Meu nome".to_string()), Float(43.2)];
        println!("my Spreadsheet: {:?}", spread_vector);
    }
}
