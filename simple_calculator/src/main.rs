use calculator::*; 
use std::io;
use std::fs::File;
use std::env; 

fn read_numbers(file_1: &str, file_2: &str){
    


}
fn main()  -> io::Result<()> {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect(); //this returns a vector of string of all the
                                                   //command line arguments
    if (args.len() != 2) Err(); 
    else read_numbers(
    
    println!("Output has been saved to output.txt"); 
    Ok(()) 

}
