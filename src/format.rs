use std::fmt;
use ansi_term::Colour;
use nanorand::{Rng, WyRand};

#[derive(Debug)]
pub struct Quiz<'a> {
    name: &'a str,
    questions: Vec<Question<'a>>,
    answers: Vec<Answer<'a>>,
    current_question: usize,
}

#[allow(dead_code)]
impl<'a> Quiz<'a> {
    pub fn from_file(lines: &'a String) -> Option<Self> {
        let (name, body) = lines.split_once("\n\n")?;
        let (answer_bank, question_pool) = body.split_once("\n\n")?;

        let answers: Vec<Answer> = answer_bank.split("\n")
            .map(|resp| Answer::new(resp))
            .collect();

        let questions = question_pool.split("\n")
            .enumerate()
            .map(|(idx, prompt)| {
                let (prompt, correct) = prompt.split_once(": ").expect("Invalid question entry.");
                let correct = correct.parse::<usize>().expect("Failed to parse correct answer ID.");

                Question::new(idx, prompt, correct, answers.len())
            })
            .collect();

        Some(Self {
            name,
            questions,
            answers,
            current_question: 0,
        })
    }

    pub fn next_question(&mut self) {
        self.current_question += 1;
        self.current_question %= self.questions.len();
    }

    pub fn goto_question(&mut self, idx: usize) -> Option<()> {
        if idx >= self.questions.len() {
            None
        } else {
            self.current_question = idx;
            Some(())
        }
    }

    pub fn is_answer_correct(&self, response: usize) -> bool {
        if response >= self.answers.len() {
            return false
        }

        let q = &self.questions[self.current_question];
        q.correct == q.choices[response]
    }
}

impl<'a> fmt::Display for Quiz<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{esc}[2J{esc}[0;0H", esc = 27 as char)?;
        writeln!(f, "{}", Colour::Yellow.paint(self.name))?;
        writeln!(f)?;

        let q = &self.questions[self.current_question];
        writeln!(f, "Q{}: {}", q.id + 1, q.prompt)?;
        writeln!(f, "{}", Colour::White.dimmed().paint("---------------------------------"))?;
        for (answer_ordinal, answer_idx) in q.choices.iter().enumerate() {
            let answer = &self.answers[*answer_idx];
            writeln!(f, "{}) {}\n", Colour::Cyan.paint((answer_ordinal + 1).to_string()), answer.resp.replace(" / ", "\n   "))?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Question<'a> {
    prompt: &'a str,
    id: usize,
    correct: usize,
    choices: Vec<usize>,
}

impl<'a> Question<'a> {
    pub fn new(id: usize, prompt: &'a str, correct: usize, num_answers: usize) -> Self {
        let mut q = Self { prompt, id, correct, choices: Vec::new() };
        q.gen_choices(0, num_answers);
        q
    }

    pub fn gen_choices(&mut self, min: usize, max: usize) {
        let mut rng = WyRand::new();

        let mut choices = vec![
            rng.generate_range(min..max),
            rng.generate_range(min..max),
            rng.generate_range(min..max),
        ];

        loop {
            if choices.iter().all(|&rid| rid != self.correct) {
                break;
            }

            for rid in &mut choices {
                if *rid == self.correct {
                    *rid = rng.generate_range(min..max);
                }
            }
        }

        choices.push(self.correct);
        rng.shuffle(&mut choices);
        self.choices = choices;
    }
}

#[derive(Debug)]
pub struct Answer<'a> {
    resp: &'a str,
}

impl<'a> Answer<'a> {
    pub fn new(resp: &'a str) -> Self {
        Self { resp }
    }
}