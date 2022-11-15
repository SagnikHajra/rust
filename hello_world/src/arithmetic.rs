pub fn run(){
    // Prints an int
    let a=10;
    let b=3;
    let c=a/b;
    println!("Result = {}", c); 
    // Prints a decimal
    let d=10.0;
    let e=3.0;
    let f=d/e;
    println!("Result = {}", f);
    // Prints a decimal after type casting
    let g=10;
    let h=2.5;
    let i=g as f64/h;
    println!("Result = {}", i);
    // Prints an int after type casting
    let j=g + h as i32;
    println!("Result = {}", j);
    // Print formatter
    println!("Result = {:.3}", f);
    println!("Result = {:8.3}", f);
    println!("Result = {:08.3}", f);
    // Positional formatter
    println!("j={1} i={0}", i, j);
}