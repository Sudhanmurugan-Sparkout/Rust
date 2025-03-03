use chrono::{Utc,Local};
mod struct_learn;
mod enums_learn;
fn main(){
    println!("{}",is_even(2));
    println!("{}",fib(4));
    let x = String::from("HappyCoding");
    println!("the length of the string is {}", len_of_the_string(&x));

    //Struct 
    struct User{
        name:String,
        age:u8,
    }

    let user = User{
        name: String::from("John"),
        age: 25,
    };
    println!{"the name of the user {}",user.age};
    //using the calculate_age module
    let age = struct_learn::Age{
        year_of_birth:2002,
        current_year:2025,
    };
    println!("the age of the user is {}",age.calculate_age());
    println!("the next year is {}",age.next_year(1));
    struct_learn::Age::static_method();

    //Enums

    enums_learn::current_direction(enums_learn::Direction::North);
    println!("the area of the Rectangle is {}", enums_learn::area_calculate(enums_learn::Shape::Rectangle(10.0,20.0)));
    let string = String::from("HappyCoding");
    match enums_learn::find_index_of_the_a_in_the_string(&string)
    {
        Some(index) =>println!("the index of the a in the string is {}",index + 1),
        None => println!("There is no a in the string"),
    }

    //Error Handling in Rust
    let result = enums_learn::read_file();
    match result{
        Ok(value) =>println!("{}",value),
        Err(error) =>println!("{}",error),
    }

    //package managent 
    let utc_time = Utc::now();
    println!("The current time is {}",utc_time);
    let local_time = Local::now();
    println!("The current time is {}",local_time);
}

/// Returns true if num is even, and false if num is odd.
fn is_even(num :i32)->bool{
    if num%2==0{
        return true;
    }
    return false;
    
}

// Returns the num-th number in the Fibonacci sequence.
// The Fibonacci sequence is 0,1,1,2,3,5,8,13,21,34,55,89,144, etc.
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

// Returns the number of characters in a given string.
fn len_of_the_string(str: &str)->usize{
    str.chars().count()
}