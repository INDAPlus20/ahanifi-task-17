use std::{cmp::min};
use std::io::prelude::*;

fn main() {

    
    
    
    let mut buffer= String::new();
    std::io::stdin().read_to_string(&mut buffer);
    let mut lines=buffer.split("\n");
    let mut correct_words:Vec<&str>=Vec::new();
    while let Some(line)=lines.next(){
        if line.trim()=="#"{
            break;
        }
        correct_words.push(line);
    }

       

    let mut current_min=100000usize;
    let mut closest_words:Vec<usize>=Vec::new();// index in correct_words

    while let Some(line) = lines.next(){
        if line.len()==0{
            break;
        }
        let wrong=line.trim();
        for (i,correct) in correct_words.iter().enumerate(){
            let distance =return_min(wrong, correct);
            //println!("error {} source {} target {} ", distance,wrong,correct);
            
            if distance==current_min{
                closest_words.push(i);
            }
            else if distance < current_min{
                current_min=distance;
                closest_words.clear();
                closest_words.push(i);
            }
        }
        print!("{} ({}) ", wrong,current_min);
        for i in &closest_words{
            print!("{} ",correct_words[*i]);
        }
        println!();
        //println!("-------------------------------");

        current_min=10000;
        closest_words.clear();
        
    }
    
    
}

fn return_min(source:&str,target:&str) -> usize{



    //println!("src {} target {}",source,target);
    let m= source.chars().count();
    let n= target.chars().count();

    //println!("m {} n {}",m,n );
    let mut matrix= vec![vec![0isize;n+1];m+1];

    for i in 0..m+1{
        matrix[i][0]=i as isize;
    }

    // print_matrix(&matrix);
    // println!();

    for j in 0..n+1{
        matrix[0][j]=j as isize;
    }

    

    for i in 1..m+1{
        for j in 1..n+1{
            let first_min=min(matrix[i-1][j]+1, matrix[i][j-1]+1);
            let replace_cost;
            if source.chars().nth(i-1)==target.chars().nth(j-1){
                replace_cost=0;
            }else{
                replace_cost=1;
            }

            matrix[i][j]= min(first_min,matrix[i-1][j-1]+replace_cost);
            print_pretty(&matrix,source,target);
            pause();
        }
    }
    
    print_pretty(&matrix,source,target);
    println!();
    matrix[m][n] as usize

}

fn print_matrix(matrix:&Vec<Vec<isize>>){
    for row in matrix{ 
        for el in row{
            print!("{} ",el);
        }
        println!();
    }
}

fn print_pretty(matrix:&Vec<Vec<isize>>,source:&str,target:&str){
    print!("    ");
    for char in target.chars(){
        print!("{} ",char);
    }
    println!();
    for (i,row) in matrix.iter().enumerate(){
        if i>0{
            print!("{} ",source.chars().nth(i-1).unwrap());
        }
        else{
            print!("  ");
        }
        
        for el in row{
            print!("{} ",el);
        }
        println!();
    }
}

fn pause(){
    let mut _temp=String::new();
    std::io::stdin().read_line(&mut _temp);
}