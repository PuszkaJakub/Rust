/*
    Zadanie 5
*/

fn perfect(num: i64)-> bool{

    let mut sum = 0;
    for i in 1..num{
        if num%i == 0{
            sum += i
        }
    }

    return sum == num;
}

fn main() {
    let num = 496;
    let x = perfect(num);

    println!("Test wyniku czy liczba {} jest liczba doskonala: {}", num, x);

}
