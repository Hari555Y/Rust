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
enum Compass{
  North, 
  South, 
  East,
  West
}
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
  let stuple = ("rice" , "roti" , "chawal" , "halwa");
  let ituple = (1,2,3,4,5,6,7,8);
  let mixedtuple = ("Smith" , 1, 1.2, true , (1,4));
  println!("the tuple contains name {} , boolean {}, and another tuple {}" , mixedtuple.0,mixedtuple.3 ,mixedtuple.4.1);
  add_num(3, 5);
  let st : &str = "BANANA";
  println!("the lowercase of string is {}, and length is {}" , st.to_ascii_lowercase() , st.len());
  let s1:Compass = Compass::East;
  match s1 {
    Compass::East => println!("s1 Please move to east"),
  Compass::North => println!("s1 Please move to North"),
  Compass::West => println!("s1 Please move to West"),
  Compass::South => println!("s1 Please move to South"),
    
    
  }
  
}


fn add_num(x: i8 , y : i8){
  let z = x+y;
  println!("sum is {}" , z);
}