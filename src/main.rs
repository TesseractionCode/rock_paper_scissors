use std::io;

#[derive(PartialEq)]
enum Choice {
    ROCK,
    PAPER,
    SCISSORS,
}

enum Outcome {
    PLAYER1_WIN,
    PLAYER2_WIN,
    DRAW,
}


fn play(player1: &Choice, player2: &Choice) -> Outcome{
    if player1.eq(&player2) {
        Outcome::DRAW
    } else {
        match player1 {
            Choice::ROCK => {
                if player2.eq(&Choice::PAPER) {Outcome::PLAYER2_WIN}
                else {Outcome::PLAYER1_WIN}
            }
            Choice::PAPER => {
                if player2.eq(&Choice::SCISSORS) {Outcome::PLAYER2_WIN}
                else {Outcome::PLAYER1_WIN}
            }
            Choice::SCISSORS => {
                if player2.eq(&Choice::ROCK) {Outcome::PLAYER2_WIN}
                else {Outcome::PLAYER1_WIN}
            }
        }
    }

}

fn getNumInput(cin: &io::Stdin) -> u8 {
    let mut str_buff = String::new();
    cin.read_line(&mut str_buff).ok();

    str_buff.trim().parse::<u8>().unwrap()
}


fn main() {
    
    let cin = io::stdin();

    println!("How many rounds of rock paper scissors do you wish to play?");
    let num_rounds = getNumInput(&cin);
    println!();

    let mut p1_wins = 0u8;
    let mut p2_wins = 0u8;

    for _ in 0..num_rounds {
        println!("It is player 1's turn. 0=Rock, 1=Paper, 2=Scissors. Enter a number:");
        let player1 = getNumInput(&cin);
        println!("It is player 2's turn. 0=Rock, 1=Paper, 2=Scissors. Enter a number:");
        let player2 = getNumInput(&cin);

        let player1_choice = match player1 {
            0 => Choice::ROCK,
            1 => Choice::PAPER,
            2 => Choice::SCISSORS,
            _ => Choice::ROCK
        };
        let player2_choice = match player2 {
            0 => Choice::ROCK,
            1 => Choice::PAPER,
            2 => Choice::SCISSORS,
            _ => Choice::ROCK
        };

        let outcome = play(&player1_choice, &player2_choice);
        match outcome {
            Outcome::DRAW => {},
            Outcome::PLAYER1_WIN => {p1_wins += 1;},
            Outcome::PLAYER2_WIN => {p2_wins += 1;}
        }

    }

    if p1_wins == p2_wins {
        println!("Draw!");
    } else if p1_wins > p2_wins {
        println!("Player 1 wins!");
    } else {
        println!("Player 2 wins!");
    }
}
