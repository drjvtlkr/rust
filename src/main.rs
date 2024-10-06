fn main() {
    println!("Hello, world!");

    let x = 5;
    let y = 10;
    let z = 1000.0001;
    println!("x: {x}, y: {y}, z:{z} ");
    //println is for appending newline
    /*if we did
     * print!(.......)
     * and then,
     * println!(......) the difference would not be significant
     * because the later print statement is the last one.
     */
    //or
    println!("x: {}, y: {}, z:{}", x, y, z);
    /*
     * this ensures that there is always new line
     * after the first print statement
     */

    /*  let mut a= 20;
        for i in 0..1000 {
           a=a+100;
       }
       print!("A final value: {}",a)
    */
    //this will throw an overflow error

    //to avoid overflow
    //we declare during initialization
    let mut a: i128 = 20;
    //like this
    for i in 0..1000 {
        a = a + 100;
    }
    println!("A final value: {}", a);

    let is_male: bool = true; //when this is true last statemnet will execute
                              //if is_male is set to false, then the first println will execute
    let is_above_18: bool = true;

    if is_male {
        println!("You are male");
    } else {
        println!("You are not male");
    }

    if is_male && is_above_18 {
        println!("You are a legal male");
    }

    let greeting: String = String::from("Hello world");
    println!("{}", greeting);
    //simple till here

    // print!("{}", greeting.chars().nth(1000))

    // let char1:Option<char>= greeting.chars().nth(0);
    // println!("char1:{}", char1);

    /*
    String manipulation in Rust is complex
    to allow string manipulation we do something like this
    */

    let char1   = greeting.chars().nth(1000);

    match char1 {
        Some(c)=>println!("{}",c),
        None=> println!("No character at index 1000"),
    }

    /* This is a special way of decalring the string and maknig changes to it */
}
