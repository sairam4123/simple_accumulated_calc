use std::io;
use fancy_regex::Regex;

fn main() {
    //simple_calculator();
    accumulated_calculator();
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
            
            match *operator {
                "+" => result += operand,
                "-" => result -= operand,
                "*" => result *= operand,
                "/" => {
                    if operand == 0f32 {
                        println!("Division by zero is not possible.");
                        return;
                    }
                    result /= operand
                },
                "^" => {
                    if operand == 0f32 && result == 0f32 {
                        println!("0^0 is undefined.");
                        return;
                    }
                    result = result.powf(operand);
                },
                
                op => {
                    println!("Unknown operator {op}.");
                    return;
                }
            }
            
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
    let res = match operator {
        "+" => number + number2,
        "-" => number - number2,
        _ => { println!("Unknown operator"); -1 },
    };
    println!("Result: \n{}", res);
}
