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

            let temperaments:Temperaments = serde_json::from_reader(BufReader::new(File::open("temperaments.json").unwrap())).unwrap();
                println!("{:?}", temperaments);

            let mut keirsey = Keirsey::new(questionnaire);

            keirsey.ask_questions();
            let temperament = keirsey.answer_grid.get_temperament();
            set_color(Color::White);
            println!("Your temperament is: ");
            set_color(Color::Green);
            print!("{}", temperament);
            change_color(Color::Reset);
            match temperament.as_str() {
                "INFP" => {temperaments.print(TemperamentType::Idealist)},
                "INFJ" => {temperaments.print(TemperamentType::Idealist)},
                "INTP" => {temperaments.print(TemperamentType::Rational)},
                "INTJ" => {temperaments.print(TemperamentType::Rational)},
                "ISFP" => {temperaments.print(TemperamentType::Artisan)},
                "ISFJ" => {temperaments.print(TemperamentType::Guardian)},
                "ISTP" => {temperaments.print(TemperamentType::Artisan)},
                "ISTJ" => {temperaments.print(TemperamentType::Guardian)},
                "ENFP" => {temperaments.print(TemperamentType::Idealist)},
                "ENFJ" => {temperaments.print(TemperamentType::Idealist)},
                "ENTP" => {temperaments.print(TemperamentType::Rational)},
                "ENTJ" => {temperaments.print(TemperamentType::Rational)},
                "ESFP" => {temperaments.print(TemperamentType::Artisan)},
                "ESFJ" => {temperaments.print(TemperamentType::Guardian)},
                "ESTP" => {temperaments.print(TemperamentType::Artisan)},
                "ESTJ" => {temperaments.print(TemperamentType::Guardian)},
                _ => {println!("Error: Temperament not found")}
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
            self.answer_grid.add_score(Score::new((i+1) as u32, answer));
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
impl Temperaments {
    fn print(&self, temperment: TemperamentType) {
        match temperment {
            TemperamentType::Artisan => println!("{}", self.artisan),
            TemperamentType::Guardian => println!("{}", self.guardian),
            TemperamentType::Idealist => println!("{}", self.idealist),
            TemperamentType::Rational => println!("{}", self.rational),
        }
    }

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