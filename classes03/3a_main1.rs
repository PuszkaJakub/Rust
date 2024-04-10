/*
    Zadanie 1
*/

fn swapuj(a: &mut i32, b: &mut i32){
    let temp = *a;
    *a = *b;
    *b = temp
}

fn main() {
    let mut x1: i32 = 4;
    let mut x2: i32 = 2;

    println!("Pierwsza liczba: {}, druga liczba: {}", x1, x2);

    swapuj(&mut x1, &mut x2);

    println!("Pierwsza liczba: {}, druga liczba: {}", x1, x2)
}
