fn borrow_vec(vector: &Vec<i32>) {
  println!("Vetor emprestado (não modificado): {:?}", vector);
}

fn borrow_string(s: &mut String) {
  s.push_str(" Esse texto foi acrescentado.");
  println!("String dentro da função: {}", s);
}

fn own_vec(mut vector: Vec<i32>) {
  vector.push(10);
  println!("Vetor com ownership movido: {:?}", vector);
}

fn own_integer(x: i32) {
  let _ = x + 1;
}

fn own_string(s: String) {
  println!("String com ownership movido: {}", s);
}

fn main() {
  let mut my_vec = vec![1, 2, 3, 4, 5];
  let my_int = 10;
  let mut my_string = String::from("Hello, world!");

  // Inteiro é Copy, então ainda pode ser usado depois
  own_integer(my_int);
  println!("Inteiro após own_integer: {}", my_int);

  // Usando borrow_vec sem mover a propriedade
  borrow_vec(&my_vec);
  println!("Vetor após borrow_vec: {:?}", my_vec); // Ainda válido

  // Modificando string via referência mutável
  borrow_string(&mut my_string);
  println!("String após borrow_string: {}", my_string); // Modificação visível aqui

  // Demonstrando ownership movido (comentado para não causar erro de compilação)
  // own_string(my_string); // Isso moveria a propriedade de my_string
  // println!("{}", my_string); // Isso causaria erro se descomentado

  // own_vec(my_vec); // Isso moveria a propriedade de my_vec
  // println!("{:?}", my_vec); // Isso causaria erro se descomentado
}
