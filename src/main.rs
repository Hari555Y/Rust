#![allow(non_snake_case)]
mod hello_world;
use std::{i8 , f32};

//use std::io::stdin;
// single line comment
/* 
multiople line comments
*/

/* comment*/
// put underscore if dont want to use a variable
fn main() {
    let num: i8  = 10;
    let _name : &str = "Hari";
    let sur_name: &str = "Yadav";
   // by default any variable have i32 datatype
    hello_world::hello_world();
    print!("The numbers are {} , {}", num , num);
    println!();
   let mut name : i8 = 100;
  let is_valid : bool = true;
  let one_char  : char  = 'a'; 
  let (team, player) = ("Barca", "Busi");
  // we can change the type also 
  println!("The numbers are {}", num);
  println!("My name is {} {}" , name ,sur_name);
  println!("The numbers are {}", num);
  println!("{} , {}" , i8::MAX , f32::MIN);
  println!("boolean value is {} , char is {}", is_valid, one_char);
  println!("{} plays in {}", player, team);
}