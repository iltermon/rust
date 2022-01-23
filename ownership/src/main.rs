struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}
fn main() {
    // let string = "Hello, world!";
    // ownership_inplace_test_function(string);
    // println!("{}", string);
    let x = ToDrop;
    println!("Made a ToDrop!");
}
#[allow(dead_code)]
fn ownership_function_int(mut number: i32) {
    number = number + 2;
    number = number.pow(2);
    println!("{}", number);
}
#[allow(dead_code)]
fn ownership_function_string(text: &str) -> String {
    let new_text = text.to_string().replace("world", "Rust");
    return new_text.to_string();
}

fn ownership_inplace_test_function(test_text: &str) {
    test_text.to_string().push_str(" Rust");
    print!("{}", test_text);
}