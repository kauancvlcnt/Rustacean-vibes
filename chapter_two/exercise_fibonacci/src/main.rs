fn main() {

    let mut m:[u128; 3] = [0, 1, 1];
    for _i in 1..185 {
       m[0] = m[1] + m[2];
       m[1] = m[2];
       m[2] = m[0];
        println!("{}",m[2]);
        
    }

}
