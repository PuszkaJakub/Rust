fn main() {

/*
    Zadanie 9
*/

let dana = 15;

let mut a = 1;



while a < dana-3{
    
    let mut b = a+1;
    let mut znaleziono = false;

    while b < dana-2{
        if znaleziono{
            break;
        }

        let mut c = b+1;

        while c < dana-1{
            if znaleziono{
                break;
            }

            if a*a + b*b == c*c{
                println!("{} {} {}", a, b, c);
                znaleziono = true;
            }

            c +=1;
        }

        b += 1;
    }

    a += 1;
}
}
