use std::io;

// This example  is a usseful application of `while` because i allows to continue
// asking for user input until the user types a specific word (in this case, "stop")

fn main () {
  let mut input = String::new();
  while input.trim() != "stop" {
    input.clear();
    println!("Please enter a worrd (type 'stop' to exit): ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    println!("You entered: {}", input);
  } 
  println!("Goodbye!");
}


// fn main () {
//   let mut i =0;
//   while i <= 5 {
//     println!("i = {}", i);
//     i += 1;
//   }
// }