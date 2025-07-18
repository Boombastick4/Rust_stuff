# Simple Calculator
This program is a basic calculator that uses file inputs and command line options to add, substract, mutiply and divide numbers. 
## How it works. 
the excecutable `main.o` takes 3 input arguments: 
>- **2** input files, that are simple integer numbers, separated by a newline (`\n`) 
>- operation argument  this argument can be 1 of 4:<br>
>    *  `add` (addition) 
>    *  `sub` (subtraction)
>    *  `mul` (multiplication)
>    *  `div` (division)

The program will go through each line in the input files, perfom the operation given and then store the output into a new file (`output.txt` for example) 

Here is an example usage: 
```
main.o file_1 file_2 add

Results stored in output_file.txt
```


## Building
Same as  the `hello_world` project, this one includes a `Makefile` you can run: 
```sh
make
```
You can slso use Rust's package manager `cargo` to build and run it : 
```sh
cargo build
cargo run 

```

