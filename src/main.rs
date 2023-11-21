fn main() {
    func1();
    func2();
    func3();
    func4();
    func5();
    func6();
    
}

fn func1() { // if
    let a: i32 = 3;
    if a < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
}

fn func2() { // else if
    let b: i32 = 5;
    
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

fn func3() { // let + if
    let condition: bool = true;
    let number: i32 = if condition { 5 } else { 6 };
    println!("Значение числа: {number}");  
}

fn func4() { // if + let + loop
    let mut counter: i32 = 0;
    let result: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Результат равен: {result}");
}

fn func5() { // цикл while
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
}

fn func6() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index: usize = 0;

    while index < 5 {
        println!("Значние: {}", a[index]);
        index += 1;
    }
}


