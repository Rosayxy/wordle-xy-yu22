use fltk::app::App;
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
use std::ops::{Deref, DerefMut};
//use fltk_table::{SmartTable, TableOpts};
//mod super::status;
use super::status::Status;
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Ops {
    Backspace,
    Enter,
    None,
}

#[derive(Debug, Clone)]
pub enum Message {
    Letter(String),
    Op(Ops),
}

pub struct MyButton {
    btn: Button,
}

impl MyButton {
    pub fn new(title: &'static str) -> MyButton {
        let mut b = MyButton {
            btn: Button::new(0, 0, 76, 0, title),
        };
        b.set_label_size(20);
        match title {
            "ENTER" | "@<-" => {
                b.resize(0, 0, 114, 0);
                b.set_color(Color::Light2);
                let shortcut = if title == "ENTER" {
                    Key::Enter
                } else {
                    Key::BackSpace
                };
                b.set_shortcut(Shortcut::None | shortcut);
            }
            "A" | "L" => {
                b.resize(0, 0, 114, 0);
                b.set_color(Color::Light3);
                b.set_shortcut(Shortcut::None | title.chars().next().unwrap());
            }
            _ => {
                b.set_color(Color::Light3);
                b.set_shortcut(Shortcut::None | title.chars().next().unwrap());
            }
        }
        b
    }
    pub fn new_table_button() -> MyButton {
        let mut b = MyButton {
            btn: Button::new(0, 0, 80, 0, ""),
        };
        b.set_label_size(20);
        b.set_color(Color::Light3);
        b
    }
}

impl Deref for MyButton {
    type Target = Button;

    fn deref(&self) -> &Self::Target {
        &self.btn
    }
}

impl DerefMut for MyButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.btn
    }
}

pub fn print_letter_status(
    current_line: usize,
    letter_status: &Vec<(char, Status)>,
    table_vec: &mut Vec<Vec<MyButton>>,
) {
    let mut col_index = 0;
    while col_index < 5 {
        table_vec[current_line as usize][col_index]
            .set_color(letter_status[col_index].1.set_status_color());
        col_index += 1;
    }
}
pub fn print_invalid(current_line: usize, table_vec: &mut Vec<Vec<MyButton>>) {
    table_vec[current_line as usize][0].set_label("I");
    table_vec[current_line as usize][1].set_label("N");
    table_vec[current_line as usize][2].set_label("V");
    table_vec[current_line as usize][3].set_label("A");
    table_vec[current_line as usize][4].set_label("L");
    crate::gui::app::sleep(1.0);
    let mut col_index = 0;
    while col_index < 5 {
        table_vec[current_line as usize][col_index].set_label("");
        col_index += 1;
    }
}
pub fn gui_init() -> (
    App,
    DoubleWindow,
    Vec<MyButton>,
    Vec<MyButton>,
    Vec<Vec<MyButton>>,
) {
    let app = app::App::default();
    let win_w = 800;
    let win_h = 1000;
    let border = 20;
    let but_row = 360;

    let mut wind = Window::default()
        .with_label("WORDLE")
        .with_size(win_w, win_h)
        .center_screen();
    wind.set_color(Color::Light3);

    let tpack = Pack::new(200, 40, win_w - 400, 360, "");
    let mut hpack = Pack::new(0, 0, win_w - 400, 60, "");
    let mut but1 = MyButton::new_table_button();
    let mut but2 = MyButton::new_table_button();
    let mut but3 = MyButton::new_table_button();
    let mut but4 = MyButton::new_table_button();
    let mut but5 = MyButton::new_table_button();
    hpack.end();
    hpack.set_type(PackType::Horizontal);
    let mut hpack = Pack::new(0, 0, win_w - 400, 60, "");
    let mut but6 = MyButton::new_table_button();
    let mut but7 = MyButton::new_table_button();
    let mut but8 = MyButton::new_table_button();
    let mut but9 = MyButton::new_table_button();
    let mut but10 = MyButton::new_table_button();
    hpack.end();
    hpack.set_type(PackType::Horizontal);
    let mut hpack = Pack::new(0, 0, win_w - 400, 60, "");
    let mut but11 = MyButton::new_table_button();
    let mut but12 = MyButton::new_table_button();
    let mut but13 = MyButton::new_table_button();
    let mut but14 = MyButton::new_table_button();
    let mut but15 = MyButton::new_table_button();
    hpack.end();
    hpack.set_type(PackType::Horizontal);
    let mut hpack = Pack::new(0, 0, win_w - 400, 60, "");
    let mut but16 = MyButton::new_table_button();
    let mut but17 = MyButton::new_table_button();
    let mut but18 = MyButton::new_table_button();
    let mut but19 = MyButton::new_table_button();
    let mut but20 = MyButton::new_table_button();
    hpack.end();
    hpack.set_type(PackType::Horizontal);
    let mut hpack = Pack::new(0, 0, win_w - 400, 60, "");
    let mut but21 = MyButton::new_table_button();
    let mut but22 = MyButton::new_table_button();
    let mut but23 = MyButton::new_table_button();
    let mut but24 = MyButton::new_table_button();
    let mut but25 = MyButton::new_table_button();
    hpack.end();
    hpack.set_type(PackType::Horizontal);
    hpack.set_type(PackType::Horizontal);
    let mut hpack = Pack::new(0, 0, win_w - 400, 60, "");
    let mut but26 = MyButton::new_table_button();
    let mut but27 = MyButton::new_table_button();
    let mut but28 = MyButton::new_table_button();
    let mut but29 = MyButton::new_table_button();
    let mut but30 = MyButton::new_table_button();
    hpack.end();
    hpack.set_type(PackType::Horizontal);
    tpack.end();
    let vpack = Pack::new(border, 400, win_w - 40, 360, "");

    let mut hpack = Pack::new(0, 0, win_w - 40, 120, "");
    let mut butq = MyButton::new("Q");
    let mut butw = MyButton::new("W");
    let mut bute = MyButton::new("E");
    let mut butr = MyButton::new("R");
    let mut butt = MyButton::new("T");
    let mut buty = MyButton::new("Y");
    let mut butu = MyButton::new("U");
    let mut buti = MyButton::new("I");
    let mut buto = MyButton::new("O");
    let mut butp = MyButton::new("P");
    hpack.end();
    hpack.set_type(PackType::Horizontal);

    let mut hpack = Pack::new(0, 0, win_w - 40, 120, "");
    let mut buta = MyButton::new("A");
    let mut buts = MyButton::new("S");
    let mut butd = MyButton::new("D");
    let mut butf = MyButton::new("F");
    let mut butg = MyButton::new("G");
    let mut buth = MyButton::new("H");
    let mut butj = MyButton::new("J");
    let mut butk = MyButton::new("K");
    let mut butl = MyButton::new("L");
    hpack.end();
    hpack.set_type(PackType::Horizontal);

    let mut hpack = Pack::new(0, 0, win_w - 40, 120, "");
    let mut butet = MyButton::new("ENTER");
    let mut butz = MyButton::new("Z");
    let mut butx = MyButton::new("X");
    let mut butc = MyButton::new("C");
    let mut butv = MyButton::new("V");
    let mut butb = MyButton::new("B");
    let mut butn = MyButton::new("N");
    let mut butm = MyButton::new("M");
    let mut butbs = MyButton::new("@<-");
    hpack.end();
    hpack.set_type(PackType::Horizontal);
    vpack.end();

    app::set_focus(&*buta);
    wind.make_resizable(true);
    wind.end();
    wind.show_with_args(&["-scheme", "gtk+", "-nokbd"]);
    wind.show();
    let mut but_vec = vec![buta, butb, butc, butd, bute, butf];
    but_vec.append(&mut vec![butg, buth, buti, butj, butk, butl, butm, butn]);
    but_vec.append(&mut vec![buto, butp, butq, butr, buts, butt, butu, butv]);
    but_vec.append(&mut vec![butw, butx, buty, butz]);
    let mut op_vec = vec![butet, butbs];
    let mut row1_vec = vec![but1, but2, but3, but4, but5];
    let mut row2_vec = vec![but6, but7, but8, but9, but10];
    let mut row3_vec = vec![but11, but12, but13, but14, but15];
    let mut row4_vec = vec![but16, but17, but18, but19, but20];
    let mut row5_vec = vec![but21, but22, but23, but24, but25];
    let mut row6_vec = vec![but26, but27, but28, but29, but30];
    let mut table_vec = vec![row1_vec, row2_vec, row3_vec, row4_vec, row5_vec,row6_vec];
    //let str=input_guess_word(app, &mut but_vec, &mut op_vec, &mut table_vec, 0);
    //println!("{}",str.clone());
    //app.run().unwrap();
    return (app, wind, but_vec, op_vec, table_vec);
}
