use std::io;
use unicode_segmentation::UnicodeSegmentation;
fn main() {
    const MAX_WRONG_GUESSES: u8 = 6;
    let mut number_of_wrong_guesses = 0;
    let mut secret_word = String::new();
    println!("Player 1, select a secret word!");
    io::stdin().read_line(&mut secret_word).expect("Failed to read line");
    secret_word = secret_word.trim().parse().expect("a");
    println!("Player 2, guess the secret word!");
    println!("{}", &secret_word);
    println!("Player 2, guess the secret word!");
    //loop for 15 iterations. use cls when you learn how to do it
    for _ in 0..100 {
        println!();
    }
    println!("Length of the secret word: {}",secret_word.graphemes(true).count()-1);
    println!("Player 2, guess the secret word!");
    loop {
        println!("Please insert the letter");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        number_of_wrong_guesses+=1;
        if number_of_wrong_guesses >= MAX_WRONG_GUESSES
        {
            break;
        }
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);
    }
}
