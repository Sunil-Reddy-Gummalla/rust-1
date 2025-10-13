use std::collections::HashMap;


fn main() {

  let mut map = HashMap::new();
  map.insert("key1", "value1");
  map.insert("key2", "value2");

  let first_value = map.get("key1");
  match first_value {
    Some(value) => println!("Found value: {}", value),
    None => {
      println!("'key1' not found, inserting default value.");
    }
  }
  println!("The value for 'key1' is: {:?}", first_value);

}