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

fn main() {
    let player1 = Choice::ROCK;
    let player2 = Choice::SCISSORS;

    let outcome = play(&player1, &player2);
    
    let outcome_msg = match outcome {
        Outcome::DRAW => "Draw!",
        Outcome::PLAYER1_WIN => "Player 1 wins!",
        Outcome::PLAYER2_WIN => "Player 2 wins!"
    };

    println!("{outcome_msg}");
}
