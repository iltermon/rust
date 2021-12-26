fn main() {
    let string = "Hello, world!";
    ownership_inplace_test_function(string);
    println!("{}", string);
}

fn ownership_function_int(mut number: i32) {
    number = number + 2;
    number = number.pow(2);
    println!("{}", number);
}

fn ownership_function_string(mut text: &str) -> String {
    let new_text =  text.to_string().replace("world", "Rust");
    return new_text.to_string();
}

fn ownership_inplace_test_function(test_text: &str) {
 test_text.to_string().push_str(" Rust");
 print!("{}", test_text);
}