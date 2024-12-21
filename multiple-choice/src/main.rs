use std::io;

struct Questionnaire {
    question: String,
    options: Vec<String>,
    answer: usize,
}

fn create_questionnaire() -> Questionnaire {
    Questionnaire {
        question: String::from("What is the capital of France?"),
        options: vec![
            String::from("Paris"),
            String::from("London"),
            String::from("Berlin"),
            String::from("Madrid"),
        ],
        answer: 0, // Index 0 corresponds to "Paris"
    }
}

fn display_question(questionnaire: &Questionnaire) {
    println!("{}", questionnaire.question);
    for (index, option) in questionnaire.options.iter().enumerate() {
        println!("{}: {}", index + 1, option); // Display options as 1, 2, 3, etc.
    }
}

fn handle_input() -> usize {
    let mut input = String::new();
    println!("Choose an option (1, 2, 3, or 4):");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input
        .trim()
        .parse::<usize>()
        .expect("Invalid input, please enter a number") - 1 // Convert to zero-based index
}

fn main() {
    let questionnaire = create_questionnaire();
    display_question(&questionnaire);

    let user_answer = handle_input();

    if user_answer == questionnaire.answer {
        println!("Correct!");
    } else {
        println!(
            "Wrong! The correct answer was: {}",
            questionnaire.options[questionnaire.answer]
        );
    }
}
