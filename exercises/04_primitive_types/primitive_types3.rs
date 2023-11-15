// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.

fn main() {
    let a: [i32; 100] = [0; 100]; 

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
