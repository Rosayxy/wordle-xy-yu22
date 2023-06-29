mod builtin_words;
use console;
use std::io::{self, Write};
enum Status{
    R,
    Y,
    G,
    X,
}
impl Status{
    fn parse_to_value(&self)->i32{
        match &self{
            Status::R=>1,
            Status::Y=>2,
            Status::G=>3,
            Status::X=>0,
        }
    }fn parse_to_char(&self)->char{
        match &self{
            Status::R=>'R',
            Status::Y=>'Y',
            Status::G =>'G',
            Status::X=>'X',
        }
    }pub fn new_from_value(val:i32)->Status{
        match val{
            1=>Status::R,
            2=>Status::Y,
            3=>Status::G,
            _=>Status::X,
        }
    }fn print_color(&self,ch:char){
        match &self{
            Status::R=>{
                print!("{} ", console::style(format!("{}",ch)).bold().red());
            }
            Status::Y=>{
                print!("{} ", console::style(format!("{}",ch)).bold().yellow());
            }
            Status::G=>{
                print!("{} ", console::style(format!("{}",ch)).bold().green());
            }
            Status::X=>{
                print!("{} ", console::style(format!("{}",ch)).bold().black());
            }
        }io::stdout().flush().unwrap();
    }
}

/// The main function for the Wordle game, implement your own logic here
fn update_letter_status(ans_word:&str,guess_word:&str,letter_status:&mut [i32;26]){
    let mut guess_index=0;
    for cguess in guess_word.chars(){
        let mut ans_index=0;
        let mut equal_flag=0;
        for cans in ans_word.chars(){
            if cguess==cans{
                if guess_index==ans_index{
                    letter_status[(cans as usize)-('A' as usize)]=3;
                }else if letter_status[(cans as usize)-('A' as usize)]<2{
                    letter_status[(cans as usize)-('A' as usize)]=2;
                }equal_flag=1;
                break;
            }ans_index+=1;
        }guess_index+=1;
        if equal_flag==0{
            letter_status[(cguess as usize)-('A' as usize)]=1;
        }
    }
}
fn create_guesses_ele(ans_word:&str,guess_word:&str)->Vec<(char,Status)>{
    let mut vec_guesses_ele=Vec::new();
    let mut guess_index=0;
    for c in guess_word.chars(){
        let mut ans_index=0;
        let mut equal_flag=0;
        let mut equal_not_pos_flag=0;
        for cans in ans_word.chars(){
            if c==cans{
                if guess_index==ans_index{
                    vec_guesses_ele.push(((c,Status::new_from_value(3))));
                    equal_flag=1;
                    break;
                }else{
                    equal_not_pos_flag=1;equal_flag=1;
                }
            }ans_index+=1;
        }if (equal_not_pos_flag==1){
            vec_guesses_ele.push((c,Status::new_from_value(2)));
        }if equal_flag==0{
            vec_guesses_ele.push((c,Status::new_from_value(1)));
        }guess_index+=1;
    }return vec_guesses_ele;
}
fn create_guesses_invalid(guess_word:&str)->Vec<(char,Status)>{
    let mut count=0;
    let mut v=Vec::new();
    for c in guess_word.chars(){
        v.push((c,Status::new_from_value(0)));
    }v
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let is_tty = atty::is(atty::Stream::Stdout);

    if is_tty {
        /*println!(
            "I am in a tty. Please print {}!",
            console::style("colorful characters").bold().blink().blue()
        );*/
    }/*  else {
        println!("I am not in a tty. Please print according to test requirements!");
    }*/
    let mut line = String::new();
    if is_tty {
        print!("{}", console::style("Your name: ").bold().red());
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line)?;
        println!("Welcome to wordle, {}!", line.trim());
    }    
    
//get the guessing answer from standard input:(ALL INPUTS ARE IN CAPITAL LETTERS!)
    let mut ans_word=String::new();
    if is_tty{
        print!("{}",console::style("please input the answer word:").bold().red());
        io::stdout().flush().unwrap();
    }
        line.clear();
        io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
        ans_word=line.trim().to_string();
    let mut letter_status=[0;26];
    let mut guesses:Vec<Vec<(char,Status)> >=Vec::new();
//read from input:
    let mut win_flag=0;
    let mut read_times=0;
    while(read_times<6){
        read_times+=1;
        if is_tty{
            let str=format!("your {} guess:(good luck)",read_times);
            print!("{}",console::style(str).italic().magenta());
            io::stdout().flush().unwrap();
        }      
        line.clear();  
        io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
        let guess_word=line.trim().to_string();
        //check if invalid
        let mut flag=0;
        let guess_word_to_lower=guess_word.to_lowercase();
        for it in builtin_words::ACCEPTABLE{
            if *it==guess_word_to_lower{
                flag=1;
                break;
            }
        }
        if (flag==0){
            //create element in guesses
            guesses.push(create_guesses_invalid(&guess_word));
            if (is_tty){
                println!("{}",console::style("sorry but your guess is INVALID,maybe try again?").italic().magenta());
                io::stdout().flush().unwrap();
            }else{
                println!("INVALID");
            }
        }else{
            //update letter status
            update_letter_status(&ans_word, &guess_word, &mut letter_status);
            //create element in guesses
            guesses.push(create_guesses_ele(&ans_word, &guess_word));
        }
        //output:
        if (is_tty==false)&&(flag==1){
            print!("{} ",guess_word);
            for letter in &letter_status{
                print!("{}",Status::new_from_value(*letter).parse_to_char());
            }println!("");
        }else if is_tty==true{
            for ele in &guesses{
                for it in ele{
                    it.1.print_color(it.0);
                }println!("");
            }
        }
        //check guess and answer_word is equal
        if guess_word==ans_word{
            win_flag=1;break;
        }
    }
    //print final result
    if win_flag==1{
        if is_tty{
            let str=format!("Success!You tried {} times!",read_times);
            println!("{}",console::style(str).italic().magenta());
        }else{
            println!("CORRECT {}",read_times);
        }
    }else{
        if is_tty{
            let str=format!("Sorry you failed,better luck next time!\nThe answer is {}",ans_word);
            println!("{}",console::style(str).italic().magenta());
        }else{
            println!("FAILED {}",ans_word);
        }
    }
    // example: print arguments
/*    print!("Command line arguments: ");
    for arg in std::env::args() {
        print!("{} ", arg);
    }
    println!("");*/
    // TODO: parse the arguments in `args`

    Ok(())
}
