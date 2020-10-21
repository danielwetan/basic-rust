/*
  Primitve Types
  ----
  Integers: u8, i8, i16, u32, i32, u64, i64, u128, i128 (numbers of bits they take in memory)
  Floats: f32, f64
  Boolean: true, false
  Characters: (char)
  Tuples
  Arrays
*/

/*
  Rust is a statically typed language,
  which means that it must know the types of all variable at compile time,
  however the compiler can usually infer what type we want to use
  based on the value and how we use it.
*/


pub fn run() {
  // By default this is "i32"
  let x = 1;

  // By default this is "f64"
  let y = 2.5;

  // Add explicit type
  let z: i64 = 233240493;

  // Find max size
  // Execute code from standard library
  println!("Max i32: {}", std::i32::MAX);
  println!("Max i64: {}", std::i64::MAX);

  // Boolean
  let is_active = true;
  // let is_active: bool = true

  // Get boolean from expression
  let is_greater: bool = 10 > 5;

  // Char
  let ab = 'a';
  // let ab = 'ab' -> error, character only contain 1 value
  let face = '\u{1F600}';


  println!("{:?}", (x, y, z, is_active, is_greater, face, ab));
}

