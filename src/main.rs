use std::cmp::min;
use std::io::prelude::*;

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer);
    let mut lines = buffer.split("\n");
    let mut correct_words: Vec<&str> = Vec::new();
    while let Some(line) = lines.next() {
        if line.trim() == "#" {
            break;
        }
        correct_words.push(line);
    }

    let max_length = 40usize;
    let mut closest_words: Vec<usize> = Vec::new(); // index in correct_words

    let mut prev_word="";

    while let Some(line) = lines.next() {
        println!("###############################################");
        if line.len() == 0 {
            break;
        }
        let source = line.trim();
        let mut matrix = vec![vec![69usize; max_length + 1]; max_length + 1];
        let mut current_min = max_length;

        //initialize basecases, i.e the margins
        for i in 0..max_length {
            matrix[i][0] = i as usize;
            matrix[0][i] = i as usize;
        }
        //go through the dictionary
        for (i, target) in correct_words.iter().enumerate() {
            let m = source.chars().count();
            let n = target.chars().count();

            if (m as isize - n as isize).abs() as usize > current_min { // The difference in length is the minium edit distance.
                continue;
            }

            let mut offset=0;
            for (index,chars) in target.chars().zip(prev_word.chars()).enumerate(){
                //print!("chars: {:#?} ",chars);
                if chars.0!=chars.1{
                    offset=index;
                    
                    break;
                }
            }
           // println!("current target {} prev target {}",target,prev_word);
            let distance = min_distance(&mut matrix, source, target, m, n,offset,current_min);
           

            if distance == current_min {
                closest_words.push(i);
            } else if distance < current_min {
                current_min = distance;
                closest_words.clear();
                closest_words.push(i);
            }

            prev_word=target;
        }

        print!("{} ({}) ", source, current_min);
        for i in &closest_words {
            print!("{} ", correct_words[*i]);
        }
        println!();
        //println!("-------------------------------");
        closest_words.clear();
        prev_word="";
        
    }
}


fn min_distance(
    matrix: &mut Vec<Vec<usize>>,
    source: &str,
    target: &str,
    m: usize,
    n: usize,
    offset:usize,
    threshhold:usize,
) -> usize {
    //println!("src {} target {}",source,target);
    //println!("m {} n {}",m,n );
    //println!("offset: {}",offset);
    let mut counter=0;

    let p= (0.5 * (threshhold as isize -(n as isize- m as isize).abs()) as f64).floor() as isize;

    for i in 1..m + 1 {
        for j in 1+offset..n + 1 {
            counter+=1;
            let first_min = min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1);
            let replace_cost;
            if source.chars().nth(i - 1) == target.chars().nth(j - 1) {
                replace_cost = 0;
            } else {
                replace_cost = 1;
            }

            matrix[i][j] = min(first_min, matrix[i - 1][j - 1] + replace_cost);
            print_pretty(matrix, source, target);
            println!("iteration {}",counter);
            // pause();
        }
    }
    print_pretty(matrix, source, target);
    println!("----------------------------");
    matrix[m][n] as usize
}

fn print_matrix(matrix: &Vec<Vec<usize>>) {
    for row in matrix {
        for el in row {
            print!("{} ", el);
        }
        println!();
    }
}

fn print_pretty(matrix: &Vec<Vec<usize>>, source: &str, target: &str) {
    print!("    ");
    for char in target.chars() {
        print!("{} ", char);
    }
    println!();
    for i in 0..source.chars().count()+1 {
        if i > 0 {
            print!("{} ", source.chars().nth(i - 1).unwrap());
        } else {
            print!("  ");
        }

        for j in 0..target.chars().count()+1 {
            print!("{} ", matrix[i][j]);
        }
        println!();
    }
}

fn pause() {
    let mut _temp = String::new();
    std::io::stdin().read_line(&mut _temp);
}
