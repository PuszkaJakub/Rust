/*
    Zadanie 4
*/

fn swap_arr(arr: &mut [i32], i:usize, j: usize){
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}


fn main() {

    let mut tablica = [0,1,2,3,4,5,6,7,8,9];

    println!("{:?}", tablica);
    swap_arr(&mut tablica, 2, 9);
    println!("{:?}", tablica)
}
