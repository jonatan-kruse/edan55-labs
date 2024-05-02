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

pub fn test_monte_carlo(path: &str) {
    let mut max_iterations = 1;
    let input = transform_input("./data/".to_owned() + path + ".in");
    let (correct_fed, correct_ups) = markov_algo(&input);
    for _ in 0..10 {
        let (mut error_fed, mut error_ups) = (1.0, 1.0);
        let mut iterations = 1;
        while (error_fed > 0.001) | (error_ups > 0.001) {
            iterations *= 10;
            let (test_fed, test_ups) = monte_carlo_algo(&input, iterations);
            error_fed = (correct_fed - test_fed) / correct_fed;
            error_ups = (correct_ups - test_ups) / correct_ups;
        }
        println!("Monte carlo needed {iterations} iterations to converge on {path} example");
        max_iterations = max_iterations.max(iterations);
    }
    println!("10 round max for iterations count: {max_iterations}");
}

pub fn run_monte_carlo(path: &str) {
    print!("{path} - Monte Carlo:  ");
    let input = transform_input("./data/".to_owned() + path + ".in");
    let (fed, post) = run_with_spinner(|| monte_carlo_algo(&input, 10000));
    let answer = "\n    fed: ".to_owned() + &(fed).to_string() + "\n    post: " + &(post).to_string();
    println!("{answer}");
}

pub fn run_markov(path: &str) {
    print!("{path} - Markov Chain: ");
    let input = transform_input("./data/".to_owned() + path + ".in");
    let (fed, post) = run_with_spinner(|| markov_algo(&input));
    let answer = "\n    fed: ".to_owned() + &(fed).to_string() + "\n    post: " + &(post).to_string();
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
