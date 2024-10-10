//ASSIGNMENT2: Word Frequency Counter
//Create a program that:
//1. Take a string of text as input
//2. Splits the texyt into words(space as separator)
//text.split_whitespace().collect;
//3. Counts the frequency of each word
//4. Returns the word with the highest frequency and its count

//Requirements:
// Use mutable references where appropriate
//Avoid using HashMaps or complex data structures 

fn most_frequent_word(text: &str) -> (String, usize) {
    //vector would probably be easier to do
    let words: Vec<&str>=text.split_whitespace().collect();
    let mut max_count=0;

    //make an empty string 
    let mut max_word=String::new();
    // now we traverse our vector and start counting 
    //when there's more than one variable we're updating in our for loop, we need to use parenthesis 
    //needed to .enumerate() to get both the index and the current_word we're on
    for (index, &current_word) in words.iter().enumerate(){
        let mut current_count=1;

        // keeping track of the words we find 
        //.skp to count the words from the next word moving forward 
        for &repeated_word in words.iter().skip(index+1)
        {
            if current_word==repeated_word{
                current_count=current_count+1;
            }
        }
        if current_count>max_count{
            max_count=current_count;
            max_word=current_word.to_string();
        }
     
    }
        
    
    (max_word, max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}