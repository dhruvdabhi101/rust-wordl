mod names;
use crossterm::cursor;
use crossterm::style::Colorize;
use crossterm::style::Styler;
use names::give_word;

fn main() {
    // initializing stuff
    let mut chances:i8 = 5;
    let word = give_word(); 
    let word = word.to_lowercase();
    let mut won = false;

    println!("{}","Welcome to Rustle, A WORDL written in Rust".green().bold());
    let wlenght = format!("You are given 6 tries and the word lenght is {} characters", word.len());
    println!("{}", wlenght.green().bold());

    println!("{}","Enter your guess:".green().bold());
    println!("");


    while chances > 0 {

        // getting input 
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess = guess.to_lowercase().trim().to_string();


        // moving cursor up to print the word again
        print!("{}", cursor::MoveUp(1));


        // print correct characters as green, wrong characters which are in the word as yellow, and wrong characters which are not in the word as red
        for i in 0..word.len() {
            if guess.as_bytes()[i] == word.as_bytes()[i] {
                print!("{}", guess.chars().nth(i).unwrap().green().bold());
            } else if word.as_bytes().contains(&guess.as_bytes()[i]) {
                print!("{}", guess.chars().nth(i).unwrap().blue().bold());
            } else {
                print!("{}", guess.chars().nth(i).unwrap().red().bold());
            }
        }

        println!();
        if guess.trim() == word {
            won = true;
            break;
        }

        chances -= 1;
    }

    // print the result
    if !won {
        println!("You Lost!!");
        println!("Word was {}", word);
    }else {
        println!("You are the G, You Won!!");
    }


}
