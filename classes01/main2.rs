fn main() {

/*
    Zadanie 2
*/

    let miesiac = 1;
    let rok = 2021;

    if miesiac == 2{
        if rok%4 == 0 && rok%100 != 0{
            println!("{miesiac} miesiac {rok} roku ma 29 dni");
        }
        else if rok%400 == 0{
            println!("{miesiac} miesiac {rok} roku ma 29 dni");
        }
        else{
            println!("{miesiac} miesiac {rok} roku ma 28 dni");
        }
    }
    else if miesiac%2 == 0{
        println!("{miesiac} miesiac {rok} roku ma 30 dni");
    }
    else{
        println!("{miesiac} miesiac {rok} roku ma 31 dni");
    }


}
