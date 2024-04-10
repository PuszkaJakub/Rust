/*
    Zadanie 2b
*/

fn f(x: f64) -> f64{
    return (x*x)-1.0
}

fn fp(x: f64) -> f64{
    return 2.0*x
}

fn met_newt(x0: f64, eps: f64, n: u128) -> f64{
    let mut x = x0;
    let mut n1 = n;

    while f(x).abs() > eps && n1 > 0 && f(x) != 0.0 {
        x = x - f(x)/fp(x);
            n1 -= 1
    }

    return x;
}

fn main() {
    let x = met_newt(-4.0, 0.01, 10);

    println!("{}",x);

}
