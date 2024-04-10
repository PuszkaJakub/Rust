/*
    Zadanie 6
*/

fn prime_factors(num: i64){
    let mut n = num;

    while n != 1{
        for i in 2..n{
            if n%i == 0{
                println!("{}",i);
                n /= i;
                break
            }
            else if i+1 == n{
                println!("{}", n);
                n /= i;
            }
        }
    }
}

fn main() {
    let num = 72;
    println!("Czynniki pierwsze liczby {}:", num);
    prime_factors(num);
}
