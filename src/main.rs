mod builtin_words;
use console;
use rand::Rng;
use std::io::{self, Write};
use std::collections::HashMap;
use clap::Parser;
use text_io::read;
use std::cmp::Ordering;
use std::error::Error;
use std::{fmt,mem};
use rand::prelude::*;
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
//error_display
#[derive(Debug)]
    struct ArgumentConflictError;    
    impl fmt::Display for ArgumentConflictError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "An error")
        }
    }    
    impl Error for ArgumentConflictError {}
#[derive(Debug)]
    struct ShuffleWhenNotRandomError;
    impl fmt::Display for ShuffleWhenNotRandomError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "An error")
        }
    }impl Error for ShuffleWhenNotRandomError{}    
/// The main function for the Wordle game, implement your own logic here
fn update_letter_status(ans_word:&str,guess_word:&str,letter_status:&mut [i32;26]){
    let mut guess_index=0;
    for cguess in guess_word.chars(){
        let mut ans_index=0;
        let mut equal_flag=0;
        for cans in ans_word.chars(){
            if cguess==cans{
                if guess_index==ans_index{
                    letter_status[(cans as usize)-('a' as usize)]=3;
                }else if letter_status[(cans as usize)-('a' as usize)]<2{
                    letter_status[(cans as usize)-('a' as usize)]=2;
                }equal_flag=1;
                break;
            }ans_index+=1;
        }guess_index+=1;
        if equal_flag==0{
            letter_status[(cguess as usize)-('a' as usize)]=1;
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
                    equal_flag=1;equal_not_pos_flag=0;
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
    }
    //check if redundant
    let mut count_times_guess=HashMap::new();
    for word in guess_word.chars(){
        let mut count_letter_times=count_times_guess.entry(word).or_insert(0);
        *count_letter_times+=1;
    }let mut count_times_ans=HashMap::new();
    for word in ans_word.chars(){
        let mut count_letter_times=count_times_ans.entry(word).or_insert(0);
        *count_letter_times+=1;
    }
    let mut redundant_vector:Vec<i32>=Vec::new();
    let mut index=0;
    for (c,s) in &vec_guesses_ele{
        if(s.parse_to_value()==2){
            let mut other_index=0;
            for (cc,ss) in &vec_guesses_ele{
                if (cc==c)&&(other_index!=index)&&(count_times_ans.get(c)<count_times_guess.get(c)){
                    if ss.parse_to_value()==3{
                        redundant_vector.push(index);
                    }else if other_index<index{
                        redundant_vector.push(index);
                    }else{}
                }else{}
                other_index+=1;
            }
        }else{}
        index+=1;
    }for i in redundant_vector{
        vec_guesses_ele[i as usize].1=Status::new_from_value(1);
    }
    return vec_guesses_ele;
}
fn create_guesses_invalid(guess_word:&str)->Vec<(char,Status)>{
    let mut count=0;
    let mut v=Vec::new();
    for c in guess_word.chars(){
        v.push((c,Status::new_from_value(0)));
    }v
}

#[derive(Parser)]
struct Cli{
    #[arg(short,long)]
    word: Option<String>,
    #[arg(short,long)]
    random: bool,
    #[arg(short)]
    D:bool,
    #[arg(long)]
    difficult:bool,
    #[arg(short)]
    t:bool,
    #[arg(long)]
    stats:bool,
    #[arg(short,long)]
    day:Option<i32>,
    #[arg(short,long)]
    seed:Option<u64>
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ans_word=String::new();
    let cli=Cli::parse();
    let mut is_random_mode=cli.random;
    let is_difficult=(cli.D||cli.difficult);
    let mut is_w=false;
    match cli.word{
        None=>{}
        Some(r)=>{
            is_w=true;
            ans_word=r;
        }
    }
    //let is_tty = atty::is(atty::Stream::Stdout);
    let is_tty=false;
    //matches_overall_info
    let mut matches_count=0;
    let mut matches_win_count=0;
    let mut guess_attempts_sum=0;
    let mut word_frequency=HashMap::new();
   // let randmode_index_history:Vec<i32>=Vec::new();
    let mut is_stats=(cli.stats||cli.t);
    //check if -w --day --seed valid
    if (is_random_mode)&&is_w{
        let argument_conflict_error = ArgumentConflictError;
        let a_boxed_error = Box::<dyn Error>::from(argument_conflict_error);
        if is_tty{
            print!("{}", console::style("The arguments -w/--word and -r/--random conflict").bold().red());
            io::stdout().flush().unwrap();
        }
            return Err(a_boxed_error);
    }else if (!is_random_mode)&&(cli.seed.is_some()||cli.day.is_some()){
        let shuffle_error=ShuffleWhenNotRandomError;
        let a_boxed_error = Box::<dyn Error>::from(shuffle_error);
        if is_tty{
            print!("{}", console::style("It is not possible to add -d/--day,-s/--seed arguments when not in random mode").bold().red());
            io::stdout().flush().unwrap();
        }
            return Err(a_boxed_error);
    }    
    let mut line = String::new();
    if is_tty {
        print!("{}", console::style("Your name: ").bold().red());
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line)?;
        println!("Welcome to wordle, {}!", line.trim());
    }
    loop{    
    //if is random_mode
    if is_random_mode{
        //if given --seed arguments
        let seed:u64=0xdeadbeef;
        let mut day=1+matches_count;
        match cli.seed{
            Some(r)=>{let seed:u64=r;}
            None=>{}
        }match cli.day{
            Some(d)=>{day=d;}
            _=>{}
        }
        let mut rng=rand::rngs::StdRng::seed_from_u64(seed);
        let mut vec_final=Vec::new();
        for i in builtin_words::FINAL{
            vec_final.push(i.clone());
        }
        vec_final.shuffle(&mut rng);
        /*let mut rng = rand::thread_rng();
        let mut index_rand=rng.gen_range(0..builtin_words::FINAL.len());
        loop{
        let mut regenerate=false;
        for it in &randmode_index_history{
            if *it==index_rand as i32{
                regenerate=true;
                break;
            }
        }if !regenerate{
            break;
        }index_rand=rng.gen_range(0..builtin_words::FINAL.len());
    }*/
        ans_word=vec_final[(day-1) as usize].to_string();
    }
//If no -w arguments are provided,get the guessing answer from standard input:(ALL OUTPUTS ARE IN CAPITAL LETTERS!)
    else if (!is_w)&&(!is_random_mode){    
    if is_tty{
        print!("{}",console::style("please input the answer word:").bold().red());
        io::stdout().flush().unwrap();
    }else{}
        line.clear();
        io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
        ans_word=line.trim().to_string();
    }else{}
    let mut letter_status=[0;26];
    let mut guesses:Vec<Vec<(char,Status)> >=Vec::new();
//read from input:
    let mut win_flag=0;
    let mut read_times=0;
    while (read_times<6){
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
            }else{}
        }
        //difficult mode:check if invalid
        if is_difficult&&(flag==1){
            if !guesses.is_empty(){
                let last=guesses.last().unwrap();
                let mut index_now_word=0;
                for (c,s) in last{
                    if (s.parse_to_value()==3){
                        if (*c!=guess_word.chars().nth(index_now_word).unwrap()){
                            flag=0;break;
                        }
                    }else if s.parse_to_value()==2{
                        let mut count_ans_s=0;
                        for (cc,ss) in last{
                            if (*cc==*c)&&(ss.parse_to_value()==2){
                                count_ans_s+=1;
                            }else{}
                        }let mut count_guessword_s=0;
                        let mut guess_word_index=0;
                        for ch in guess_word.chars(){
                            if (ch==*c)&&(ans_word.chars().nth(guess_word_index).unwrap()!=ch){
                                count_guessword_s+=1;
                            }guess_word_index+=1;
                        }if(count_ans_s>count_guessword_s){
                            flag=0;
                            break;
                        }
                    }index_now_word+=1;
                } 
            }
        }
        if (flag==0){
            //create element in guesses
            read_times-=1;
            //guesses.push(create_guesses_invalid(&guess_word));
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
            //update word_frequency
            let key=guess_word.to_lowercase();
            let word_frequency_ref=word_frequency.entry(key).or_insert(0);
            *word_frequency_ref+=1;
        }
        //output:
        if (is_tty==false)&&(flag==1){
            //print!("{} ",guess_word);
            let ele=guesses.last().expect("guesses_last_not_found");
                for it in ele{
                    print!("{}",it.1.parse_to_char());
                }print!(" ");
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
            let str=format!("Sorry you failed,better luck next time!\nThe answer is {}",ans_word.to_uppercase());
            println!("{}",console::style(str).italic().magenta());
        }else{
            println!("FAILED {}",ans_word.to_uppercase());
        }
    }    
    //update data for this match
    matches_count+=1;
    if win_flag==1{
        matches_win_count+=1;
        guess_attempts_sum+=read_times;
    }else{}
    //if -t,print stats
    if is_stats{
    //sort frequency
        let mut sorted_map = word_frequency.iter().collect::<Vec<_>>();
        sorted_map.sort_by(|a,b| if (b.1<a.1)||(a.1==b.1)&&(a.0<b.0){Ordering::Less}else{Ordering::Greater});
        let mut average_guess_time=0.0;
        if matches_win_count==0{
        }else{
            average_guess_time=0.5*(guess_attempts_sum as f64)*2.0/(matches_win_count as f64);
        }
        if is_tty{
        let str=format!("Here is your gaming stats:");
        println!("{}",console::style(str).italic().blue());        
        let str=format!("Success: {} Fail: {} Average guess times: {:.2}",matches_win_count,matches_count-matches_win_count,average_guess_time);
        println!("{}",console::style(str).italic().blue());
        let str=format!("Most frequently used words:");
        println!("{}",console::style(str).italic().blue());
        let mut sorted_map_index=0;
        while (sorted_map_index<5)&&(sorted_map_index<sorted_map.len()){
            let str=format!("{} {} ;",sorted_map[sorted_map_index].0,sorted_map[sorted_map_index].1);
            print!("{}",console::style(str).italic().blue());
        }println!("");
    }else{
        println!("{} {} {:.2}",matches_win_count,matches_count-matches_win_count,average_guess_time);
        let mut sorted_map_index=0;
        while (sorted_map_index<5)&&(sorted_map_index<sorted_map.len()){
            print!("{} {}",sorted_map[sorted_map_index].0,sorted_map[sorted_map_index].1);
        }println!("");
    }
 }
 //determine if break
    if is_w{
        break;
    }else if is_tty{
        let str=format!("Do you want to play another round?(y/n)(default is n)");
            println!("{}",console::style(str).italic().magenta());
        let input_break:char=read!();
        match input_break{
            'y'=>{}
            _=>{break;}
        }
    }else{
        let mut input_break=String::new();
        io::stdin().read_line(&mut input_break).unwrap();
        let input_break=input_break.trim().to_string();
        match input_break.chars().nth(0).unwrap(){
            'Y'=>{}
            _=>{break;}
        }
    }
}//end loop
    Ok(())
}
