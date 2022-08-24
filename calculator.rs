use std::io;

pub fn run(){
    println!("What number would you like to start with?");

    let mut quit = false;
    let mut number_string = String::new();

    //checks for valid input
    match io::stdin().read_line(&mut number_string){
        Ok(_) => { println!("You entered {}", number_string); },
        Err(_e) => { println!("operator error"); }
    }
    // parses a float from the input
    let mut number: f64 = number_string.trim().parse::<f64>().unwrap();
    
    while !quit{
        println!("What would you like to do with {}?", number);
        println!("(+, -, *, /, ^, q to quit)");
        let mut operator = String::new();
        match io::stdin().read_line(&mut operator){
            Ok(_) => {},
            Err(_e) => {}
        }
        if operator == "q"{
            quit = true;
            break;
        }
        get_operation(&operator.trim(), number);

        let mut new_num = String::new();

        match io::stdin().read_line(&mut new_num){
            Ok(_) => {},
            Err(_e) => {}
        }
        if operator.trim() == "/" && new_num.trim() == "0"{
            println!("Divide by zero error, please try again");
            continue;
        }
        number = do_operation(number, new_num.trim().parse::<f64>().unwrap(), &operator.trim());
        println!("New number: {}", number);
    }
    println!("Closing the calculator.");   
}

// detects operation type, prints to console
fn get_operation(c: &str, num: f64){
    match c{
        "+" => println!("What would you like to add to {}?", num),
        "-" => println!("What would you like to subtract from {}?", num),
        "*" => println!("What would you like to multiply {} by?", num),
        "/" => println!("What would you like to divide {} by?", num),
        "^" => println!("What power would you like to set {} to?", num),
        x => panic!("Unexpected operator {:?}", x)
    }
}
// returns the new number after the operation
fn do_operation(num1: f64, num2: f64, op: &str) -> f64{

    match op{
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        "^" => f64::powf(num1, num2),
        _x => panic!("Unexpected operator type")
    }
}
