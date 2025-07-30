use std::io;
use std::fs::File;
use std::env; 
use std::io::BufReader;
use std::io::prelude::*;
mod  calculator;

fn read_numbers(filename_1: &str, filename_2: &str, op: &str){
    let newline = b'\n';

    //getting the operations from string

    let mut Operation = match op.to_lowercase().as_str(){
        "add"  =>  calculator::Calculator_Operations::add,
        "sub"  =>  calculator::Calculator_Operations::sub,
        "mul"  =>  calculator::Calculator_Operations::mul, 
        "div"  =>  calculator::Calculator_Operations::div, 
        &_ => todo!("Create better case handling"),
    };
    let mut file_1 = File::open(&filename_1).expect("Input file 1 not found"); 
    let mut file_2 = File::open(&filename_2).expect("Input file 2 not found"); 
    let reader_1 = io::BufReader::new(file_1); 
    let reader_2 = io::BufReader::new(file_2); 
    if reader_1.lines().count() != reader_2.lines().count() {panic!("Not equal numbers in both files !!!"); }
    
    else{
        let mut num_1 = Vec::new(); 
        let mut num_2 = Vec::new(); 
        let mut output = File::create("output.txt").expect("Error creating output file"); 
        for line in reader_1.lines(){
            let num = line.expect("error").parse::<i32>().expect("Faied to parse number"); 
            num_1.push(num);
        }
        for line_2 in reader_2.lines(){
            let num = line_2.expect("Error").parse::<i32>().expect("Faied to parse number"); 
            num_2.push(num);
        }
        for i  in 0..num_1.len(){
            let  result = calculator::Calculate(num_1[i], num_2[i], Operation); 

            writeln!(output, "{}", result).expect("Failed to write to output file"); 
        }
    }
}



fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect(); //this returns a vector of string of all the
                                                   //command line arguments
    if args.len() != 3 {panic!("Argument number not equal to 3"); }
    else{ read_numbers(&args[1], &args[2], &args[3]); }
    
    
    println!("Output has been saved to output.txt"); 
    Ok(())

}
