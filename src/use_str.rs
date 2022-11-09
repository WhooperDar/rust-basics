
pub fn use_strings() {

    println!("====== Strings ====== ");
   // str
   // string in rust
   
   // with mutable
   let mut data_string = String::from("Hello");
   let mut name_string = String::from("Biringan");
   println!("{}", data_string);

   data_string = String::from("Darius");
   println!("Hi, {}, I'm from {} ", data_string, name_string);

   name_string = String::from("Samar");

   // push only one character (Push char)
   data_string.push('S');

   // push many character (Push string)
   data_string.push_str("sszzz");

   // Capacity in bytes
   println!("Capacity: {} bytes", data_string.capacity());

   // Check if the string is empty
   println!("Is empty: {}", data_string.is_empty());

   println!("{}", data_string);
   println!("Joke, i'm from {} btw", name_string);

   // Contains substring
   println!("Contains 'Darius' {}", data_string.contains("Darius"));

   data_string = data_string.replace("Darius", "Darius Gwapo Talaga Ako");
   
   // Replace
   println!("Replace: {}", data_string);

   //length of string
   println!("Length: {}", data_string.len());

   // Loop through string by whitespace
   for word in data_string.split_whitespace() {
       println!("{}", word);
   }

   // Create string with capacity
   let mut s = String::with_capacity(3);
   s.push('a');
   s.push('b');
   s.push('c');

   println!("{}", s);

   // Assertion testing
   assert_eq!(3, s.len());
   assert_eq!(3, s.capacity());
}