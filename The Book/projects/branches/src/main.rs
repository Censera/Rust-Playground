fn main() {
  let number = 3;

  if number != 0 {
    println!("Number is not a zero");
  }
  if number > 2 {
    println!("Condition was true");
  } else if number == 1 {
    println!("The number of the value is one.");
  } else {
    println!("Condition was false");
  }

  let condition = true;
  let result = if condition { 1 } else { 2 };

  println!("The result is {result}");

  let mut i = 10;
  'outter: loop {
    if i < 0 {
      break;
    }
    println!("HI! {i}");
    
    loop {
      println!("HALLO {i}");
      
      if i < 2 {break 'outter; }
    i -= 1;
    }
  }
}
