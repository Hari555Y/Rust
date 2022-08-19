const NAME : &str = "rust";
fn main() {
  // let only can be used in function's scope 
// can't define let outside of function scope
    println!("{}" , NAME);
    let x = 25;
  // conditionals
    if x<=20 {
      println!("You are less than 20");
    }
  else{
      println!("You are less than 30");    
  }
  let mut  n = 0;  // loops, break and continue
  loop {
    n +=1;
    if n ==13 {
      continue;
    }
    println!("value of n is  {}" , n);
    if n >=15{
      break;
    }
  }
  let mut range  = 1..6;
  range = 1..111;
  let fruits = vec!["orange" , "banana", "mango", "melon"];
  // for i in fruits{
  //   println!("your fruit is {}" , i);
  // }
  for (index, i) in fruits.iter().enumerate(){
    println!("your fruit at {} is {}" , index, i);
  }
  let mut number : i32 = 1;
  while number<=15{
    println!("the number is {} ", number);
    number+=1;
  }
  
}
