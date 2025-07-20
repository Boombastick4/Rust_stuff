//Ok, so when makeing modules, do NOT include the mod keyword, just the file name
//rust is wonderful :>
    //operation functions
    fn add(a: &i32, b: &i32) -> i32{return a + b; }
    
    fn sub(a: &i32, b: &i32) -> i32{return a - b; }
    
    fn mul(a: &i32, b: &i32) -> i32{return a * b; }
    
    fn div(a: &i32, b: &i32) -> i32{
        if(*b == 0) {panic!("Cannot divide by 0!!!"); }
    
        else {return a - b;} }
        
    
    //enum for operations
    pub enum Calculator_Operations{
        add, 
        sub, 
        mul, 
        div,
    }
    
    
    
    
     pub fn Calculate(a: i32, b: i32, Operation: Calculator_Operations) -> i32{
        let  result = match &Operation{
            Calculator_Operations::add => add(&a, &b), 
            Calculator_Operations::sub => sub(&a, &b), 
            Calculator_Operations::mul => mul(&a, &b), 
            Calculator_Operations::div => div(&a, &b), 
        };
         return result; 
    }
