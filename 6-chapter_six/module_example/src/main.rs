use module_example::calculadora::div;
use module_example::calculadora::mult;
fn main() {
    let my_division = div::division(10, 5);
    let my_pow = mult::pow(2, 5);

    //de qualquer maneira rust sempre retornará uma expressão, se não retornar
    //println, ele retornará uma tupla vazia "()"
    println!("div: {} pow: {}", my_division, my_pow)
}
