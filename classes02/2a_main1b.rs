fn main() {
    /*
        Zadanie 1
    */

    // Edycja zadanie 9

    let dana = 15;
    let mut a = 1;

    println!("Wyniki dla petli loop:");
    loop{
        if a >= dana-3{
            break
        }

        let mut b = a+1;
        let mut znaleziono = false;

        loop{
            if b >= dana-2 || znaleziono{
                break
            }

            let mut c = b+1;

            loop{
                if c >= dana-1 || znaleziono{
                    break
                }

                if a*a + b*b == c*c{
                    println!("{} {} {}", a, b, c);
                    znaleziono = true
                }

                c += 1
            }

            b +=1
        }

        a += 1
    }

    println!("Wyniki dla petli for:");

    for a in 1..dana-2{

        for b in a+1..dana-1{

            for c in b+1..dana{
                if a*a + b*b == c*c{
                    println!("{} {} {}", a, b, c);
                    break
                }
            }
        }
    }


}