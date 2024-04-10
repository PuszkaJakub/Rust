/*
    Zadanie 2
*/

fn segreguj(a: &mut i32, b: &mut i32, c: &mut i32){
    let mut temp;
    if a > b{
        temp = *a;
        *a = *b;
        *b = temp;
    }
    if a > c{
        temp = *a;
        *a = *c;
        *c = temp;
    }
    if b > c{
        temp = *b;
        *b = *c;
        *c = temp;
    }
        
}

fn main() {
    let mut a: i32 = 16;
    let mut b: i32 = 12;
    let mut c: i32 = 3;

    println!("Pierwsza liczba: {}, druga liczba: {}, trzecia liczba: {}", a, b, c);

    segreguj(&mut a, &mut b, &mut c);

    println!("Pierwsza liczba: {}, druga liczba: {}, trzecia liczba: {}", a, b, c);

}
