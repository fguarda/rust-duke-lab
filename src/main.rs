fn own_vec(mut vector: &Vec<i32>) -> Vec<i32> {
  // create new vector
  let mut new_vector =  Vec::new();
  new_vector.push(10);
  new_vector
  // return
  // vector.push(10);
  // println!("{:?}", vector);    
}

fn own_integer(x: i32) {
  x + 1;    
}

fn own_string(s: String) {
  println!("{}", s)    
}

// Borrowing is the mechanism by which Rust allows you to lend ownership of a variable to a function
// or another part of your program, without actually transferring ownership of the variable.
// When you borrow a variable, you're esssentially saying
//"I want to use this variable for a little wwhile, buta I promise I won't modify it."

fn main() {
  let mut my_vec = vec![1, 2, 3, 4, 5];
  let my_int = 10;
  let my_string = String::from("Hello, world!");

  // this compiles no problem!
  own_integer(my_int);
  println!("{}", my_int);

  own_string(my_string); // take ownership of my_string
  // this is using my_string wich has also moved and is invalid
  // println!("{:?}", my_string); //this will  not compile!

 let new_vector = own_vec(&my_vec);
  // but this is using which was borrowed (moved) and yet  is now invalid
  println!("{:?}", new_vector); // this will not compile! 
}

// Borrowing is a key concept in Rust because it allows you to write code that is both safe and efficient.
// By lending ownership of a variable instead of transfering it, Rust ensures that only
// one part of your program can modify the variable at a time, wich helps prevent
// bugs and makes it easier to reason about your code.