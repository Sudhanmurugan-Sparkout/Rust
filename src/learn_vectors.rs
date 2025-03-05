use std::vec;

pub fn vectors(){
    let mut x = Vec::new();
    let y = vec!(1,2,3);
    x.push(1);
    x.push(2);
    println!("{}",x[0]);
    println!("{:?}",x);
    println!("{:?}",y);
    println!("even numbers are{:?}",find_even_numbers(&y)) ;
}

pub fn find_even_numbers(num: &Vec<i32>)->Vec<i32>{
    let mut even_numbers= Vec::new();
    for &val in num{
        if val%2 == 0{
            even_numbers.push(val);
        }
    }
    even_numbers
}