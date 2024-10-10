//PROBLEM3: MUTABLE REFERENCE SUM
// Write a modified sum function that takes a mutable reference for the destination of the sum from low to high

#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    // Write your code here!
    // we have a total, low and high that we'll be grabbing (8.3 example)
    // so we initialize total first 
    *total=0;
    // we'll be getting a low and high number so we want to go through those numbers
    //=high since we need to take the 'high' number in consideration
    for number in low..=high
    {*total+=number;}
}

fn main() {
    // create necessary variables and test your function for low 0 high 100
    let low=0; //won't change so doesn't need to be mutable
    let  high=100;// won't change so doesn't need to be mutable
    let mut total=0; // will change so needs to be mutable

    //let mut sum_of_num;
   // sum_of_num=sum(&mut total,low,high);
   //need to call the sum function directly since it's mutable
   // kept getting errors but I was wondering why I had to make total mutable again
   //Turns out that when calling another function and sending in a value, we're not passing the actual variable
   //What we need to pass it in it's entirety- pass by reference- so the function can modify it on it's end
   sum(&mut total, low ,high);
    println!("The total sum of the numbers {} through {} is: {} ", low, high, total);

    // total should be 5050
}