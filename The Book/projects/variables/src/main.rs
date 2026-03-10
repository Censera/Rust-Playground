fn main() {
  let  x = 4;
  let x = x + 1;
  
  {
    let x = x + 2;
    println!("x is {x}");
  }

  println!("x is {x}");
}
