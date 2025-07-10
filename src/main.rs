use std::io::stdout;

use crossterm::{
    cursor, execute,
    style::{self},
    terminal::ClearType,
};
use rand::seq::SliceRandom;

fn get_random_sentence() -> &'static str {
    let sentences = [
        "this is a random 1",
        "this is a random 2",
        "this is a random 3",
        "this is a random 4",
        "this is a random 5",
    ];

    let mut rng = rand::thread_rng();
    sentences.choose(&mut rng).unwrap()
}

fn main() -> std::io::Result<()> {
    let sentences = get_random_sentence();
    let mut stdout = stdout();

    execute!(
        stdout,
        crossterm::terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        style::Print("type the following:"),
        style::Print(format!("{}", sentences)),
        style::Print("Start typing:\n> ")
    )?;

    Ok(())
}
