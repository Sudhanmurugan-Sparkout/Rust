
fn main(){
    println!("{}",is_even(2));
    println!("{}",fib(4));
    println!("the length of the string is {}", len_of_the_string(String::from("HappyCoding")));
}

/// Returns true if num is even, and false if num is odd.
fn is_even(num :i32)->bool{
    if num%2==0{
        return true;
    }
    return false;
    
}

/// Returns the num-th number in the Fibonacci sequence.
/// The Fibonacci sequence is 0,1,1,2,3,5,8,13,21,34,55,89,144, etc.
fn fib(num: u32)->u32{
    if num ==0 || num==0{
        return num;
    }
    let mut first:u32 = 0;
    let mut second:u32 = 1;
     for _i in 0..(num-1){
       let temp:u32   = second;
        second = first + second;
        first = temp;
     }  
     return second;
        
}

fn len_of_the_string(str: String)->usize{
    str.chars().count()
}