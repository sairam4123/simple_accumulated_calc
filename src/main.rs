use std::io;
use fancy_regex::Regex;

fn main() {
    println!("Choose a calculator to use:\n1. Accumulated Calculator\n2. Simple Calculator\n>");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read from stdin.");
    let is_accumulated = buffer.trim().to_lowercase().starts_with("acc");
    if is_accumulated {
        accumulated_calculator();
    } else {
        simple_calculator();
    }
}

fn calculation_kernel(op1: f32, op2: f32, op: &str) -> f32 {
    match op {
        "+" => op1 + op2,
        "-" => op1 - op2,
        "*" => op1 * op2,
        "/" => {
            if op2 == 0f32 {
                println!("Division by zero is not possible.");
                return -1f32;
            }
            op1 / op2
        },
        "^" => {
            if op1 == 0f32 && op2 == 0f32 {
                println!("0^0 is undefined.");
                return -1f32;
            }
            op1.powf(op2)
        },

        op => {
            println!("Unknown operator {op}.");
            -1f32
        }
    }
}

fn accumulated_calculator() {
    println!("Accumulated calculator");
    println!("Enter a expression:");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read from stdin");
    let input = buffer.trim();
    if input.is_empty() {
        println!("Input cannot be empty.");
    }

    let re = Regex::new(r"(?<=[+\-*^/])|(?=[+\-*^/])")
        .expect("Something went wrong");

    let tokens: Vec<&str> = re.split(input)
        .map(|x| x.unwrap())
        .collect();
    if tokens.is_empty() || tokens.len() % 2 == 0 {
        println!("Invalid expression. Provide a valid expression.");
        return;
    }

    let mut result: f32 = match tokens[0].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Must be a number");
            return;
        }
    };
    
    for chunks in tokens[1..].chunks(2) {
        if let [operator, operand_str] = chunks {
            let operand: f32 = match operand_str.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Must be a number.");
                    return;
                }
            };
            
            result = calculation_kernel(result, operand, *operator);
            
        } else {
            println!("Malformed expression. Try again.")
        }
    }
    println!("The result: {}", result);
}

fn simple_calculator() {
    // need two inputs
    println!("Enter number A: ");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Must not be empty");
    let number: i32 = buffer.trim_end().parse::<i32>().expect("Must be an integer");
    println!("Enter number B: ");
    let mut buffer2 = String::new();
    io::stdin().read_line(&mut buffer2).expect("Must not be empty");
    let number2: i32 = buffer2.trim_end().parse::<i32>().expect("Must be an integer");
    println!("Enter operator: ");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("Must not be empty");
    let operator = operator.trim_end();
    assert_eq!(operator.len(), 1);
    let result = calculation_kernel(number as f32, number2 as f32, operator);
    println!("Result: \n{}", result);
}
