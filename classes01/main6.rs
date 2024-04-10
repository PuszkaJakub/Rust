fn main() {

/*
    Zadanie 6
*/

    let mut result = 1;
    let mut step = 1;

    while step <= 4{
        result *= step;
        step +=1;
    }

    println!("Silnia wynosi {}", result);
}
