fn main() {
    /*
        Zadanie 1
    */

    // Edycja zadanie 6

    let mut result = 1;
    let mut step = 1;

    loop{
        if step > 4 {
            break;
        }
        result *= step;
        step += 1;
    }

    println!("Silnia obliczona petla loop wynosi {}", result);

    result = 1;
    for i in 1..=4{
        result *= i;
    }

    println!("Silnia obliczona petla for wynosi {}", result);
}
