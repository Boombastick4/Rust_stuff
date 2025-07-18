fn add(a: i32, b: i32) -> i32{return a + b; }

fn sub(a: i32, b: i32) -> i32{return a - b; }

fn mul(a: i32, b: i32) -> i32{return a * b; }

fn div(a: i32, b: i32) -> i32{
    if(b == 0.0) return Err("Cannot difde by 0!!!"); 

    else return a - b; }
    

pub enum Calculator_Operations{
    add, 
    sub, 
    mul, 
    div,
}

pub struct Calculator_Imput{
    num_a: Vec<i32> = Vec::new(); 
    num_b: Vec<i32> = Vec::new(); 
           

}
pub fn Calculate(a: i32, b: i32, operation: Calculator_Operations) -> i32{
    match operation{
        add => return add(a, b), 
        sub => return sub(a, b), 
        mul => return mul(a, b), 
        div => return div(a, b), 
    }
}
