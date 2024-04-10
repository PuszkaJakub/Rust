/*
    Zadanie 2
*/

fn main() {
    let mut num: u8 = 33;

    while num <= 126 {
        let b: char = num as char;
        println!("{} = {}", num, b);
        num += 1
    }

}
