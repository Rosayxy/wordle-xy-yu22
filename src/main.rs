mod builtin_words;
mod wordle_solver;
use builtin_words::ACCEPTABLE;
mod gui;
use crate::gui::{Message, Ops};
use clap::Parser;
use console;
use fltk::{
    app,
    button::Button,
    enums::{Color, Key, Shortcut},
    group::{Pack, PackType},
    output::Output,
    prelude::*,
    window::DoubleWindow,
    window::Window,
};
use gui::MyButton;
use rand::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};
use std::{clone, fmt, mem};
use std::{thread, time};
use text_io::read;
mod status;
use status::Status;
mod create_sets;
use create_sets::{create_set_from_builtin, create_set_from_file, sets_create, DictionaryError};

//when the arguments in the commandline conflicts,this error will be thrown
#[derive(Debug)]
struct ArgumentConflictError;
impl fmt::Display for ArgumentConflictError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "An error")
    }
}
impl Error for ArgumentConflictError {}
fn argument_conflict_error_output(is_tty: bool) -> Box<dyn std::error::Error> {
    let argument_conflict_error = ArgumentConflictError;
    let a_boxed_error = Box::<dyn Error>::from(argument_conflict_error);
    if is_tty {
        print!(
            "{}",
            console::style("The arguments -w/--word and -r/--random conflict")
                .bold()
                .red()
        );
        io::stdout().flush().unwrap();
    }
    a_boxed_error
}
//when you are not in random mode but you provide seed,this error will be thrown
#[derive(Debug)]
struct ShuffleWhenNotRandomError;
impl fmt::Display for ShuffleWhenNotRandomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "An error")
    }
}
impl Error for ShuffleWhenNotRandomError {}
fn shuffle_error_output(is_tty: bool) -> Box<dyn std::error::Error> {
    let shuffle_error = ShuffleWhenNotRandomError;
    let a_boxed_error = Box::<dyn Error>::from(shuffle_error);
    if is_tty {
        print!(
            "{}",
            console::style(
                "It is not possible to add -d/--day,-s/--seed arguments when not in random mode"
            )
            .bold()
            .red()
        );
        io::stdout().flush().unwrap();
    }
    a_boxed_error
}
//read in your guess_word
#[inline(always)]
fn read_guessword(is_tty: bool, read_times: usize) -> String {
    if is_tty {
        let str = format!("your {} guess:(good luck)", read_times);
        print!("{}", console::style(str).italic().magenta());
        io::stdout().flush().unwrap();
    }
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let guess_word = line.trim().to_string();
    guess_word
}
//analyze the letter status
#[inline(always)]
fn update_letter_status(ans_word: &str, guess_word: &str, letter_status: &mut [i32; 26]) {
    let mut guess_index = 0;
    for cguess in guess_word.chars() {
        let mut ans_index = 0;
        let mut equal_flag = 0;
        for cans in ans_word.chars() {
            if cguess == cans {
                if guess_index == ans_index {
                    letter_status[(cans as usize) - ('a' as usize)] = 3;
                } else if letter_status[(cans as usize) - ('a' as usize)] < 2 {
                    letter_status[(cans as usize) - ('a' as usize)] = 2;
                }
                equal_flag = 1;
                //break;
            }
            ans_index += 1;
        }
        guess_index += 1;
        if equal_flag == 0 {
            letter_status[(cguess as usize) - ('a' as usize)] = 1;
        }
    }
}
//creates a vector in which each element is the letter of the word and the corresponding status
#[inline(always)]
fn create_guesses_ele(ans_word: &str, guess_word: &str) -> Vec<(char, Status)> {
    let mut vec_guesses_ele = Vec::new();
    let mut guess_index = 0;
    for c in guess_word.chars() {
        let mut ans_index = 0;
        let mut equal_flag = 0;
        let mut equal_not_pos_flag = 0;
        for cans in ans_word.chars() {
            if c == cans {
                if guess_index == ans_index {
                    vec_guesses_ele.push(((c, Status::new_from_value(3))));
                    equal_flag = 1;
                    equal_not_pos_flag = 0;
                    break;
                } else {
                    equal_not_pos_flag = 1;
                    equal_flag = 1;
                }
            }
            ans_index += 1;
        }
        if (equal_not_pos_flag == 1) {
            vec_guesses_ele.push((c, Status::new_from_value(2)));
        }
        if equal_flag == 0 {
            vec_guesses_ele.push((c, Status::new_from_value(1)));
        }
        guess_index += 1;
    }
    //check if redundant
    let mut count_times_guess = HashMap::new();
    for word in guess_word.chars() {
        let mut count_letter_times = count_times_guess.entry(word).or_insert(0);
        *count_letter_times += 1;
    }
    let mut count_times_ans = HashMap::new();
    for word in ans_word.chars() {
        let mut count_letter_times = count_times_ans.entry(word).or_insert(0);
        *count_letter_times += 1;
    }
    let mut redundant_vector: Vec<i32> = Vec::new();
    let mut index = 0;
    for (c, s) in &vec_guesses_ele {
        if (s.parse_to_value() == 2) {
            let mut other_index = 0;
            for (cc, ss) in &vec_guesses_ele {
                if (cc == c)
                    && (other_index != index)
                    && (count_times_ans.get(c) < count_times_guess.get(c))
                {
                    if ss.parse_to_value() == 3 {
                        redundant_vector.push(index);
                    } else if other_index < index {
                        redundant_vector.push(index);
                    } else {
                    }
                } else {
                }
                other_index += 1;
            }
        } else {
        }
        index += 1;
    }
    for i in redundant_vector {
        vec_guesses_ele[i as usize].1 = Status::new_from_value(1);
    }
    return vec_guesses_ele;
}

#[derive(Serialize, Deserialize, Clone)]
struct Round {
    answer: Option<String>,
    guesses: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize)]
struct State {
    total_rounds: Option<i32>,
    games: Option<Vec<Round>>,
}
//when your input json has the wrong format,throws this error
#[derive(Debug)]
struct ParseJsonError;
impl fmt::Display for ParseJsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "An error")
    }
}
impl Error for ParseJsonError {}
fn json_error_output(error: ParseJsonError, is_tty: bool) -> Box<dyn std::error::Error> {
    if is_tty {
        println!(
            "{}",
            console::style("Your json file might has the wrong format")
                .bold()
                .red()
        );
        io::stdout().flush().unwrap();
    }
    let parse_error = error;
    let a_boxed_error = Box::<dyn Error>::from(parse_error);
    a_boxed_error
}
//from the json file,update the answords,word frequencies and guess histories
#[inline]
fn assign_state(
    str: &str,
    previous_answord: &mut BTreeSet<String>,
    word_frequency: &mut HashMap<String, i32>,
    history: &mut Vec<Round>,
) -> Result<(i32, i32, usize), ParseJsonError> {
    let mut total_rounds: i32 = 0;
    let mut win_rounds = 0;
    let mut f = std::fs::File::open(str).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    let parsed = serde_json::from_str(&buffer);
    let mut guess_attempts_sum = 0;
    let mut state: State;
    match parsed {
        Err(_) => {
            return Err(ParseJsonError {});
        }
        Ok(r) => {
            state = r;
        }
    }
    match state.total_rounds {
        None => {
            match state.games {
                None => {
                    return Ok((0, 0, 0));
                }
                Some(r) => {
                    //history_restore
                    total_rounds += r.len() as i32;
                    for i in &r {
                        history.push(i.clone());
                        match &i.answer {
                            None => {}
                            Some(rr) => {
                                previous_answord.insert(rr.clone());
                                match i.guesses.as_ref() {
                                    None => {}
                                    Some(guesses_string) => {
                                        //if wins
                                        if guesses_string.last().unwrap() == rr {
                                            win_rounds += 1;
                                            guess_attempts_sum += guesses_string.len();
                                        }
                                        for i in guesses_string {
                                            let count_frequency = word_frequency
                                                .entry(i.to_string().to_lowercase())
                                                .or_insert(0);
                                            *count_frequency += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    return Ok((total_rounds, win_rounds, guess_attempts_sum));
                }
            }
        }
        Some(total_round) => {
            match state.games {
                None => {
                    return Ok((0, 0, 0));
                }
                Some(r) => {
                    //history_restore
                    if total_round != (r.len() as i32) {
                        return Err(ParseJsonError {});
                    }
                    total_rounds += r.len() as i32;
                    for i in &r {
                        history.push(i.clone());
                        match &i.answer {
                            None => {}
                            Some(rr) => {
                                previous_answord.insert(rr.clone());
                                match i.guesses.as_ref() {
                                    None => {}
                                    Some(guesses_string) => {
                                        //if wins
                                        if guesses_string.last().unwrap() == rr {
                                            win_rounds += 1;
                                            guess_attempts_sum += guesses_string.len();
                                        }
                                        for i in guesses_string {
                                            let count_frequency = word_frequency
                                                .entry(i.to_string().to_lowercase())
                                                .or_insert(0);
                                            *count_frequency += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            return Ok((total_rounds, win_rounds, guess_attempts_sum));
        }
    }
}
//commandline options
#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    word: Option<String>,
    #[arg(short, long)]
    random: bool,
    #[arg(long, short = 'D')]
    difficult: bool,
    #[arg(long, short = 't')]
    stats: bool,
    #[arg(short, long)]
    day: Option<i32>,
    #[arg(short, long)]
    seed: Option<u64>,
    #[arg(short, long)]
    final_set: Option<String>,
    #[arg(short, long)]
    acceptable_set: Option<String>,
    #[arg(long, short = 'S')]
    state: Option<String>,
    #[arg(short, long)]
    config: Option<String>,
    #[arg(short, long)]
    gui: bool,
    #[arg(long)]
    solver: Option<usize>,
}
#[derive(Serialize, Deserialize)]
struct Config {
    random: bool,
    difficult: bool,
    stats: bool,
    day: Option<i32>,
    seed: Option<u64>,
    final_set: Option<String>,
    acceptable_set: Option<String>,
    state: Option<String>,
    word: Option<String>,
}
//parse the config file
fn parse_file_config(conf: String, is_tty: bool) -> Result<Config, ParseJsonError> {
    let mut f = std::fs::File::open(conf.clone()).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    let parse = serde_json::from_str(&buffer);
    let mut file_config: Config;
    match parse {
        Ok(r) => {
            file_config = r;
            return Ok(file_config);
        }
        _ => {
            let error = ParseJsonError {};
            if is_tty {
                println!(
                    "{}",
                    console::style("Your config json file might has the wrong format")
                        .bold()
                        .red()
                );
                io::stdout().flush().unwrap();
            }
            let parse_error = error;
            Err(parse_error)
        }
    }
}
//create current config
fn update_config<T: Clone>(file_config: Option<T>, cli_config: Option<T>) -> Option<T> {
    match cli_config {
        None => match file_config {
            None => None,
            Some(fconfig) => Some(fconfig.clone()),
        },
        Some(cli_config) => Some(cli_config.clone()),
    }
}

//generate random seeds
#[inline]
fn rand_seed_generate(
    seed: u64,
    mut day: i32,
    previous_answord: &mut BTreeSet<String>,
    final_set: &mut BTreeSet<String>,
) -> String {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let mut vec_final = Vec::new();
    let mut final_set_clone = final_set.clone();
    while !final_set_clone.is_empty() {
        vec_final.push(final_set_clone.pop_first().unwrap());
    }
    vec_final.shuffle(&mut rng);
    let mut ans_word = vec_final[((day - 1) as usize) % 2315]
        .to_string()
        .to_lowercase();
    while previous_answord.contains(&ans_word.clone()) {
        day += 1;
        ans_word = vec_final[(day - 1) as usize].to_string().to_lowercase();
    }
    ans_word
}
//input answord
#[inline(always)]
fn stdin_answord(is_tty: bool) -> String {
    if is_tty {
        print!(
            "{}",
            console::style("please input the answer word:").bold().red()
        );
        io::stdout().flush().unwrap();
    } else {
    }
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let mut ans_word = line.trim().to_string();
    ans_word
}
//output if the answord is invalid
fn ans_word_invalid_output(is_tty: bool) -> Box<dyn std::error::Error> {
    let answord_error = DictionaryError::new(4);
    if is_tty {
        println!(
            "{}",
            console::style("Your answerword is not in our range")
                .bold()
                .red()
        );
        io::stdout().flush().unwrap();
    }
    let a_boxed_error = Box::<dyn Error>::from(answord_error);
    a_boxed_error
}
//check if your input is valid
#[inline(always)]
fn invalid_check(
    acceptable_set: &BTreeSet<String>,
    guess_word: String,
    is_difficult: bool,
    guesses: &mut Vec<Vec<(char, Status)>>,
) -> bool {
    let mut flag = acceptable_set.contains(&guess_word.to_uppercase());
    //difficult mode:check if invalid
    if is_difficult && flag {
        if !guesses.is_empty() {
            let last = guesses.last().unwrap();
            let mut index_now_word = 0;
            for (c, s) in last {
                //check if all green letters are in the right positions
                if s.parse_to_value() == 3 {
                    if *c != guess_word.chars().nth(index_now_word).unwrap() {
                        flag = false;
                        break;
                    }
                } else if s.parse_to_value() == 2 {
                    //check if all yellow words appear times is above last time
                    //count the letter in last guess word
                    let mut count_ans_s = 0;
                    for (cc, ss) in last {
                        if (*cc == *c) && (ss.parse_to_value() == 2) {
                            count_ans_s += 1;
                        } else {
                        }
                    }
                    let mut count_guessword_s = 0;
                    let mut guess_word_index = 0;
                    for ch in guess_word.chars() {
                        if (ch == *c) {
                            count_guessword_s += 1;
                        }
                        guess_word_index += 1;
                    }
                    if count_ans_s > count_guessword_s {
                        flag = false;
                        break;
                    }
                }
                index_now_word += 1;
            }
        }
    }
    flag
}
//if your guess is invalid,output this
#[inline]
fn flag_invalid_print(is_tty: bool) {
    if (is_tty) {
        println!(
            "{}",
            console::style("sorry but your guess is INVALID,maybe try again?")
                .italic()
                .magenta()
        );
        io::stdout().flush().unwrap();
    } else {
        println!("INVALID");
    }
}
//output the letter status
#[inline]
fn status_output(
    is_tty: bool,
    flag: bool,
    guesses: &Vec<Vec<(char, Status)>>,
    letter_status: &[i32; 26],
) {
    if (is_tty == false) && (flag) {
        //print!("{} ",guess_word);
        let ele = guesses.last().expect("guesses_last_not_found");
        for it in ele {
            print!("{}", it.1.parse_to_char());
        }
        print!(" ");
        for letter in letter_status {
            print!("{}", Status::new_from_value(*letter).parse_to_char());
        }
        println!("");
    } else if is_tty == true {
        for ele in guesses {
            for it in ele {
                it.1.print_color((it.0 as u8 + 'A' as u8 - 'a' as u8) as char);
            }
            println!("");
        }
        let mut letter_index = 0;
        for letter in letter_status {
            let t = Status::new_from_value(*letter);
            t.print_color((letter_index as u8 + 'A' as u8) as char);
            letter_index += 1;
        }
        println!("");
    }
}
//output your final result
#[inline]
fn print_final_result(is_tty: bool, win_flag: i32, ans_word: &str, read_times: usize) {
    if win_flag == 1 {
        if is_tty {
            let str = format!("Success!You tried {} times!", read_times);
            println!("{}", console::style(str).italic().magenta());
        } else {
            println!("CORRECT {}", read_times);
        }
    } else {
        if is_tty {
            let str = format!(
                "Sorry you failed,better luck next time!\nThe answer is {}",
                ans_word.to_uppercase()
            );
            println!("{}", console::style(str).italic().magenta());
        } else {
            println!("FAILED {}", ans_word.to_uppercase());
        }
    }
}
//print the statistic of the matches
fn stats_print(
    word_frequency: &mut HashMap<String, i32>,
    matches_win_count: i32,
    guess_attempts_sum: usize,
    matches_count: i32,
    is_tty: bool,
    is_gui: bool,
) -> String {
    let mut sorted_map = word_frequency.iter().collect::<Vec<_>>();
    sorted_map.sort_by(|a, b| {
        if (b.1 < a.1) || (a.1 == b.1) && (a.0 < b.0) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    let mut average_guess_time = 0.0;
    if matches_win_count == 0 {
    } else {
        average_guess_time = 0.5 * (guess_attempts_sum as f64) * 2.0 / (matches_win_count as f64);
    }
    let mut str = format!("Here is your gaming stats:");
    if is_tty || is_gui {
        if !is_gui {
            println!("{}", console::style(str.clone()).italic().blue());
        }
        let str1 = format!(
            "Success: {} Fail: {} Average guess times: {:.2}",
            matches_win_count,
            matches_count - matches_win_count,
            average_guess_time
        );
        if !is_gui {
            println!("{}", console::style(str1).italic().blue());
        } else {
            str.push('\n');
            str.push_str(&str1);
        }
        let str2 = format!("Most frequently used words:");
        if !is_gui {
            println!("{}", console::style(str2).italic().blue());
        } else {
            str.push('\n');
            str.push_str(&str2);
        }
        let mut sorted_map_index = 0;
        while (sorted_map_index < 5) && (sorted_map_index < sorted_map.len()) {
            let str3 = format!(
                "{} {} ;",
                sorted_map[sorted_map_index].0.to_uppercase(),
                sorted_map[sorted_map_index].1
            );
            sorted_map_index += 1;
            if !is_gui {
                print!("{}", console::style(str3).italic().blue());
            } else {
                str.push_str(&str3);
            }
        }
        if !is_gui {
            println!("");
        }
    } else {
        println!(
            "{} {} {:.2}",
            matches_win_count,
            matches_count - matches_win_count,
            average_guess_time
        );
        print!("{} {}", sorted_map[0].0.to_uppercase(), sorted_map[0].1);
        let mut sorted_map_index = 1;
        while (sorted_map_index < 5) && (sorted_map_index < sorted_map.len()) {
            print!(
                " {} {}",
                sorted_map[sorted_map_index].0.to_uppercase(),
                sorted_map[sorted_map_index].1
            );
            sorted_map_index += 1;
        }
        println!("");
    }
    if is_gui == false {
        return String::from("");
    } else {
        return str;
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let is_tty = atty::is(atty::Stream::Stdout);
    //matches_overall_info
    let mut matches_count = 0;
    let mut matches_win_count = 0;
    let mut guess_attempts_sum = 0;
    let mut word_frequency = HashMap::new();
    let mut previous_answord = BTreeSet::new();
    let mut history_record: Vec<Round> = Vec::new();
    let mut previous_matches_count = 0;

    //parse config
    let mut current_config: Config;
    match cli.config {
        None => {
            current_config = Config {
                random: cli.random,
                difficult: cli.difficult,
                stats: cli.stats,
                day: cli.day,
                seed: cli.seed,
                final_set: cli.final_set,
                acceptable_set: cli.acceptable_set,
                state: cli.state,
                word: cli.word,
            };
        }
        Some(conf) => {
            match parse_file_config(conf, is_tty) {
                Err(r) => {
                    let a_boxed_error = Box::<dyn Error>::from(r);
                    return Err(a_boxed_error);
                }
                Ok(t) => {
                    let file_config = t;
                    current_config = Config {
                        random: cli.random || file_config.random,
                        difficult: cli.difficult || file_config.difficult,
                        stats: cli.stats || file_config.stats,
                        //后面几个使用泛型编程
                        day: update_config(file_config.day, cli.day),
                        seed: update_config(file_config.seed, cli.seed),
                        final_set: update_config(file_config.final_set, cli.final_set),
                        acceptable_set: update_config(
                            file_config.acceptable_set,
                            cli.acceptable_set,
                        ),
                        state: update_config(file_config.state, cli.state),
                        word: update_config(file_config.word, cli.word),
                    };
                }
            }
        }
    }
    //define flags
    let is_random_mode = current_config.random;
    let is_difficult = current_config.difficult;
    let is_state = current_config.state.is_some();
    let is_stats = current_config.stats;
    let is_w = current_config.word.is_some();
    let mut ans_word = match &current_config.word {
        None => String::new(),
        Some(s) => s.clone(),
    };

    //create final and acceptable set
    let mut acceptable_set = BTreeSet::new();
    let mut final_set = BTreeSet::new();
    //fn sets_create(config_acceptable:&Option<String>,config_final:&Option<String>)->Result<(BTreeSet<String>,BTreeSet<String>),Box<dyn std::error::Error>>
    let result = sets_create(
        &current_config.acceptable_set,
        &current_config.final_set,
        is_tty,
    );
    match result {
        Ok((a, f)) => {
            (acceptable_set, final_set) = (a, f);
        }
        Err(r) => {
            return Err(r);
        }
    }

    //judge random_mode and word conflict
    if (is_random_mode) && is_w {
        return Err(argument_conflict_error_output(is_tty));
    }
    //judge word-mode and seed,day conflict
    else if (!is_random_mode) && (current_config.seed.is_some() || current_config.day.is_some()) {
        return Err(shuffle_error_output(is_tty));
    }

    //parse json
    let mut json_file_name = String::new();
    match &current_config.state {
        None => {}
        Some(r) => {
            json_file_name = r.clone();
            let assign_state_return = assign_state(
                &r,
                &mut previous_answord,
                &mut word_frequency,
                &mut history_record,
            );
            match assign_state_return {
                Err(error) => {
                    return Err(json_error_output(error, is_tty));
                }
                Ok((add_total, add_win, add_guess_attempts_sum)) => {
                    matches_count += add_total;
                    matches_win_count += add_win;
                    previous_matches_count = add_total;
                    guess_attempts_sum += add_guess_attempts_sum;
                }
            }
        }
    }

    //game starts
    let mut line = String::new();
    loop {
        //if is random_mode
        if is_random_mode {
            //if given --seed arguments
            let mut seed: u64 = 0xdeadbeef;
            let mut day = matches_count - previous_matches_count;
            match current_config.seed {
                Some(r) => {
                    seed = r;
                }
                None => {}
            }
            match current_config.day {
                Some(d) => {
                    day = d + matches_count - previous_matches_count;
                }
                _ => {}
            }
            ans_word = rand_seed_generate(seed, day, &mut previous_answord, &mut final_set);
            //println!("day,seed,answerword:{} {} {}",day,seed,ans_word);
            day += 1;
        }
        //If no -w arguments are provided,get the guessing answerword from standard input:(ALL OUTPUTS ARE IN CAPITAL LETTERS!)
        else if (current_config.word.is_none()) && (!is_random_mode) && (!cli.gui) {
            ans_word = stdin_answord(is_tty);
            //check if ans_word is invalid
            if !final_set.contains(&ans_word.to_uppercase()) {
                return Err(ans_word_invalid_output(is_tty));
            }
        } else {
        }
        let mut letter_status = [0; 26];
        let mut guesses: Vec<Vec<(char, Status)>> = Vec::new();
        let mut guesses_in_word: Vec<String> = Vec::new();
        //read from input:
        let mut win_flag = 0;
        let mut read_times = 1;
        let mut gui_continue_flag = false;
        let mut gui_round = Round {
            answer: None,
            guesses: None,
        };
        if cli.gui {
            let (mut app, mut wind, mut but_vec, mut op_vec, mut table_vec, mut out) =
                gui::gui_init();
            let mut col_index = 0;
            let mut str = String::new();
            let (s, r) = app::channel::<Message>();
            for but in &mut *but_vec {
                let label = but.label();
                but.emit(s.clone(), Message::Letter(label.clone()));
            }
            for mut but in &mut *op_vec {
                let op = match but.label().as_str() {
                    "ENTER" => Ops::Enter,
                    "@<-" => Ops::Backspace,
                    "Play" => Ops::Play,
                    "Quit" => Ops::Quit,
                    _ => Ops::None,
                };
                but.emit(s.clone(), Message::Op(op));
            }
            let mut is_ansinput = {
                if ans_word.len() < 2 {
                    true
                } else {
                    false
                }
            };
            if ans_word.len() < 2 {
                out.set_value("please input your answord");
            }
            let mut end_game_flag = false;
            while app.wait() {
                if let Some(i) = r.recv() {
                    match i {
                        Message::Letter(s) => {
                            //println!("{}", s);
                            if col_index < 5 {
                                table_vec[read_times - 1 as usize][col_index as usize]
                                    .set_label(&s);
                                str.push(s.chars().nth(0).unwrap());
                                col_index += 1;
                            }
                        }
                        Message::Op(s) => {
                            match s {
                                Ops::Backspace => {
                                    if col_index > 0 {
                                        col_index -= 1;
                                        table_vec[read_times - 1 as usize][col_index as usize]
                                            .set_label("");
                                        app.redraw();
                                        str.pop().unwrap();
                                    }
                                }
                                Ops::Enter => {
                                    let guess_word = str.to_lowercase();
                                    if is_ansinput {
                                        ans_word = guess_word.clone();
                                        if !final_set.contains(&ans_word.to_uppercase()) {
                                            return Err(ans_word_invalid_output(is_tty));
                                        }
                                        is_ansinput = false;
                                        let mut col_index = 0;
                                        while col_index < 5 {
                                            table_vec[read_times as usize][col_index as usize]
                                                .set_label("");
                                            col_index += 1;
                                        }
                                        app.redraw();
                                        col_index = 0;
                                        str.clear();
                                        continue;
                                    }
                                    let flag = invalid_check(
                                        &acceptable_set,
                                        guess_word.clone(),
                                        is_difficult,
                                        &mut guesses,
                                    );
                                    if flag == false {
                                        //create element in guesses
                                        read_times -= 1;
                                        //flag_invalid_print(is_tty);
                                        gui::print_invalid(read_times, &mut table_vec);
                                    } else {
                                        //update letter status
                                        update_letter_status(
                                            &ans_word,
                                            &guess_word,
                                            &mut letter_status,
                                        );
                                        //create element in guesses
                                        guesses.push(create_guesses_ele(&ans_word, &guess_word));
                                        //update word_frequency
                                        let key = guess_word.to_lowercase();
                                        let word_frequency_ref =
                                            word_frequency.entry(key).or_insert(0);
                                        *word_frequency_ref += 1;
                                        guesses_in_word.push(guess_word.clone().to_uppercase());
                                        let guess_last = guesses.last().unwrap().clone();
                                        let mut col_index = 0;
                                        while col_index < 5 {
                                            table_vec[read_times - 1 as usize][col_index]
                                                .set_color(
                                                    guess_last[col_index].1.set_status_color(),
                                                );
                                            col_index += 1;
                                        }
                                        app.redraw();
                                        let mut keyboard = 0;
                                        while keyboard < 26 {
                                            but_vec[keyboard].set_color(
                                                Status::new_from_value(letter_status[keyboard])
                                                    .set_status_color(),
                                            );
                                            keyboard += 1;
                                        }
                                        app.redraw();
                                    }

                                    if guess_word.to_lowercase() == ans_word.to_lowercase() {
                                        win_flag = 1;
                                        end_game_flag = true;
                                        let mut str =
                                            format! {"Success! guess_times:{}",read_times};
                                        matches_count += 1;
                                        matches_win_count += 1;
                                        guess_attempts_sum += read_times;
                                        if is_stats {
                                            str.push_str(&stats_print(
                                                &mut word_frequency,
                                                matches_win_count,
                                                guess_attempts_sum,
                                                matches_count,
                                                is_tty,
                                                true,
                                            ));
                                        }
                                        out.set_text_size(8);
                                        out.set_value(&str);
                                        app.redraw();
                                    }
                                    read_times += 1;
                                    if read_times > 6 {
                                        end_game_flag = true;
                                        matches_count += 1;
                                        let mut str =
                                            format! {"Fail,ans:{}",ans_word.to_uppercase()};
                                        if is_stats {
                                            str.push_str(&stats_print(
                                                &mut word_frequency,
                                                matches_win_count,
                                                guess_attempts_sum,
                                                matches_count,
                                                is_tty,
                                                true,
                                            ));
                                        }
                                        out.set_text_size(8);
                                        out.set_value(&str);
                                        app.redraw();
                                    }
                                    col_index = 0;
                                    str.clear();
                                }
                                Ops::None => {}
                                Ops::Play => {
                                    if end_game_flag {
                                        if is_w {
                                            gui_continue_flag = false;
                                        } else {
                                            gui_continue_flag = true;
                                        }
                                        if is_state {
                                            gui_round = Round {
                                                answer: Some(ans_word.to_ascii_uppercase()),
                                                guesses: Some(guesses_in_word.clone()),
                                            }
                                        }
                                        ans_word.clear();
                                        app.quit();
                                    }
                                }
                                Ops::Quit => {
                                    if end_game_flag {
                                        gui_continue_flag = false;
                                        if is_state {
                                            gui_round = Round {
                                                answer: Some(ans_word.to_ascii_uppercase()),
                                                guesses: Some(guesses_in_word.clone()),
                                            }
                                        }
                                        ans_word.clear();
                                        app.quit();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            read_times = 0;
            while (read_times < 6) {
                //wordle_solver_use
                match cli.solver {
                    Some(r) => {
                        if read_times >= r {
                            let t = wordle_solver::possible_ans(&acceptable_set, &guesses, true);
                            wordle_solver::recommend(&t);
                        }
                    }
                    None => {}
                }

                read_times += 1;
                let mut guess_word = String::new();
                guess_word = read_guessword(is_tty, read_times);
                //check if invalid
                //invalid_check(acceptable_set:&BTreeSet<String>,guess_word:String,is_difficult:bool,guesses: &mut Vec<Vec<(char, Status)>>)->bool
                let flag = invalid_check(
                    &acceptable_set,
                    guess_word.clone(),
                    is_difficult,
                    &mut guesses,
                );
                if flag == false {
                    //create element in guesses
                    read_times -= 1;
                    flag_invalid_print(is_tty);
                } else {
                    //update letter status
                    update_letter_status(&ans_word, &guess_word, &mut letter_status);
                    //create element in guesses
                    guesses.push(create_guesses_ele(&ans_word, &guess_word));
                    //update word_frequency
                    let key = &guess_word;
                    let word_frequency_ref = word_frequency.entry(key.to_string()).or_insert(0);
                    *word_frequency_ref += 1;
                    guesses_in_word.push(guess_word.clone().to_uppercase());
                }
                //output:
                //status_output(is_tty:bool,flag:bool,guesses:&Vec<Vec<(char, Status)>>,letter_status:&[i32,26])
                status_output(is_tty, flag, &guesses, &letter_status);
                if guess_word == ans_word {
                    win_flag = 1;
                    break;
                }
            }
            //check guess and answer_word is equal
        }
        //print final result
        if !cli.gui {
            print_final_result(is_tty, win_flag, &ans_word, read_times);
        }
        //update data for this match
        if !cli.gui {
            matches_count += 1;
            if win_flag == 1 {
                matches_win_count += 1;
                guess_attempts_sum += read_times;
                // println!("guess_attempts_sum:{}",guess_attempts_sum);
            } else {
            }
        }
        //if --stats,update json
        if is_state {
            let mut this_round_data: Round;
            if cli.gui {
                this_round_data = gui_round;
            } else {
                this_round_data = Round {
                    answer: Some(ans_word.to_uppercase()),
                    guesses: Some(guesses_in_word.clone()),
                };
            }
            history_record.push(this_round_data);
            let state_update = State {
                total_rounds: Some(matches_count),
                games: Some(history_record.clone()),
            };
            let write_json = serde_json::to_string(&state_update).unwrap();
            let mut buffer = File::create(json_file_name.clone())?;
            buffer.write(write_json.as_bytes())?;
        }
        //if -t,print stats
        if is_stats && (!cli.gui) {
            //sort frequency
            //fn stats_print(&mut word_frequency: HashMap<String, i32>,matches_win_count,guess_attempts_sum,matches_count);
            stats_print(
                &mut word_frequency,
                matches_win_count,
                guess_attempts_sum,
                matches_count,
                is_tty,
                false,
            );
        }
        //determine if break
        if is_w {
            break;
        }
        if cli.gui {
            if gui_continue_flag {
                continue;
            } else {
                break;
            }
        } else if is_tty {
            let str = format!("Do you want to play another round?(y/n)(default is n)");
            println!("{}", console::style(str).italic().magenta());
            let input_break: char = read!();
            match input_break {
                'y' => {}
                _ => {
                    break;
                }
            }
        } else {
            let mut input_break = String::new();
            io::stdin().read_line(&mut input_break).unwrap();
            let input_break = input_break.trim().to_string();
            match input_break.chars().nth(0).unwrap() {
                'Y' => {}
                _ => {
                    break;
                }
            }
        }
    } //end loop
    Ok(())
}
