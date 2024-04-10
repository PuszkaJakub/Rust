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

    for _ in 1..n {
        if f(x).abs() <= eps || f(x) == 0.0{
            break
        }
        else{
            x = x - f(x)/fp(x);
        }
    }

    return x;
}

fn main() {
    let x = met_newt(-4.0, 0.01, 10);

    println!("{}",x);

}
