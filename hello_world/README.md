# Hello World
This is a beggining project to get familiar with the Rust build system and package manager `cargo`. 
## Initial dependencies
You of course need to have `rust` installed on your machine

### Linux
This is probably where most people are gonna install Rust

### Windows
You can instll rust by following ![This Link](https://www.rust-lang.org/tools/install), which will take you to the official Rust webpage, where you can download an `.exe` file

## Creating a project
to create a blank project, run: 
```
cargo new <project_name> 
TODO: add common keywords ...
```
## The `Cargo.toml` file
The `Cargo.toml` file is where all the project packages and configurations are stored. 

## Building and running
> NOTE: this is assuming you have a `src/main.rs` file, which is required when building any project
To build, simply run: 
```
rustc main.rc
```
This will work for you if you only have a simple project with no extra configuration, otherwise you can use a regular `Makefile` to build and run your code 
```
make all
```


