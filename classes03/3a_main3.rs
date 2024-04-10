/*
    Zadanie 3
*/

fn rand(seed: &mut u128, min_rand: i128, max_rand: i128)-> i128{

    *seed = (75 * *seed + 74) % 65537;
    *seed as i128 % (max_rand-min_rand+1)+min_rand     
}

fn main() {
    let mut ziarno = 10;

    println!("{}", rand(&mut ziarno, 1, 100));
    println!("{}", rand(&mut ziarno, 1, 100));
    println!("{}", rand(&mut ziarno, 1, 100));
    println!("{}", rand(&mut ziarno, 1, 100));
    println!("{}", rand(&mut ziarno, 1, 100));
    println!("{}", rand(&mut ziarno, 1, 100));
    println!("{}", rand(&mut ziarno, 1, 100));
    println!("{}", rand(&mut ziarno, 1, 100));
}
