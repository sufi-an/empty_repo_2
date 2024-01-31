

/* 
    format!: write formatted text to String
    print!: same as format! but the text is printed to the console (io::stdout).
    println!: same as print! but a newline is appended.
    eprint!: same as print! but the text is printed to the standard error (io::stderr).
    eprintln!: same as eprint! but a newline is appended.
 */


pub fn print_hello(){
    println!("Hello, Rust!");
    println!("The following number, value is stringyfied and taken from arguments, {} {}",123,3.14159);
}