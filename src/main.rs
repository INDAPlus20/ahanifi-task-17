use std::io::prelude::*;
#[cfg(testing)]
use std::time::{Duration, Instant};
use std::{cmp::min, fmt, usize};

#[derive(Clone, Copy)]
struct CharArray {
    bytes: [u8; 40], // we only have 40 chars;
    len: usize,
}

impl CharArray {
    pub fn new() -> Self {
        Self {
            bytes: [0; 40],
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, byte: u8) {
        self.bytes[self.len] = byte;
        self.len += 1;
    }
    pub fn clear(&mut self) {
        self.len = 0;
    }
    pub fn similar(&self, other: &CharArray) -> usize {
        let minlen = min(self.len(), other.len());
        for i in 0..minlen {
            if self.bytes[i] != other.bytes[i] {
                return i;
            }
        }
        return minlen;
    }
}

impl fmt::Display for CharArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.len() {
            let b = self.bytes[i];
            match b {
                0x7b => write!(f, "å"),
                0x7c => write!(f, "ä"),
                0x7d => write!(f, "ö"),
                r => write!(f, "{}", r as char),
                _ => continue,
            };
        }
        Ok(())
    }
}

impl fmt::Debug for CharArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in self.bytes.iter() {
            write!(f, "{} ", b);
        }

        Ok(())
    }
}

fn main() {
    //let now = Instant::now();

    let mut correct_words: Vec<CharArray> = Vec::with_capacity(500000);
    //let mut misspelled_words: Vec<CharArray> = Vec::with_capacity(1000);

    let mut bytes = Vec::with_capacity(5000000);
    std::io::stdin().read_to_end(&mut bytes);
    let mut byte_iter = bytes.iter();

    // let mut input = String::new();
    // std::io::stdin().read_to_string(&mut input);
    // let mut byte_iter=input.bytes();

    let mut word_buffer = CharArray::new();

    while let Some(byte) = byte_iter.next() {
        match byte {
            b'#' => {
                byte_iter.next();
                break;
            },
            b'\n' | b'\r' => {
                correct_words.push(word_buffer);
                word_buffer.clear();
            }
            0xc3 => word_buffer.push(match byte_iter.next().unwrap() {
                0xa5 => 0x7b, // å => {
                0xa4 => 0x7c, // ä => |
                0xb6 => 0x7d, // ö => }
                _ => panic!("non-allowed character!"),
            }),
            b => word_buffer.push(*b),
        }
    }

    let max_length = 40usize;

    let mut prev_word: CharArray = CharArray::new();
    let mut closest_words: Vec<CharArray> = Vec::with_capacity(128);
    let mut matrix = vec![vec![0; max_length + 1]; max_length + 1];

    //initialize basecases, i.e the margins
    for i in 0..max_length + 1 {
        matrix[i][0] = i;
        matrix[0][i] = i;
    }

    while let Some(byte) = byte_iter.next() {
        // println!("byte {}",byte);

        match byte {
            b'\n' | b'\r' => { //for every misspelled word

                let mut current_min = max_length;
                let m = word_buffer.len;

                
                
                //go through the dictionary

                for target in correct_words.iter() {
                    let n = target.len;

                    let diff = if m >= n { m - n } else { n - m };

                    if diff > current_min { // The difference in length is the minium edit distance.
                        continue;
                    }

                    let offset= prev_word.similar(target);
                    

                    for i in 1..m + 1 {
                        for j in 1 + offset..n + 1 {

                            let replace_cost =
                                if word_buffer.bytes[(i - 1)] == target.bytes[(j - 1)] {
                                    0
                                } else {
                                    1
                                };

                            let ins_del_minimum = min(matrix[(i - 1)][j] + 1, matrix[i][j - 1] + 1);
                            matrix[i][j] =
                                min(ins_del_minimum, matrix[i - 1][j - 1] + replace_cost);
                        }
                    }
                    let distance = matrix[m][n];

                    if distance == current_min {
                        closest_words.push(*target);
                    } else if distance < current_min {
                        current_min = distance;
                        closest_words.clear();
                        closest_words.push(*target);
                    }

                    prev_word = *target;
                }

                print!("{} ({}) ", word_buffer, current_min);
                for word in &closest_words {
                    print!("{} ", word);
                }
                println!();

                closest_words.clear();
                prev_word.clear();

                word_buffer.clear();
            },

            0xc3 => word_buffer.push(match byte_iter.next().unwrap() {
                0xa5 => 0x7b, // å => {
                0xa4 => 0x7c, // ä => |
                0xb6 => 0x7d, // ö => }
                _ => panic!("non-allowed character!"),
            }),

            b => word_buffer.push(*b),
        }
    }
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
    for i in 0..source.chars().count() + 1 {
        if i > 0 {
            print!("{} ", source.chars().nth(i - 1).unwrap());
        } else {
            print!("  ");
        }

        for j in 0..target.chars().count() + 1 {
            print!("{} ", matrix[i][j]);
        }
        println!();
    }
}

fn pause() {
    let mut _temp = String::new();
    std::io::stdin().read_line(&mut _temp);
}


