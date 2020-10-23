// Tuples group together values of different types
// Max 12 elements

pub fn run() {
  let person: (&str, &str, i8) = ("Daniel", "Indonesia", 19);
  println!("{} is from {} and he is {}", person.0, person.1, person.2);
}