 pub enum Direction{
    North,
    East,
    West,
    South
 }

 pub enum Shape{
   Square(f64),
   Rectangle(f64,f64)
 }

    /// Prints the current direction given as a parameter.
    /// The direction is expected to be one of the four cardinal directions:
    /// North, East, West, or South.
 pub fn current_direction(dir: Direction){
    match dir{
        Direction::North => println!("your current direction is north"),
        Direction::East => println!("your current direction is east"),
        Direction::West => println!("your current direction is west"),
        Direction::South => println!("your current direction is south"),
    }
 }

 pub fn area_calculate(shape: Shape)->f64{
    match shape{
      Shape::Square(value) => value * value,
      Shape::Rectangle(a, b) => a * b,
    }
 }

/// Returns the index of the first occurrence of 'a' in the given string.
/// If there is no 'a' in the string, returns None.
pub fn find_index_of_the_a_in_the_string(str: &str) -> Option<i32> {
   for (index, char) in str.chars().enumerate() {
       if char == 'a' {
           return Some(index as i32);
       }
   }
   return None;
}
