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

            let mut keirsey = Keirsey::new(questionnaire);

            keirsey.ask_questions();
            let temperament = keirsey.answer_grid.get_temperament();
            set_color(Color::White);
            println!("Your temperament is: ");
            set_color(Color::Green);
            print!("{}\n", temperament);
            set_color(Color::Reset);
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
            set_color(Color::Yellow);
            println!("\nWould you like to take the test again? (Y/N)");
            set_color(Color::Green);
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            if input.trim().to_uppercase() == "Y" {
                main();
            }
        }
        Err(error) => println!("Error: {:?}", error),
    }
}

#[derive(Debug, Deserialize, Clone)]
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


#[derive(Debug, Deserialize, Clone)]
struct Questionnaire {
    questions: Vec<Question>,
}
#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
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
            dbg!(i, group);
            if i == 0 {
                if group.0 > group.1 {
                    temperment.push('E');
                } else if group.0 < group.1 {
                    temperment.push('I');
                } else {
                    temperment.push('X');
                }
            } else if i == 3 {
                if group.0 > group.1 {
                    temperment.push('S');
                } else if group.0 < group.1 {
                    temperment.push('N');
                } else {
                    temperment.push('X');
                }
            } else if i == 6 {
                if group.0 > group.1 {
                    temperment.push('T');
                }  else if group.0 < group.1 {
                    temperment.push('F');
                } else {
                    temperment.push('X');
                }
            } else if i == 9 {
                if group.0 > group.1 {
                    temperment.push('J');
                } else if group.0 < group.1 {
                    temperment.push('P');
                } else {
                    temperment.push('X');
                }
            }
        }
        temperment
    }
}


#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
enum Options {
    A(String),
    B(String),
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
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
            TemperamentType::Artisan => write!(f, "- Artisan: \n"),
            TemperamentType::Guardian => write!(f, "- Guardian: \n"),
            TemperamentType::Idealist => write!(f, "- Idealist: \n"),
            TemperamentType::Rational => write!(f, "- Rational: \n"),
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_score() {
        let score = Score::new(Answer::A);
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

        // On the the 8th addition it will add a new array to the scores array
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

        for _ in keirsey.questionnaire.questions.iter().enumerate() {
            keirsey.answer_grid.add_score(Score::new(Answer::A));
        }

        keirsey.answer_grid.tally();

        // The tally method should return a vector of tuples with the first value being the number of A's and the second being the number of B's
        assert_eq!(keirsey.answer_grid.tally(), vec![(10, 0), (10, 0), (10, 0), (20, 0), (10, 0), (10, 0), (20, 0), (10, 0), (10, 0), (20, 0)]);
    }

    #[test]
    fn test_temperaments() {
        let mut keirseys:Vec<Keirsey> = vec![];

        let og_keirsey = Keirsey::new(
            serde_json::from_reader(
                BufReader::new(
                    File::open("questions.json").unwrap()
                )
            ).unwrap()
        );

        for _ in 0..16 {
            keirseys.push(og_keirsey.clone())
        }

        for (i, mut keirsey) in keirseys.iter_mut().enumerate() {
            for j in 0..70 {
                let pattern_variable = match i {
                    0 => false,                                                                 // ESTJ
                    1 => j % 7 == 0,                                                            // ISTJ
                    2 => (j-1) % 7 == 0 || j == 2,                                              // ENTJ
                    3 => (j-2) % 7 == 0 && j != 2 || (j-3) % 7 == 0 || j == 4,                  // ESFJ
                    4 => (j-3) % 7 == 0 && j == 4 || (j-5) % 7 == 0 || j == 6,                  // ESTP
                    5 => j % 7 == 0 || (j-1) % 7 == 0 || j == 2,                                // INTJ
                    6 => j % 7 == 0 || (j-2) % 7 == 0 && j != 2 || (j-3) % 7 == 0 || j == 4,    // ISFJ
                    7 => j % 7 == 0 || (j-5) % 7 == 0 || j == 6,                                // ISTP
                    8 => (j-1) % 7 == 0 || j == 2 || (j-3) % 7 == 0 || j == 4,                  // ENFJ
                    9 => (j-3) % 7 == 0 || j == 4 || (j-5) % 7 == 0 || j == 6,                  // ESFP
                    10 => (j-1) % 7 == 0 || j == 2 || (j-5) % 7 == 0 || j == 6,                 // ENTP
                    11 => j % 7 == 0 || (j-1) % 7 == 0 || j == 2 || (j-3) % 7 == 0 || j == 4,   // INFJ
                    12 => j % 7 == 0 || (j-3) % 7 == 0 || j == 4 || (j-5) % 7 == 0 || j == 6,   // ISFP
                    13 => j % 7 == 0 || (j-1) % 7 == 0 || j == 2 || (j-5) % 7 == 0 || j == 6,   // INTP
                    14 => (j-1) % 7 == 0 || j == 2 || (j-3) % 7 == 0 || j == 4 || (j-5) % 7 == 0 || j == 6, // ENFP
                    15 => (j-1) % 7 == 0,                                                       // EXTJ
                    _ => {true}                                                                 // INFP
                };
                let score = if pattern_variable {
                    Score::new(Answer::B)
                } else {
                    Score::new(Answer::A)
                };
                // dbg!(i+1, j+1, &score);
                keirsey.answer_grid.add_score(score);
            }
            // dbg!(i+1, &keirsey.answer_grid.tally(), &keirsey.answer_grid.get_temperament());
        }

        let answer_array = ["ESTJ","ISTJ","ENTJ","ESFJ","ESTP","INTJ","ISFJ","ISTP","ENFJ","ESFP","ENTP","INFJ","ISFP","INTP","ENFP","INFP"];

        // for (i, keirsey) in keirseys.iter().enumerate() {
        //
        //     assert_eq!(keirsey.answer_grid.get_temperament(), answer_array[i], "{:?}", keirsey.answer_grid);
        //
        // }

        let idx = 14;
        dbg!(&keirseys[idx].answer_grid.tally(), &keirseys[idx].answer_grid.get_temperament());
        assert_eq!(keirseys[idx].answer_grid.get_temperament(), answer_array[idx]);
    }
}