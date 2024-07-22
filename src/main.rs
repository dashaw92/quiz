use std::fs::read_to_string;

use format::Quiz;

mod format;

fn main() {
    let file = read_to_string("q_codes.quiz.txt").expect("Missing quiz file.");
    let quiz = Quiz::from_file(&file).expect("Failed to load quiz");
    print!("{quiz}");
}
