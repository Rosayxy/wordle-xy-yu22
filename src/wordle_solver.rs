use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap},
};

pub fn possible_ans(
    acceptable_set: &BTreeSet<String>,
    guesses: &Vec<Vec<(char, super::Status)>>,
    is_print: bool,
) -> BTreeSet<String> {
    let mut vector = Vec::new();
    for i in acceptable_set {
        vector.push((i.clone().to_lowercase(), 1));
    }
    for it in guesses {
        let mut index = 0;
        while index < 5 {
            match it[index].1 {
                crate::status::Status::G => {
                    let mut vec_index = 0;
                    while vec_index < vector.len() {
                        if vector[vec_index].0.chars().nth(index).unwrap() != it[index].0 {
                            vector[vec_index].1 = 0;
                        }
                        vec_index += 1;
                    }
                }
                crate::status::Status::Y => {
                    let ch = it[index].0;
                    let mut vec_index = 0;
                    while vec_index < vector.len() {
                        if vector[vec_index].0.chars().nth(index).unwrap() == it[index].0 {
                            vector[vec_index].1 = 0;
                        } else {
                            let mut flag = 0;
                            for i in vector[vec_index].0.chars() {
                                if i == ch {
                                    flag = 1;
                                }
                            }
                            if flag == 0 {
                                vector[vec_index].1 = 0;
                            }
                        }
                        vec_index += 1;
                    }
                }
                crate::status::Status::R => {
                    let ch = it[index].0;
                    let mut vec_index = 0;
                    while vec_index < vector.len() {
                        if vector[vec_index].0.chars().nth(index).unwrap() == it[index].0 {
                            vector[vec_index].1 = 0;
                        }
                        vec_index += 1;
                    }
                }
                crate::status::Status::X => {}
            }
            index += 1;
        }
    }
    let mut possible = BTreeSet::new();
    for i in vector {
        if i.1 == 1 {
            possible.insert(i.0.clone());
        }
    }
    if is_print {
        println!("here are your possible words:");
        for i in &possible {
            println!("{}", i);
        }
    }
    possible
}

pub fn recommend(possible: &BTreeSet<String>) -> Vec<(String, f64)> {
    let mut vector = Vec::new();
    let mut total = possible.len();
    for i in possible {
        vector.push((i.clone().to_lowercase(), 0.0));
    }
    let mut vec_guesses = Vec::new();
    for i in &mut vector {
        let mut str_status = Vec::new();
        for ii in [1, 2, 3] {
            str_status.push((
                i.0.chars().nth(0).unwrap(),
                crate::status::Status::new_from_value(ii),
            ));
            for j in [1, 2, 3] {
                str_status.push((
                    i.0.chars().nth(1).unwrap(),
                    crate::status::Status::new_from_value(j),
                ));
                for k in [1, 2, 3] {
                    str_status.push((
                        i.0.chars().nth(2).unwrap(),
                        crate::status::Status::new_from_value(k),
                    ));
                    for l in [1, 2, 3] {
                        str_status.push((
                            i.0.chars().nth(3).unwrap(),
                            crate::status::Status::new_from_value(l),
                        ));
                        for m in [1, 2, 3] {
                            str_status.push((
                                i.0.chars().nth(4).unwrap(),
                                crate::status::Status::new_from_value(m),
                            ));
                            vec_guesses.push(str_status);
                            let possible_num = possible_ans(possible, &vec_guesses, false).len();
                            if possible_num != 0 {
                                i.1 += -f64::log2((possible_num as f64) / (total as f64))
                                    * (possible_num as f64)
                                    / (total as f64);
                            }
                            str_status = vec_guesses.pop().unwrap();
                            str_status.pop();
                        }
                        str_status.pop();
                    }
                    str_status.pop();
                }
                str_status.pop();
            }
            str_status.pop();
        }
    }
    vector.sort_by(|a, b| {
        if a.1 > b.1 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });
    let mut final_vec = Vec::new();
    let mut index = 0;
    while index < 5 && index < vector.len() {
        let t = vector.pop().unwrap();
        index += 1;
        final_vec.push(t);
    }
    println!("here are your five (or less) most recommended words");
    for i in &final_vec {
        println!("{:?}", i);
    }
    final_vec
}
