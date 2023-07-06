use std::io;
use std::io::Write;
pub enum Status {
    R,
    Y,
    G,
    X,
}

impl Status {
    pub fn parse_to_value(&self) -> i32 {
        match &self {
            Status::R => 1,
            Status::Y => 2,
            Status::G => 3,
            Status::X => 0,
        }
    }
    pub fn parse_to_char(&self) -> char {
        match &self {
            Status::R => 'R',
            Status::Y => 'Y',
            Status::G => 'G',
            Status::X => 'X',
        }
    }
    pub fn new_from_value(val: i32) -> Status {
        match val {
            1 => Status::R,
            2 => Status::Y,
            3 => Status::G,
            _ => Status::X,
        }
    }
    pub fn set_status_color(&self)->fltk::enums::Color{
        match &self {
            Status::R => {
                fltk::enums::Color::Red},
            Status::Y => {
                
                fltk::enums::Color::Yellow},
            Status::G => {
                fltk::enums::Color::Green},
            Status::X => {
                fltk::enums::Color::Light3}
        }
    }
    pub fn print_color(&self, ch: char) {
        match &self {
            Status::R => {
                print!("{} ", console::style(format!("{}", ch)).bold().red());
            }
            Status::Y => {
                print!("{} ", console::style(format!("{}", ch)).bold().yellow());
            }
            Status::G => {
                print!("{} ", console::style(format!("{}", ch)).bold().green());
            }
            Status::X => {
                print!("{} ", console::style(format!("{}", ch)).bold().black());
            }
        }
        io::stdout().flush().unwrap();
    }
}