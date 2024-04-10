/*
    Zadanie 5
*/

fn rand(seed: &mut u128, min_rand: i128, max_rand: i128)-> i128{

    *seed = (75 * *seed + 74) % 65537;
    *seed as i128 % (max_rand-min_rand+1)+min_rand     
}

fn swap_arr(arr: &mut [i32], i:usize, j: usize){
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}

fn rand_perm(arr: &mut [i32], seed: &mut u128){
    for i in 0..arr.len(){
        let n = arr.len() as i128;
        let j = rand(seed, 0, n-1) as usize;
        swap_arr(arr, i, j);
    }
}


fn main() {

    let mut tablica = [0,1,2,3,4,5,6,7,8,9];
    let mut seed = 201;
    rand_perm(&mut tablica, &mut seed);
    println!("{:?}", tablica)
}
