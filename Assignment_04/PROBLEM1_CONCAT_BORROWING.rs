//Write a function that concatenates two strings without taking ownership, 
//i.e., by borrowing.
//PROBLEM 1: STRING CONCATENATION WITH BORROWING

fn concat_strings(s1: &String, s2: &String) -> String {
    // Your code here
    // we can use references
    let mut con=String::from(s1);
    con.push_str(s2);
    return con
  
}

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"
}