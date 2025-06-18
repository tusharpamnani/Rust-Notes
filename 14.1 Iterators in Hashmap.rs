use::std::collections::HashMap;

fn main(){
  //create a HashMap and populate it with some key-value pairs
  let mut scores = HashMap::new();
  scores.insert("Tushar", 50);
  scores.insert("Harkirat", 60);
  scores.insert("Rust", 80);

  // Example 1: Iterating over references to key-value pairs
  println!("Iterating over key-value pairs:");
  for (key, value) in scores.iter() {
    println!("{}: {}", key, value);
  }

  // Example 2: Iterating over mutable references to key-value pairs
  println!("\nIterating over mutable key-value pairs:");
  for (key, value) in scores.iter_mut() {
    *value += 10; // increment each socre by 10
    println!("{}: {}", key, value);
  }
}

/* --------------------------------------------------------------------
            OUTPUT
Iterating over key-value pairs:
Harkirat: 60
Rust: 80
Tushar: 50

Iterating over mutable key-value pairs:
Harkirat: 70
Rust: 90
Tushar: 60
-------------------------------------------------------------------- */ 
