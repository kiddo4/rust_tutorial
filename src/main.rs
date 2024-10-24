#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // println!("What is your name?");
    // let mut name = String::new();
    // let greeting = "Nice to meet you";
    // io::stdin().read_line(&mut name)
    //     .expect("Didn't Recieve Input");

    // println!("Hello {}! {}", name.trim_end(), greeting);

    // // variables
    // const ONE_MIL: u32 = 1_000_000;
    // const PI: f32 = 3.141592;
    // let age = "47";
    // let mut age: u32 = age.trim().parse()
    //     .expect("Age wasn't assigned a number");
    // age = age + 1;
    // println!("I'm {} and I want ${}", age, ONE_MIL);

    // // Datatypes
    // println!("Max u32 : {}", u32::MAX);
    // println!("Max u64 : {}", u64::MAX);
    // println!("Max f64 : {}", f64::MAX);
    // let is_true = true;
    // let my_grade = 'A';


    // Math
    // let num_1: f32 = 1.111111111111111
    // println!("f32 : {}", num_1 + 0.111111111111111);
    // let num_2: f64 = 1.111111111111111
    // println!("f64 : {}", num_2 + 0.111111111111111);
    // let num_3: u32 = 5;
    // let num_3: u32 = 4;
    // println!("5 + 4 = {}", num_3 + num_4)
    // let random_num = rand::thread_rng().gen_range(1..101);
    // println!("Random : {}", random_num)

    // if statement
    // let age = 8;
    // if (age >= 1) && (age <= 18){
    //     println!("Important Birthday");
    // } else if (age == 21) || (age == 50) {
    //     println!("Important Birthday");  
    // } else if age>= 65 {
    //     println!("Important Birthday");
    // } else {
    //     println!("Not an Important Birthday");  
    // }
    // tunary expression simulation
    // let mut my_age = 47:
    // let can_vote = if my_age >= 18{
    //     true
    // } else {
    //     false
    // };
    // println("Can Vote: {}", can_vote);

    // match 
    // let age2 = 8:
    // match age2 {
    //     1..=18 =>  println!("Important Birthday"),
    //     21 | 50 =>  println!("Important Birthday"),
    //     65..=i32::MAX =>  println!("Important Birthday"),
    //     _ => println!("Not an Important Birthday");  
    // };
    // let my_age = 18;
    // let voting_age = 18;
    // match my_age.cmp(&voting_age){
    //     Ordering::Less => println!("Can't Vote"),
    //     Ordering::Greater => println!("Can Vote"),
    //     Ordering::Equal => println!("You gained the right to vote"),
    // };

    // arrays
    // let arr_1 = [1,2,3,4];
    // println!("1st : {}", arr_1[0]);
    // println!("Lenght : {}", arr_1.len());
    // loop arrays 
    let arr_2 = [1,2,3,4,5,6,7,8,9];
    let mut loop_idx = 0;
    // loop {
    //     if arr_2[loop_idx] % 2 == 0 {
    //         loop_idx +=1;
    //         continue;
    //     }
    //     if arr_2[loop_idx] == 9{
    //         break;
    //     }
    //     println!("Val : {}", arr_2[loop_idx]);
    //     loop_idx +=1;
    // }
    // while
    // while loop_idx < arr_2.len(){
    //     println!("Arr : {}", arr_2[loop_idx]);
    //     loop_idx +=1;
    // }
    // for loops
    // for val in arr_2.iter(){
    //     println!("Val : {}", val);
    // }
    //tupple
    // let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);
    // println!("Name : {}", my_tuple.1);
    // let(v1,v2,v3) = my_tuple;
    // println!("Age : {}", v1)
}
