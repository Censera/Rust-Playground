fn main() {
    println!("Hello, world!");

    another_function();

    add_two(3);
}

fn another_function() {
  println!("Another Hello!");
}

fn add_two(x: i32) {
  println!("{}", x + plus_one(1));
}

fn plus_one(x: i32) -> i32 { x + 1 } // Cool
