//Create a simple number guessing game  in Rust. The program should:

// Implement a function check_guess(guess: i32, secret: i32) -> i32 that returns:
//0 if the guess is correct
//1 if the guess is too high
//-1 if the guess is too low 

fn check_guess(guess: i32, secret: i32)->i32 {
    if guess==secret{
        //we can use the keyword return to return the values (the more you know )
        return 0
    }
    else if guess>secret{
        return 1
    }
    else
    {
        return -1
    }
}


fn main (){
    //Use a mutable variable to store a secret number (you can hardcode this)
    //kept modifying this number until it was a reachable number
 let mut secret=375; 

 //Use a loop to repeatedly:
 //Set a mutable guess variable to a number of your choice (simulating user input)
 let mut user_guess=0;
let mut guess_counter=0;
 loop{
    //First user generated guess(hardcoded)
    //kept changing the user_guess calculation to make it a reasonable user guess simulation
    user_guess=user_guess+5; //just a random calculation so the number will change during the guessing 
    // Call the check_guess function 
    let guess_fun= check_guess(user_guess, secret);
    // Use an if-else expression to pring whether the guess was correct, to high or too low
    if guess_fun==1{
        println!("Sorry {} is too high. Try again ", user_guess);
        guess_counter+=1;
        user_guess=user_guess-40;
    }
    else if guess_fun==-1{
        println!("Sorry {} is too low. Try again ", user_guess);
        guess_counter+=1;
        user_guess=user_guess+15;
        
    }
    //if the guess is correct, break the loop   
    else
    {
        println!("Congrats !! You guessed correctly :D ");
        guess_counter+=1;
        break;
    }
    
 }


 //After the loop ends, print how many guesses it took (you'll need to keep track of this variable)
println!("The number of guesses the user took to guess the number was: {} ", guess_counter);   

}

