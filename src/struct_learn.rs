 pub struct Age {
     pub year_of_birth:u32,
     pub current_year:u32,
 }

impl Age{

    // Calculates the age by subtracting the year of birth from the current year.
       pub fn calculate_age(&self)->u32{
            &self.current_year - &self.year_of_birth
        }
        // Adds a given number of years to the current year and returns the result.
        pub fn next_year(&self,num:u32)->u32{
            &self.current_year + num
        }
        pub fn static_method()->(){
            println!("This is a static method");
        }
 }