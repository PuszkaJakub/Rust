fn main() {

/*
    Zadanie 8
*/

    let mut number = 45123;
    let mut result = 0;

    while number != 0{
        result += number%10;
        number /= 10;
    }

    println!("Suma cyfr wynosi {}", result);

}
