use std::fmt;
use std::io::prelude::*;
use std::process;
use std::time::{Duration, SystemTime};
use std::{cmp, fmt::Display};
use std::{convert, usize};

use cmp::*;
use process::exit;

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]

const MAXLENGTH: usize = 40;

#[derive(Copy, Clone)]
struct charVec {
    array: [u8; 40],
    len: usize,
}

impl charVec {
    pub fn new() -> Self {
        Self {
            array: [0; 40],
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, byte: u8) {
        self.array[self.len] = byte;
        self.len += 1;
    }
    pub fn clear(&mut self) {
        self.len = 0;
    }
    pub fn similar(&self, other: &charVec) -> usize {
        let minlen = min(self.len(), other.len());
        for i in 0..minlen {
            if self.array[i] != other.array[i] {
                return i;
            }
        }
        return minlen;
    }
}

impl fmt::Display for charVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.len() {
            let b = self.array[i];
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

impl fmt::Debug for charVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in self.array.iter() {
            write!(f, "{} ", b);
        }

        Ok(())
    }
}

fn main() {
    #[cfg(feature = "bench")]
    let nowTotal = SystemTime::now();

    // Take input

    let mut wordList: Vec<charVec> = Vec::with_capacity(500_000);

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input);

    // | --

    // Convert and allocate as bytes

    let mut bytes = input.bytes();

    let mut wordBuffer = charVec::new();

    loop {
        if let Some(byte) = bytes.next() {
            match byte {
                b'#' => {
                    bytes.next();
                    wordBuffer.clear();
                    break;
                }
                b'\n' | b'\r' => {
                    wordList.push(wordBuffer);
                    wordBuffer.clear();
                }
                0xc3 => wordBuffer.push(match bytes.next().unwrap() {
                    0xa5 => 0x7b, // å => {
                    0xa4 => 0x7c, // ä => |
                    0xb6 => 0x7d, // ö => }
                    _ => panic!("non-allowed character!"),
                }),
                b => wordBuffer.push(b),
            }
        } else {
            break;
        }
    }

    // | --

    // Initialize matrix

    let mut dMatrix = [[99 as usize; MAXLENGTH + 1]; MAXLENGTH + 1];

    for i in 0..MAXLENGTH + 1 {
        dMatrix[i][0] = i;
        dMatrix[0][i] = i;
    }

    // | --
    let mut sum = 0usize;
    let mut counter = 0usize;
    let mut otherwise = 0usize;

    let mut misspelledWord = charVec::new();

    while let Some(byte) = bytes.next() {
        match byte {
            b'\n' | b'\r' => {
                // Do for each misspelled word
                let m = misspelledWord.len();
                let mut minimumDistance = MAXLENGTH;
                let mut oldtarget = charVec::new();
                let mut closestWords: Vec<charVec> = Vec::with_capacity(100);

                for target in wordList.iter() {
                    let n = target.len();
                    if (if m > n { m - n } else { n - m } > minimumDistance) {
                        continue;
                    }

                    let offset = oldtarget.similar(target);
                    let k = minimumDistance as isize;

                    if k < m as isize && k < n as isize {
                        
                        let start_first = cmp::max(1 as isize, (offset as isize - k + 1)) as usize;
                        let end_first = max(n as isize, n as isize - k) as usize;

                        
                        for i in start_first..=end_first {
                            #[cfg(feature = "debug_specific")]
                            {
                                print!(
                                    "for i: {}, ceil: {}, floor {}",
                                    i, loweredCeil, raisedFloor
                                );
                            }

                            for j in offset + 1 as usize..(i - start_first + 1) {
                                #[cfg(feature = "debug_specific")]
                                {
                                    print!("j: {} ", j);
                                }

                                let replace_cost =
                                    if misspelledWord.array[(i - 1)] != target.array[(j - 1)] {
                                        1
                                    } else {
                                        0
                                    };
                                let length_changing =
                                    cmp::min(dMatrix[i - 1][j] + 1, dMatrix[i][j - 1] + 1);

                                dMatrix[i][j] =
                                    cmp::min(dMatrix[i - 1][j - 1] + replace_cost, length_changing);
                            }
                        }

                        let end_second = (offset as isize + k) as usize;

                        for i in end_first..end_second {
                            #[cfg(feature = "debug_specific")]
                            {
                                print!(
                                    "for i: {}, ceil: {}, floor {}",
                                    i, loweredCeil, raisedFloor
                                );
                            }

                            for j in offset..=n {
                                #[cfg(feature = "debug_specific")]
                                {
                                    print!("j: {} ", j);
                                }

                                let replace_cost =
                                    if misspelledWord.array[(i - 1)] != target.array[(j - 1)] {
                                        1
                                    } else {
                                        0
                                    };
                                let length_changing =
                                    cmp::min(dMatrix[i - 1][j] + 1, dMatrix[i][j - 1] + 1);

                                dMatrix[i][j] =
                                    cmp::min(dMatrix[i - 1][j - 1] + replace_cost, length_changing);
                            }
                        }

                        for i in end_second..m + 1 {
                            #[cfg(feature = "debug_specific")]
                            {
                                print!(
                                    "for i: {}, ceil: {}, floor {}",
                                    i, loweredCeil, raisedFloor
                                );
                            }

                            for j in offset + (i - end_second)..=n {
                                #[cfg(feature = "debug_specific")]
                                {
                                    print!("j: {} ", j);
                                }

                                let replace_cost =
                                    if misspelledWord.array[(i - 1)] != target.array[(j - 1)] {
                                        1
                                    } else {
                                        0
                                    };
                                let length_changing =
                                    cmp::min(dMatrix[i - 1][j] + 1, dMatrix[i][j - 1] + 1);

                                dMatrix[i][j] =
                                    cmp::min(dMatrix[i - 1][j - 1] + replace_cost, length_changing);
                            }
                        }
                    } else {
                        for i in 1..m + 1 {
                            let raisedFloor =
                                cmp::max((i as isize - k), (offset as isize + 1)) as usize;
                            let loweredCeil = cmp::min((k + i as isize), (n as isize)) as usize;

                            #[cfg(feature = "debug_specific")]
                            {
                                print!(
                                    "for i: {}, ceil: {}, floor {}",
                                    i, loweredCeil, raisedFloor
                                );
                            }

                            for j in raisedFloor..=loweredCeil {
                                #[cfg(feature = "debug_specific")]
                                {
                                    print!("j: {} ", j);
                                }

                                let replace_cost =
                                    if misspelledWord.array[(i - 1)] != target.array[(j - 1)] {
                                        1
                                    } else {
                                        0
                                    };
                                let length_changing =
                                    cmp::min(dMatrix[i - 1][j] + 1, dMatrix[i][j - 1] + 1);

                                dMatrix[i][j] =
                                    cmp::min(dMatrix[i - 1][j - 1] + replace_cost, length_changing);
                            }
                        }
                    }

                    let distance = dMatrix[m][n];

                    if distance < minimumDistance {
                        minimumDistance = distance;
                        closestWords.clear();
                        closestWords.push(*target);
                    } else if distance == minimumDistance {
                        closestWords.push(*target);
                    }
                    oldtarget = *target;
                }

                print!("{} ({}) ", misspelledWord, minimumDistance);
                for word in closestWords {
                    print!("{} ", word);
                }
                println!();
                misspelledWord.clear();
            }
            0xc3 => misspelledWord.push(match bytes.next() {
                Some(0xa5) => 0x7b, // å => {
                Some(0xa4) => 0x7c, // ä => |
                Some(0xb6) => 0x7d, // ö => }
                _ => panic!("non-allowed character!"),
            }),
            b => misspelledWord.push(b),
        }
    }

    // "Global" Assignments

    // | --

    #[cfg(feature = "bench")]
    println!("{:?}", nowTotal.elapsed().unwrap());
    #[cfg(feature = "debug")]
    println!("counter: {} sum: {}", counter, (sum / counter));
    #[cfg(feature = "debug")]
    println!("otherwise: {}", otherwise);
}

fn printMatrix(matrix: &[[usize; MAXLENGTH + 1]; MAXLENGTH + 1], m: usize, n: usize) {
    for row in matrix[0..m].iter() {
        for el in row[0..n].iter() {
            print!("{} ", el);
        }
        println!();
    }
    println!();
}
