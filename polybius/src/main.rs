use std::collections::HashMap;
const LETTER_TABLE: [[char; 5]; 5] = [
    ['A', 'B', 'C', 'D', 'E'],
    ['F', 'G', 'H', 'I', 'K'],
    ['L', 'M', 'N', 'O', 'P'],
    ['Q', 'R', 'S', 'T', 'U'],
    ['V', 'W', 'X', 'Y', 'Z'],
];
trait Convert {
    fn to_int(&self) -> i32;
}
impl Convert for String {
    fn to_int(&self) -> i32 {
    return self.parse::<i32>().unwrap();
    }
}
impl Convert for usize {
    fn to_int(&self) -> i32 {
        return self.to_string().parse::<i32>().unwrap();
    }
}
   


fn initilaze_dictionary() -> HashMap<char, i32> {
    let mut letter_hash_map: HashMap<char, i32> = HashMap::new();
    for i in 0..LETTER_TABLE.len() {
        for j in 0..LETTER_TABLE[i].len() {
            letter_hash_map.insert(
                LETTER_TABLE[i][j],
                (Convert::to_int(&i) + 1) * 10 + Convert::to_int(&j) + 1,
            );
        }
    }
    return letter_hash_map;
}
fn encrypt(raw_message: &str) -> String {
    let letter_hash_map = initilaze_dictionary();
    let mut encrypted_message: String = String::new();
    for letter in raw_message.chars() {
        if (letter <= 'Z' && letter >= 'A') || letter == ' ' {
            if letter == ' ' {
                encrypted_message.push_str(" ");
            } else if letter == 'J' || letter == 'i' {
                encrypted_message.push_str(&letter_hash_map.get(&'I').unwrap().to_string());
            } else if letter != 'J' {
                encrypted_message.push_str(&letter_hash_map.get(&letter).unwrap().to_string());
            }
        }
    }
    return encrypted_message;
}
fn decrypt(encrypted_message: &str) -> String {
    let mut decrypted_message: String = String::new();
    let mut paired_messages: Vec<Vec<String>> = Vec::new();
    for i in encrypted_message.split(" ") {
        paired_messages.push(pair_numbers(i));
    }
    for i in paired_messages {
        for j in i {
            decrypted_message.push_str(
                &LETTER_TABLE[((Convert::to_int(&j) / 10) - 1) as usize]
                    [(Convert::to_int(&j) % 10 - 1) as usize]
                    .to_string(),
            );
        }
    }
    return decrypted_message;
}
fn pair_numbers(encrypted_message: &str) -> Vec<String> {
    let mut paired_message = Vec::new();
    for i in 0..encrypted_message.len() {
        if i % 2 == 0 {
            paired_message.push(format!(
                "{}{}",
                &encrypted_message.chars().nth(i).unwrap().to_string(),
                &encrypted_message.chars().nth(i + 1).unwrap().to_string()
            ));
        } else {
            continue;
        }
    }
    return paired_message;
}

fn main() {
    //upper case and trim when sending to functions
    let mut mode;
    println!("Welcome to Polybius Cipher");
    println!("Please select a mode");
    println!("1. Encrypt");
    println!("2. Decrypt");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        mode = input.trim().parse::<i32>().unwrap();
        if mode == 1 || mode == 2 {
            break;
        } else {
            println!("Please enter a valid mode");
        }
    }
    let mut message = String::new();
    println!("Enter your message: ");
    std::io::stdin().read_line(&mut message).unwrap();
    message = message.trim().to_string().to_uppercase();
    if mode == 1 {
        println!("Encrypted message: {}", encrypt(&message));
    } else if mode == 2 {
        println!("Decrypted message: {}", decrypt(&message));
    }

}
