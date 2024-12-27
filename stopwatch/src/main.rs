use stopwatch::Stopwatch;
use std::io;

fn main() {
    let mut stopwatch = None; 
    loop {
        println!("Enter 'start' to start the stopwatch, 'stop' to stop it, or 'exit' to quit:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let command = input.trim().to_lowercase();

        match command.as_str() {
            "start" => {
                stopwatch = Some(start_watch());
            }
            "stop" => {
                if let Some(mut sw) = stopwatch.take() {
                    stop_watch(&mut sw);
                } else {
                    println!("Stopwatch has not been started yet.");
                }
            }
            "exit" => {
                println!("Exiting the program.");
                break;
            }
            _ => {
                println!("Invalid input. Please enter 'start', 'stop', or 'exit'.");
            }
        }
    }
}

fn start_watch() -> Stopwatch {
    println!("Stopwatch started!");
    Stopwatch::start_new()
}

fn stop_watch(sw: &mut Stopwatch) {
    sw.stop();
    println!("Stopwatch stopped. Elapsed time: {} ms", sw.elapsed_ms());
}
