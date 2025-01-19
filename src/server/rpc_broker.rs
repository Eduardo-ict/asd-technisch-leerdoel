pub fn invoke_function(function_name: &str, args: &Vec<i32>) -> String {
    match function_name {
        "add" => add(args[0], args[1]).to_string(),
        "subtract" => subtract(args[0], args[1]).to_string(),
        "multiply" => multiply(args[0], args[1]).to_string(),
        "divide" => divide(args[0], args[1]).to_string(),
        "hoi" => hoi(),
        _ => "Function not found".to_string(),
    }
}

// Functies voor de standaard bewerkingen

fn add(v1: i32, v2: i32) -> i32 {
    v1 + v2
}

fn subtract(v1: i32, v2: i32) -> i32 {
    v1 - v2
}

fn multiply(v1: i32, v2: i32) -> i32 {
    v1 * v2
}

fn divide(v1: i32, v2: i32) -> i32 {
    v1 / v2
}

fn hoi() -> String {
    "Hoi, alles goed?".to_string()
}