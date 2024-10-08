//Implement a function is_even(n: i32) -> bool that returns true if a number is even, false otherwise.
fn is_even(n: i32)-> bool
{
    //if n mod 2 is equal to 0 that usually means that it's even so it'll come out to true otherwise it'll default to false
    n%2==0
}


fn main(){
//Create an array of 10 integer numbers of your choice
let myarray=[3,5,12,7,89,34,90,2,76,60];    //I thought they needed to be seperated by semicolons but that's only for repeated values

//Use a for loop to iterate through the array and for each number:
//For loop iterates through each element in myarray using the .iter() method
//Came across an error during the iteration. So the iter allow us to iterate but we don't get the actual number inside the index
//To fix this we pass number by reference
for &number in myarray.iter(){
//print whether it's even or odd using your iseven function
// if the number is not even then we print out that the number is not even 

 if !is_even(number) && number%3!=0 && number%5!=0
    {println!("{} is odd", number);}
    // if it is even then we print out that the number is even



    // if the number is divisible by 3, print "Fizz" instead
    else if number %3==0 && number%5!=0
    {
        println!("{} is divisibly by 3: Fizz", number);
    }
//if the number is divisible by 5, print "Buzz" instead
   else if number%5==0 && number%3!=0
    {
        println!("{} is divisible by 5: Buzz", number);
    }
//if the number is divisible by both 3 and 5 then prin "FizzBuzz" instead
    else if number%5 ==0 && number%3==0{
        println!("{} is divisible by both 3 and 5: FizzBuzz", number);
    }
    else{
        println!("{} is even", number);}
}

// Use a while loop to find and print the sum of all numbers in the array
let mut count=0;
let mut sum=0;
while count<10
{
    //can I use a for loop ????
    for add in myarray.iter(){
        sum=sum+add;
        count+=1;
    }
}
println!("After adding all the numbers in the array our total is: {}", sum);

//Use a loop to find and print the largest number in the array
let mut large=0;
let mut counter=0;
loop{
    for &biggest in myarray.iter(){
        // iterate through the array 
        //if the current number you're on is bigger than what we have saved in large, then replace the number 
        if biggest>large{
            large=biggest;
            // increment counter since we only have 10 numbers and we don't want to go over that set amount
            counter+=1;
        // if we reach 10 ( meaning the 10th number in our array; we only have 10 numbers) then we break out of the for loop
        if counter>10{
            break;
        }            
        }
       
    }
    //wasn't breaking so I added another statement for when we exit the for loop to make sure it doesn't keep going 
    break;
}
//Print the largeset number in the array
println!("The biggest number in the array is: {}", large);
}
