// public function
pub fn run() {
  // Print to console
  println!("Hello from the print.rs file");

  // println!(1); -> error, me must create placeholder
  println!("Number: {}", 1);

  // Basic formatting
  // {} can be string, array, integer or anything
  println!("{} is from {}", "Daniel", "Indonesia");

  // Positional Arguments
  println!("{0} is from {1} and {1} likes to {2}", "Daniel", "Indonesia", "code");

  // Named Arguments
  println!("{name} likes to play {activity}", name = "Daniel", activity = "Baseball");

  // Placeholder traits
  println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

  // Placeholder for debug traits
  println!("{:?}", (12, true, "hello"));

  // Basic math
  println!("10 + 10 = {}", 10 + 10);
}

