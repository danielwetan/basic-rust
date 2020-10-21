// Variables hold primitve data or reference to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
  let name = "Daniel"; // we can't change this variable
  let mut age = 19; // create mutable variable, so we can change the value
  println!("My name is {} and I am {}", name, age);

  age = 20;
  println!("My name is {} and I am {}", name, age);

  // Define constant
  const ID: i32 = 199; // integer 32
  println!("ID: {}", ID);

  // Assign multiple vars
  let ( my_name, my_age ) = ("Brad", 18);
  println!("{} is {} years old", my_name, my_age);
}