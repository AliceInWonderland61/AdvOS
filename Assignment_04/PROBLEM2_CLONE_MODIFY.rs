//ASSIGNMENT2: CLONE AND MODIFY
// Given a string, clone it and modify the cloned string by appending a new word. Print both the original string 
//and the cloned, modified string to show that the original has not been changed.
fn clone_and_modify(s: &String) -> String {
    // Your code here
    //Cloning: Deep Copy 
    //clone what s has into c_string
    let mut c_string=s.clone();
    //now we add the 2nd word to our cloned string
    c_string.push_str("World! ");
    return c_string;
}

fn main() {
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
}