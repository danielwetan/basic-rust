/*
  Primitive string = Immutable fixed-length string somewhere in memory
  String = Growable, heap-allocated data structure - Use when you need modify string data
*/

pub fn run() {
  let hello = "Hello"; // primitve string
  let mut name = String::from("Daniel"); // growable string

  // Get length
  println!("Length: {}", hello.len());

  // push 1 character
  name.push('W');

  // push several character
  name.push_str("ZXC");

  // Capacity in bytes
  println!("Capacity: {}", name.capacity());

  // Check if empty
  println!("is Empty: {}", name.is_empty());

  // Contains
  println!("Contains 'Daniel' {}", name.contains("Daniel"));

  // Replace
  println!("Replace: {}", name.replace("Daniel", "There"));

  // Loop through string by whitespace
  for word in name.split_whitespace() {
    println!("{}", word);
  }

  // Create string with capacity
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');
  println!("{}", s);
  
  // Assertion testing
  // Test both value equal
  assert_eq!("hello", "hello"); // nothing happen if pass
  assert_eq!(2, s.len());
  assert_eq!(10, s.capacity());
}

