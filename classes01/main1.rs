fn main() {

/*
    Zadanie 1
*/

    let rok = 2004;

    if rok%4 == 0 && rok%100 != 0{
        println!("Rok {rok} jest przestepny");
    }
    else if rok%400 == 0{
        println!("Rok {rok} jest przestepny");
    }
    else{
        println!("Rok {rok} nie jest przestepny");
    }


}
