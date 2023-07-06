use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};
use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};
use std::{clone, fmt, mem};
//mod super::builtin_words;
#[derive(Debug)]
pub struct DictionaryError {
    error_type: i32,
}
impl fmt::Display for DictionaryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "An error")
    }
}
impl Error for DictionaryError {}
impl DictionaryError {
    pub fn new(error_type: i32) -> DictionaryError {
        return DictionaryError { error_type };
    }
}
pub fn dictionary_error_output(r: DictionaryError, is_tty: bool) -> Box<dyn std::error::Error> {
    let dictionary_error = &r;
    let error_type = dictionary_error.error_type;
    let a_boxed_error = Box::<dyn Error>::from(r);
    if is_tty {
        if error_type == 0 {
            println!(
                "{}",
                console::style("You have redundant words in your dictionary")
                    .bold()
                    .red()
            );
        } else {
            println!(
                "{}",
                console::style("Your acceptable dictionary might has the wrong format")
                    .bold()
                    .red()
            );
        }
        io::stdout().flush().unwrap();
    }
    a_boxed_error
}
pub fn subset_error_output(is_tty: bool) -> Box<dyn std::error::Error> {
    let subset_error = DictionaryError::new(3);
    if is_tty {
        println!(
            "{}",
            console::style("Your acceptable word set is not a subset of the final set")
                .bold()
                .red()
        );
        io::stdout().flush().unwrap();
    }
    let a_boxed_error = Box::<dyn Error>::from(subset_error);
    a_boxed_error
}
pub fn create_set_from_builtin(array: &[&str]) -> BTreeSet<String> {
    let mut t = BTreeSet::new();
    for i in array {
        t.insert(i.to_string().to_uppercase());
    }
    return t;
}
pub fn create_set_from_file(file_name: String) -> Result<BTreeSet<String>, DictionaryError> {
    let mut t = BTreeSet::new();
    let mut f = File::open(file_name).unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    //check file format
    let mut file_index = 0;
    for i in buffer.chars() {
        if (file_index % 6 == 5) && (i != '\n') {
            return Err(DictionaryError { error_type: 1 });
        }
    }
    for word in buffer.split("\n") {
        if t.contains(&word.to_uppercase()) {
            return Err(DictionaryError { error_type: 0 });
        } else {
            t.insert(word.to_string().to_uppercase());
        }
    }
    return Ok(t);
}
pub fn sets_create(
    config_acceptable: &Option<String>,
    config_final: &Option<String>,
    is_tty: bool,
) -> Result<(BTreeSet<String>, BTreeSet<String>), Box<dyn std::error::Error>> {
    let mut acceptable_set = BTreeSet::new();
    let mut final_set = BTreeSet::new();
    match config_acceptable {
        None => {
            acceptable_set = create_set_from_builtin(super::builtin_words::ACCEPTABLE);
        }
        Some(str) => {
            let set = create_set_from_file(str.clone());
            match set {
                Err(r) => {
                    return Err(dictionary_error_output(r, is_tty));
                }
                Ok(r) => {
                    acceptable_set = r;
                }
            }
        }
    }
    match config_final {
        None => {
            final_set = create_set_from_builtin(super::builtin_words::FINAL);
        }
        Some(str) => {
            let set = create_set_from_file(str.clone());
            match set {
                Err(r) => {
                    return Err(dictionary_error_output(r, is_tty));
                }
                Ok(r) => {
                    final_set = r;
                }
            }
        }
    }
    //check if the two sets are subsets
    if !(final_set.is_subset(&acceptable_set)) {
        return Err(subset_error_output(is_tty));
    } else {
        return Ok((acceptable_set, final_set));
    }
}
