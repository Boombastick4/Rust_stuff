mod calculator{
//operation functions
fn add(a: i32, b: i32) -> i32{return a + b; }

fn sub(a: i32, b: i32) -> i32{return a - b; }

fn mul(a: i32, b: i32) -> i32{return a * b; }

fn div(a: i32, b: i32) -> i32{
    if(b == 0.0) return Err("Cannot difde by 0!!!"); 

    else return a - b; }
    

//enum for operations
pub enum Calculator_Operations{
    add, 
    sub, 
    mul, 
    div,
}




 pub fn Calculate(a: i32, b: i32, Operation: Calculator_Operations) -> i32{
    match Operation{
        add => return add(&a, &b), 
        sub => return sub(&a, &b), 
        mul => return mul(&a, &b), 
        div => return div(&a, &b), 

    }
} 
