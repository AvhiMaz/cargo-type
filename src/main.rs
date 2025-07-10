use std::{
    io::{Write, stdout},
    time::{Duration, Instant},
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{self},
    terminal::{ClearType, disable_raw_mode, enable_raw_mode},
};
use rand::seq::SliceRandom;

fn get_random_sentence() -> &'static str {
    let sentences = [
        "fn map<T, U, F: Fn(T) -> U>(v: Vec<T>, f: F) -> Vec<U> { v.into_iter().map(f).collect() }",
        "let x: Option<&str> = Some(\"hello\").as_ref().filter(|s| s.len() > 3);",
        "match value { Ok(v) => println!(\"{}\", v), Err(e) => eprintln!(\"{}\", e) }",
        "let arc = Arc::new(Mutex::new(HashMap::new()));",
        "unsafe { *ptr.add(1) = 42 } // pointer arithmetic in unsafe block",
        "let closure = |x: i32| -> i32 { x * x };",
        "fn lifetime<'a>(s: &'a str) -> &'a str { s }",
        "impl<T: Display> ToString for T { fn to_string(&self) -> String { format!(\"{}\", self) } }",
        "let s = String::from(\"Rust\").chars().rev().collect::<String>();",
        "let future = async { do_something().await?; Ok(()) };",
        "let result = data.iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect::<Vec<_>>();",
        "enum Result<T, E> { Ok(T), Err(E) }",
        "let boxed: Box<dyn Fn()> = Box::new(|| println!(\"boxed closure\"));",
        "let mut buf = [0u8; 1024]; stream.read(&mut buf)?;",
        "trait Animal { fn speak(&self); } impl Animal for Dog { fn speak(&self) { println!(\"woof\") } }",
        "let slice = &arr[1..=3];",
        "if let Some(val) = maybe_val { println!(\"{}\", val); }",
        "use std::collections::HashSet; let mut set = HashSet::new(); set.insert(\"rust\");",
        "let sum: i32 = (1..=100).sum();",
        "let json: serde_json::Value = serde_json::from_str(input)?;",
    ];

    let mut rng = rand::thread_rng();
    sentences.choose(&mut rng).unwrap()
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
