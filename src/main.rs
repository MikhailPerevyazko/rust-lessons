fn main() {
    func1();
    func2();
    func3();
    
}

fn func1() {
    let a = 3;
    if a < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
}

fn func2() {
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

fn func3() {
    
}
