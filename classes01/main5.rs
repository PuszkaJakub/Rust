fn main() {

/*
    Zadanie 5
*/

    let czas1: [i32; 3] = [3,2,3];
    let czas2: [i32; 3] = [2,3,4];

    let czas1wynik = czas1[0] + czas1[1]*60 + czas1[2]*3600;
    let czas2wynik = czas2[0] + czas2[1]*60 + czas2[2]*3600;

    let mut roznica;

    if czas1wynik > czas2wynik{
        roznica = czas1wynik-czas2wynik;

    }
    else{
        roznica = czas2wynik-czas1wynik;
    }

    let mut wynik: [i32; 3] = [0,0,0];

    wynik[2] = roznica%60;
    roznica -= roznica%60;
    roznica /= 60;

    wynik[1] = roznica%60;
    roznica -= roznica%60;
    roznica /= 60;

    wynik[0] = roznica;

    println!("Roznica czasu wynosi {} : {} : {}", wynik[0], wynik[1], wynik[2]);
}
