use std::io;

fn main() {
    hangman("rustisfun".to_string());
}

fn hangman(original_word: String) {
    let mut word = "-".repeat(original_word.len()).to_string();
    let mut guessed_letters = "".to_string();
    let mut number_of_guesses: u32 = 5;

    loop {
        println!("The word so far is {}", word);
        println!("You have guessed the following letters: {}", guessed_letters);
        println!("You have {} guesses left", number_of_guesses);

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
            let guess: char = match guess.trim().parse() {
                Ok(char) => char,
                Err(_) => continue
            };

        if original_word.contains(guess) {
            for (i, c) in original_word.chars().enumerate() {
                if c == guess {
                    word.replace_range(i..i+1, &guess.to_string());
                }
            } 
            
        }
        else {
            number_of_guesses -= 1;
        }

        guessed_letters += &guess.to_string();

        if !word.contains('-') {
            println!("Yeahhh! You are the champion, my friend!");
            println!("You guessed the word {}!", original_word);
            break;
        }

        if number_of_guesses == 0 {
            println!("Noooooo, you lost :(");
            break;
        }
    }
}
