fn main() {
  let height = 190;
  let height = height - 20;

  let result = if height > 180 {
        "Tall"
  } else if height >= 170 {
      "Average"
  } else {
    "Short"
  };

  println!("Result: {}", result);

  let health = if height < 180 {"good"} else {"unknow"};
  println!("Health: {}", health);

  // shadowing to a different type
  let health = if height < 180 {true} else {false};
}
