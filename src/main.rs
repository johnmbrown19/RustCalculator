use std::io::{stdin, stdout, Write};

fn read(input: &mut String) {
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("failed to read");
}

fn main() {
    println!("Welcome to my Rust Calculator!");
    println!("- - - - - - - - - - -");

    let mut n1 = String::new();
    let mut n2 = String::new();
    let mut operator = String::new();

    print!("Enter the first number: ");
    read(&mut n1);

    print!("Enter the second number: ");
    read(&mut n2);

    print!("Enter operator [+-*/]: ");
    read(&mut operator);

    println!("{} {} {}", n1, n2, operator);

    let n1: f32 = n1.trim().parse().unwrap();
    let n2: f32 = n2.trim().parse().unwrap();
    let operator: char = operator.trim().chars().next().unwrap();

    let operators = String::from("+-*/");

    if !operators.contains(operator) {
        println!("Unknown Operator!");
        return;
    }

    let result = match operator {
        '+' => n1 + n2,
        '-' => n1 - n2,
        '*' => n1 * n2,
        '/' => n1 / n2,
        _ => panic!("Error in operator")
    };

    println!("The result of {} {} {} = {}", n1, operator, n2, result);
}
