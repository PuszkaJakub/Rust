/*
    Zadanie 4
*/

fn armstrong(num: i64)-> bool{
    let mut n = num;
    let mut sum = 0;
    let mut power = 0;
    while n != 0{
        power += 1;
        n /= 10
    }

    n = num;
    while n != 0{
        sum += (n%10).pow(power);
        n /= 10
    }

    return sum == num
}

fn main() {
    let num = 1741725;
    let x = armstrong(num);

    println!("Wynik testu czy liczba {} jest liczba Armstronga: {}", num, x);

}
