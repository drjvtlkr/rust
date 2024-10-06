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
    println!("x: {}, y: {}, z:{}",x,y,z);
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
    let mut a:i128= 20;
    //like this
    for i in 0..1000 {
        a=a+100;
    }
    println!("A final value: {}",a)


}
