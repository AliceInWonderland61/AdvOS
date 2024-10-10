//Implement a function sum_with_step that: 
//Take a mutable reference for the sum result, and three integers:low,high and step
#[allow(unused_variables, unused_mut)]
fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32)
{
    //This is basically what we did for the practice problems; namely Practice Problem 3
    //we calculate the sum from low to high inclusive- meanining we use the '=' in our for loop
    //then store the result in the mutable reference 
    //FORGOT ABOUT STEP; Step just determines how many numbers you skip when calculating the sum 
    *total=0;
    let mut next=low;
/*
    for number in low..=high{
        if next<=high
        {
            *total=*total+number;
            next+=step;
        }
        
    }
   // return total;
   */
   // Since I forgot the step I need to redo and rethink my approach for this problem
   //While the step we receive isn't bigger than the current highest number, then we can keep on adding the necessary numbers 
   while next<=high{
    // we add the current number next has to the total, then we just move on to the next 'step' specified from main 
    *total=*total+next;
    next=next+step;
   }
}

//This is part of the Assignment that will test our code 
fn main() {
    let mut result = 0;
    sum_with_step(&mut result, 0, 100, 1);
    println!("Sum 0 to 100, step 1: {}", result);

    result = 0;
    sum_with_step(&mut result, 0, 10, 2);
    println!("Sum 0 to 10, step 2: {}", result);

    result = 0;
    sum_with_step(&mut result, 5, 15, 3);
    println!("Sum 5 to 15, step 3: {}", result);
}