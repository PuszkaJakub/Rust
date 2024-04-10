/*
    Zadanie 3
*/

fn collatz(num: i64)-> i64{
    let mut counter: i64 = 0;
    let mut n = num;
    while n != 1{
        if n%2 ==0{
            n = n/2
        }
        else{
            n = 3*n + 1
        }
        counter += 1
    }

    return counter;
}

fn main() {
    let num = 27;
    let x = collatz(num);

    println!("Liczba {} osiaga jedynke w problemie Collatza przy {} iteracji", num, x);

}
