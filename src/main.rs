use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;
use crossterm::{execute,
                terminal::{ClearType, Clear, SetTitle},
                style::{SetForegroundColor, Color},
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

            let temperaments:Temperaments = serde_json::from_reader(BufReader::new(File::open("temperaments.json").unwrap())).unwrap();
                println!("{:?}", temperaments);

            let mut keirsey = Keirsey::new(questionnaire);

            keirsey.ask_questions();
            keirsey.answer_grid.print_temperment_info();

            println!("Would you like to take the test again? (Y/N)");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            if input.trim().to_uppercase() == "Y" || input.trim().to_uppercase() == "YES" {
                main();
            }
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
            self.answer_grid.add_score(Score::new(answer));
            clear_terminal();
        }
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
        if input.trim().to_uppercase() == "A" {
            return Answer::A;
        } else if input.trim().to_uppercase() == "B" {
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
    value: Answer,
}
impl Score {
    fn new(value: Answer) -> Score {
        Score {
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

    fn tally(&self) -> Vec<(u8, u8)> {
        let mut groups:Vec<(u8, u8)> = vec![];

        let mut i = 0;
        while i < 7 {
            let mut count_a:u8 = 0;
            let mut count_b:u8 = 0;
            for scores in &self.scores {

                match scores[i].value {
                    Answer::A => count_a += 1,
                    Answer::B => count_b += 1,
                }
            };
            match i {
                2 => {
                    groups.push((count_a, count_b));
                    groups.push((count_a+ groups[1].0, count_b + groups[1].1));
                },
                4 => {
                    groups.push((count_a, count_b));
                    groups.push((count_a + groups[4].0, count_b + groups[4].1));
                },
                6 => {
                    groups.push((count_a, count_b));
                    groups.push((count_a + groups[7].0, count_b + groups[7].1));
                },
                _ => {
                    groups.push((count_a, count_b));
                },
            }
            i += 1;
        }
        groups
    }

    fn get_temperament(&self) -> String {
        let groups = self.tally();
        let mut temperment = String::new();
        for (i, group) in groups.iter().enumerate() {
            if i == 0 {
                if group.0 > group.1 {
                    temperment.push('E');
                } else {
                    temperment.push('I');
                }
            } else if i == 1 {
                if group.0 > group.1 {
                    temperment.push('S');
                } else {
                    temperment.push('N');
                }
            } else if i == 2 {
                if group.0 > group.1 {
                    temperment.push('T');
                } else {
                    temperment.push('F');
                }
            } else if i == 3 {
                if group.0 > group.1 {
                    temperment.push('J');
                } else {
                    temperment.push('P');
                }
            }
        }
        temperment
    }

    fn print_temperment_info(&self) {
        let temperament = self.get_temperament();
        set_color(Color::White);
        println!("Your temperament is: ");
        set_color(Color::Green);
        print!("{}\n", temperament);
        set_color(Color::Reset);
        match temperament.as_str() {
            "INFP" => {Temperaments::print(TemperamentType::Idealist)},
            "INFJ" => {temperament.print(TemperamentType::Idealist)},
            "INTP" => {temperament.print(TemperamentType::Rational)},
            "INTJ" => {temperament.print(TemperamentType::Rational)},
            "ISFP" => {temperament.print(TemperamentType::Artisan)},
            "ISFJ" => {temperament.print(TemperamentType::Guardian)},
            "ISTP" => {temperament.print(TemperamentType::Artisan)},
            "ISTJ" => {temperament.print(TemperamentType::Guardian)},
            "ENFP" => {temperament.print(TemperamentType::Idealist)},
            "ENFJ" => {temperament.print(TemperamentType::Idealist)},
            "ENTP" => {temperament.print(TemperamentType::Rational)},
            "ENTJ" => {temperament.print(TemperamentType::Rational)},
            "ESFP" => {temperament.print(TemperamentType::Artisan)},
            "ESFJ" => {temperament.print(TemperamentType::Guardian)},
            "ESTP" => {temperament.print(TemperamentType::Artisan)},
            "ESTJ" => {temperament.print(TemperamentType::Guardian)},
            _ => {println!("Error: Temperament not found")}
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


#[derive(Debug, Deserialize, PartialEq)]
struct Temperaments {
    artisan: String,
    guardian: String,
    idealist: String,
    rational: String,
}
#[derive(Debug, Deserialize, PartialEq)]
enum TemperamentType {
    Artisan,
    Guardian,
    Idealist,
    Rational,
}

impl Display for TemperamentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TemperamentType::Artisan => write!(f, "- Artisan: "),
            TemperamentType::Guardian => write!(f, "- Guardian: "),
            TemperamentType::Idealist => write!(f, "- Idealist: "),
            TemperamentType::Rational => write!(f, "- Rational: "),
        }
    }

}
impl Temperaments {
    fn print(&self, temperment: TemperamentType) {
        match temperment {
            TemperamentType::Artisan => println!("{}\n{}", temperment, self.artisan),
            TemperamentType::Guardian => println!("{}\n{}", temperment, self.guardian),
            TemperamentType::Idealist => println!("{}\n{}", temperment, self.idealist),
            TemperamentType::Rational => println!("{}\n{}", temperment, self.rational),
        }
    }

}

fn set_color(color: Color) {
    execute!(std::io::stdout(), SetForegroundColor(color)).unwrap();
}

fn clear_terminal() {
    execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
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
        scoring_grid.add_score(Score::new(Answer::A));
        scoring_grid.add_score(Score::new(Answer::B));
        scoring_grid.add_score(Score::new(Answer::B));
        scoring_grid.add_score(Score::new(Answer::A));
        scoring_grid.add_score(Score::new(Answer::A));
        scoring_grid.add_score(Score::new(Answer::A));
        scoring_grid.add_score(Score::new(Answer::A));
        scoring_grid.add_score(Score::new(Answer::B));
        dbg!(&scoring_grid);

        assert_eq!(scoring_grid.scores.len(), 2);

    }

    #[test]
    fn test_talley() {
        let mut keirsey: Keirsey = Keirsey::new(
            serde_json::from_reader(
                BufReader::new(
                    File::open("questions.json").unwrap()
                )
            ).unwrap()
        );

        for (i, question) in keirsey.questionnaire.questions.iter().enumerate() {
            keirsey.answer_grid.add_score(Score::new((i+1) as u32, Answer::A));
        }

        dbg!(&keirsey);
        keirsey.answer_grid.tally();
    }
}