use::std::collections::HashMap;
pub fn hashmap(){
    let mut a= HashMap::new();
     a.insert("sudhan", 1);
     a.insert("madhan", 2);
     println!("{:?}",a);
     println!("{:?}",a.get("sudhan"));
     let checkItExist = a.get("sudhan");
    match checkItExist{
        Some(value )=> println!("{}",value),
        None => println!("No value found"),
    }
    input_to_hashmap();
}

pub fn vec_to_hashmap(vec :Vec<(String,i32)>)->HashMap<String,i32>{
    let mut a = HashMap::new();
    for (key,value) in vec{
        a.insert(key,value);
    }
    return a;
 }

 pub fn input_to_hashmap(){
     let a = vec![(String::from("sudhan"),1),(String::from("madhan"),2)];
        let b = vec_to_hashmap(a);
        println!("Vector to hashMap {:?}",b);
 }

//  pub fn hashmap_to_vec(hashmap: HashMap<String,i32>)->Vec<i32>{
//          let a = Vec::new();
//          let b:Vec<i32> = a.iter().map(|(&x,&y)|(x,y)).collect();
//          return b;
//  }


