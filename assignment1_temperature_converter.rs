//Declare a constant for the freezing point of water in Fahrenheit (32°F).
//In rust you need to declare the bit number type (i for signed and u for unsigned)
//Floating Point Types: f32 and f64
//Since our function is using f64, we'll use it in our const value

const FREEZING_POINT:f64=32.0;


//Implement two functions:



//fahrenheit_to_celsius(f: f64) -> f64: Converts Fahrenheit to Celsius
//f is the type of parameter, the arrow is used to indicate the return type of a function so in this case we are returning a type f64 
// So this function takes a single argument 'f' which is a floating-point number of type f64 and it'll then return a value of type f64
// yeah rust doesn't like capital letters so they'll all be lowercase from here on out 
//fn Farenheight_to_Celsius(f: f64) -> f64{

fn fahrenheit_to_celsius(f: f64) -> f64{
    //convering farenehight to celsius
    //formula: subtract 30 then divide by 2.
    (f-32.0)*(5.0/9.0)
}

//Implement two functions: celsius_to_fahrenheit(c: f64) -> f64: Converts Celsius to Fahrenheit
// so here we used c as the type of parameter; same as the previous one, we take a f64 and return an f64
//lowercase cause Rust has a thing for uppercase and hates it...
//Comment this out since it won't let me run it cause I haven't used it

fn celsius_to_fahrenheit(c: f64) -> f64{
//also since it's a return value we can't add a semi-colon after it like the other regular lines of code 
//converting celsius to farenheight
//Celsius to Fahrenheit, multiply by 2 then add 30.
    (c*(9.0/5.0))+32.0
}


//In the main function:
fn main()
{
    //declare a mutable variable with a temperature in Farenheit
    //I'm under the assumption it can be any temperature ?
    let f_temp: f64=35.0;

    //Convert it to Celsius using your function
    //Just like c++ make a variable, then call it within that variable to store the answer

    let f_to_c=fahrenheit_to_celsius(f_temp);

    //print the result
   // the \u{00B0}F is to print out the degree symbol cause my keyboard doesn't have it lol
    println!("{}\u{00B0}F farenheit to celsius is: {:.2}\u{00B0}C", f_temp,f_to_c);


    // Use a loop to convert and print the next 5 integer temperatures
    //(e.g., if you start with 32°F, print conversions for 33°F, 34°F, 35°F, 36°F, and 37°F)
    //counter variable to make sure we don't go over 5 number conversions
    let mut count=0;
    //We're starting with temperature 32F as per instructions
    let mut new_temp: f64=32.0;
    loop{
        //we call our fahrenheit converter and pass on the new temperature we'll be converting
        // we will call it each time so the new temp will be updating +1 (line 5)

        let temp=fahrenheit_to_celsius(new_temp);    
        //if our counter reaches 5 then we break out of our loop    
        if count==5
        {break;}
        // if we haven't reached 5 yet we go ahead and print out the current integer conversion 
       else{
        // the \u{00B0}F is to print out the degree symbol
        println!("{}\u{00B0}F conversion: {:.2}\u{00B0}C", new_temp,temp);
        //we update our counter for the next conversion 
        count+=1;
        //update our new temperature by 1 so it can be converted each time the loop occurs (32F,33F,34F,35F,36F)
        new_temp+=1.0;        
       }
        
            
    }
}