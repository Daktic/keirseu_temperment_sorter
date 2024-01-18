use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;
use crossterm::{execute,
                terminal::{ClearType, Clear, SetTitle},
                style::{Print, SetForegroundColor, Color},
};


fn main() {
    // Attempt to read the file, or throw an error if it fails
    match File::open("questions.json") {
        Ok(file) => {
            // Create a buffered reader to read the file efficiently
            let reader = BufReader::new(file);

            // Parse the JSON file into the Questionnaire struct
            // Give the user feedback if it is formatted incorrectly
            let questionnaire: Questionnaire = match serde_json::from_reader(reader) {
                Ok(questionnaire) => questionnaire,
                Err(error) => {
                    println!("Error: {:?}", error);
                    return;
                }
            };

            let mut keirsey = Keirsey::new(questionnaire);

            keirsey.ask_questions();
            dbg!(keirsey.answer_grid);

            // println!("{}", keirsey)

        }
        Err(error) => println!("Error: {:?}", error),
    }
}

#[derive(Debug, Deserialize)]
struct Keirsey {
    questionnaire: Questionnaire,
    answer_grid: ScoringGrid,
}

impl Keirsey {
    fn new(questionnaire:Questionnaire) -> Keirsey {
        Keirsey {
            questionnaire,
            answer_grid: ScoringGrid::new(),
        }
    }
    fn ask_questions(&mut self) {
        //Clear the terminal and set the title
        execute!(std::io::stdout(), Clear(ClearType::All), SetTitle("Keirsey Temperament Sorter")).unwrap();
        for (i, question) in self.questionnaire.questions.iter().enumerate() {
            println!("Question {} of {}", i+1, self.questionnaire.questions.len());
            let answer = question.ask();
            self.answer_grid.add_score(Score::new(i as u32, answer));
            clear_terminal();
        }
    }
}

// impl Display for Keirsey {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let (a, b) = self.answers;
//         let total = a + b;
//         let a_percentage = a as f32 / total as f32 * 100.0;
//         let b_percentage = b as f32 / total as f32 * 100.0;
//         write!(f, "A: {:.2}%\nB: {:.2}%", a_percentage, b_percentage)
//     }
// }

#[derive(Debug, Deserialize)]
struct Questionnaire {
    questions: Vec<Question>,
}
#[derive(Debug, Deserialize)]
struct Question {
    question: String,
    options: Vec<Options>,
}

impl Question {
    fn ask(&self) -> Answer {

        set_color(Color::Green);
        println!("\n{}", self.question);

        set_color(Color::White);
        for option in &self.options {
            match option {
                Options::A(text) => println!("A: {}", text),
                Options::B(text) => println!("B: {}", text),
            }
        }

        set_color(Color::Cyan);
        // Await input from the user for the answer
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim() == "A" {
            return Answer::A;
        } else if input.trim() == "B" {
            return Answer::B;
        } else {
            clear_terminal();
            set_color(Color::Red);
            print!("Invalid input");
            set_color(Color::Reset);
            self.ask()
        }
    }
}

#[derive(Debug, Deserialize)]
struct Score {
    id: u32,
    value: Answer,
}
impl Score {
    fn new(id: u32, value: Answer) -> Score {
        Score {
            id,
            value,
        }
    }

}

#[derive(Debug, Deserialize)]
struct ScoringGrid {
    scores: Vec<Vec<Score>>,
}
impl ScoringGrid {
    fn new() -> ScoringGrid {
        ScoringGrid {
            scores: Vec::new(),
        }
    }
    fn add_score(&mut self, score: Score) {
        let score_array_length = self.scores.len();

        if score_array_length == 0 {
            self.scores.push(Vec::new());
            self.scores[score_array_length].push(score)
        } else if self.scores[score_array_length-1].len() % 7 == 0 {
            self.scores.push(Vec::new());
            self.scores[score_array_length].push(score);
        } else {
            self.scores[score_array_length-1].push(score);
        }

    }
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Options {
    A(String),
    B(String),
}
#[derive(Debug, Deserialize, PartialEq)]
enum Answer {
    A,
    B,
}

fn set_color(color: Color) {
    execute!(std::io::stdout(), SetForegroundColor(color)).unwrap();
}

fn clear_terminal() {
    execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
}

fn print_score(a:u8,b:u8) {
    set_color(Color::Reset);
    println!("Score:");
    execute!(std::io::stdout(), Print("A: ")).unwrap();
    set_color(Color::Green);
    execute!(std::io::stdout(), Print(a)).unwrap();
    print!(" ");
    set_color(Color::Reset);
    execute!(std::io::stdout(), Print("B: ")).unwrap();
    set_color(Color::Green);
    execute!(std::io::stdout(), Print(b)).unwrap();
    print!("\n\n");
}

mod tests {
    use super::*;
    #[test]
    fn test_score() {
        let score = Score::new(1, Answer::A);
        assert_eq!(score.id, 1);
        assert_eq!(score.value, Answer::A);
    }

    #[test]
    fn test_scoring_grid() {
        let mut scoring_grid = ScoringGrid::new();
        scoring_grid.add_score(Score::new(1, Answer::A));
        scoring_grid.add_score(Score::new(2, Answer::B));
        scoring_grid.add_score(Score::new(3, Answer::B));
        scoring_grid.add_score(Score::new(4, Answer::A));
        scoring_grid.add_score(Score::new(5, Answer::A));
        scoring_grid.add_score(Score::new(6, Answer::A));
        scoring_grid.add_score(Score::new(7, Answer::A));
        scoring_grid.add_score(Score::new(8, Answer::B));
        dbg!(&scoring_grid);

        assert_eq!(scoring_grid.scores.len(), 2);

    }

    #[test]
    fn test_talley() {
        let mut keirsey = Keirsey::new(Questionnaire {
            questions: vec![
                Question {
                    question: "Question 1".to_string(),
                    options: vec![
                        Options::A("Option 1".to_string()),
                        Options::B("Option 2".to_string()),
                    ],
                },
                Question {
                    question: "Question 2".to_string(),
                    options: vec![
                        Options::A("Option 1".to_string()),
                        Options::B("Option 2".to_string()),
                    ],
                },
                Question {
                    question: "Question 3".to_string(),
                    options: vec![
                        Options::A("Option 1".to_string()),
                        Options::B("Option 2".to_string()),
                    ],
                },
                Question {
                    question: "Question 4".to_string(),
                    options: vec![
                        Options::A("Option 1".to_string()),
                        Options::B("Option 2".to_string()),
                    ],
                },
                Question {
                    question: "Question 5".to_string(),
                    options: vec![
                        Options::A("Option 1".to_string()),
                        Options::B("Option 2".to_string()),
                    ],
                },
                Question {
                    question: "Question 6".to_string(),
                    options: vec![
                        Options::A("Option 1".to_string()),
                        Options::B("Option 2".to_string()),
                    ],
                },
                Question {
                    question: "Question 7".to_string(),
                    options: vec![
                        Options::A("Option 1".to_string()),
                        Options::B("Option 2".to_string()),
                    ],
                },
            ],
        });
    }
}