mod markov;
mod monte_carlo;
mod transform_input;
use crossterm::{cursor, ExecutableCommand};
use markov::markov_algo;
use monte_carlo::monte_carlo_algo;
use std::io::{stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time::Duration};
use transform_input::transform_input;

pub fn run_monte_carlo(path: &str) {
    print!("{path} - Monte Carlo:  ");
    let input = transform_input("./data/".to_owned() + path + ".in");
    let answer = run_with_spinner(|| monte_carlo_algo(input));
    println!("{answer}");
}

pub fn run_markov(path: &str) {
    print!("{path} - Markov Chain: ");
    let input = transform_input("./data/".to_owned() + path + ".in");
    let answer = run_with_spinner(|| markov_algo(input));
    println!("{answer}");
}

pub fn run_with_spinner<T>(func: impl FnOnce() -> T) -> T {
    let spinner_chars = [
        "🌕 ", "🌕 ", "🌔 ", "🌓 ", "🌒 ", "🌑 ", "🌑 ", "🌘 ", "🌗 ", "🌖 ",
    ];
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();
    // keep track of what was the cursor position
    stdout().execute(cursor::SavePosition).unwrap();

    let handle = std::thread::spawn(move || {
        let mut index = 0;
        while running_clone.load(Ordering::SeqCst) {
            print!("{}", spinner_chars[index]);
            let _ = stdout().flush();
            index = (index + 1) % spinner_chars.len();
            thread::sleep(Duration::from_millis(100));
            // move cursor to the beginning of the line and up one line
            stdout().execute(cursor::RestorePosition).unwrap();
            print!("  ");
            stdout().execute(cursor::RestorePosition).unwrap();
        }
    });

    let result = func();

    running.store(false, Ordering::SeqCst);
    handle.join().unwrap();

    result
}
