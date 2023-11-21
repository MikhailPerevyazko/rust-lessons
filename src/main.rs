fn main() {
    let a = 3;

    if a < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    testif();
}

fn testif() {
    let b = 5;

    if b % 4 == 0 {
        println!("Число делится на 4.");
    } else if b % 3 == 0 {
        println!("Число делится на 3");
    } else if b % 2 == 0 {
        println!("Число делится на 2.");
    } else {
        println!("Число не делится ни на 4, ни 3, ни на 2.");
    }
}
