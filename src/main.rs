use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{self},
    terminal::{ClearType, disable_raw_mode, enable_raw_mode},
};
use rand::seq::SliceRandom;
use std::{
    io::{Write, stdout},
    time::{Duration, Instant},
};
mod sentences;
use sentences::SENTENCES;

fn get_random_sentence() -> &'static str {
    let mut rng = rand::thread_rng();
    SENTENCES.choose(&mut rng).unwrap()
}

fn main() -> std::io::Result<()> {
    let sentence = get_random_sentence();
    let mut stdout = stdout();

    execute!(
        stdout,
        crossterm::terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0),
        style::Print("type the following:\n\n"),
        style::Print(format!("{}\n\n", sentence)),
        style::Print("Start typing:\n> ")
    )?;

    enable_raw_mode()?; // raw mode, key presses are sent directly to the program without waiting for Enter.

    let mut typed = String::new();
    let mut started = false;
    let mut started_time = Instant::now();

    while typed != sentence {
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                if !started {
                    started = true;
                    started_time = Instant::now();
                }

                match key_event.code {
                    KeyCode::Char(c) => {
                        typed.push(c);
                        print!("{}", c);
                        stdout.flush()?;
                    }
                    KeyCode::Backspace => {
                        if !typed.is_empty() {
                            typed.pop();
                            execute!(
                                stdout,
                                cursor::MoveLeft(1),
                                style::Print(" "),
                                cursor::MoveLeft(1)
                            )?;
                        }
                    }
                    KeyCode::Esc => {
                        disable_raw_mode()?;
                        println!("\n\nExited.");
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
    }

    let duration = started_time.elapsed().as_secs_f64();
    let word_count = sentence.split_whitespace().count() as f64;
    let wpm = (word_count / duration) * 60.0;

    disable_raw_mode()?;

    println!("\n\ntime taken: {:.2} seconds", duration);
    println!("wpm: {:.2}", wpm);

    Ok(())
}
