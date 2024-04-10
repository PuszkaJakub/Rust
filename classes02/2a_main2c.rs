/*
    Zadanie 2c
*/

fn f(x: f64) -> f64{
    return (x*x)-1.0
}

fn fp(x: f64) -> f64{
    return 2.0*x
}

fn met_newt(x0: f64, eps: f64, n: u128) -> f64{
    if f(x0).abs() <= eps || n == 0 || f(x0) == 0.0{
        return x0
    }
    else{
        return met_newt(x0 - f(x0)/fp(x0), eps, n-1) 
    }
}

fn main() {
    let x = met_newt(-4.0, 0.01, 10);

    println!("{}",x);

}
