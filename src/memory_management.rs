pub fn ownership(){
    let a:String = String::from("Hello");
    let b = a;
    println!("{}",b); // owner of hello is b because we move the ownership a to b
   // println!("{}",a); // Did not prin because the owner of the string is a 
   cloning();
   moving();
}

pub fn cloning(){
    let a :String  = String::from("Hello Sudhan");
    let b:String = a.clone();
    println!("{}",a);
    println!("{}",b); //you can access both value because hello sudhan store 2 times in heap memory 
}

pub fn moving(){
    let a:String = String::from("Happy Coding");
    dublicate(a);
    //println!("{}",a); // you coudnot print because the owner of the string is moved
}

pub fn dublicate(a:String){
    println!("{}",a);
}