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

            println!("{}", keirsey)

        }
        Err(error) => println!("Error: {:?}", error),
    }
}

#[derive(Debug, Deserialize)]
struct Keirsey {
    questionnaire: Questionnaire,
    answers: (u32, u32),
}

impl Keirsey {
    fn new(questionnaire:Questionnaire) -> Keirsey {
        Keirsey {
            questionnaire,
            answers: (0, 0),
        }
    }
    fn ask_questions(&mut self) {
        //Clear the terminal and set the title
        execute!(std::io::stdout(), Clear(ClearType::All), SetTitle("Keirsey Temperament Sorter")).unwrap();
        for question in &self.questionnaire.questions {
            print_score(self.answers.0 as u8, self.answers.1 as u8);
            let (a, b) = question.ask();
            self.answers.0 += a;
            self.answers.1 += b;
            clear_terminal();
        }
    }
}

impl Display for Keirsey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (a, b) = self.answers;
        let total = a + b;
        let a_percentage = a as f32 / total as f32 * 100.0;
        let b_percentage = b as f32 / total as f32 * 100.0;
        write!(f, "A: {:.2}%\nB: {:.2}%", a_percentage, b_percentage)
    }
}

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
    fn ask(&self) -> (u32, u32) {

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
            return (1, 0);
        } else if input.trim() == "B" {
            return (0, 1);
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
    value: (u32, u32),
}
impl Score {
    fn new(id: u32, value: (u32, u32)) -> Score {
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
        let score = Score::new(1, (1, 2));
        assert_eq!(score.id, 1);
        assert_eq!(score.value, (1, 2));
    }

    #[test]
fn test_scoring_grid() {
        let mut scoring_grid = ScoringGrid::new();
        scoring_grid.add_score(Score::new(1, (0, 1)));
        scoring_grid.add_score(Score::new(2, (0, 1)));
        scoring_grid.add_score(Score::new(3, (0, 1)));
        scoring_grid.add_score(Score::new(4, (1, 0)));
        scoring_grid.add_score(Score::new(5, (1, 0)));
        scoring_grid.add_score(Score::new(6, (1, 0)));
        scoring_grid.add_score(Score::new(7, (1, 0)));
        scoring_grid.add_score(Score::new(8, (0, 1)));
        dbg!(&scoring_grid);

        assert_eq!(scoring_grid.scores.len(), 2);

    }
}