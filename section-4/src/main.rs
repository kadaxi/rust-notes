use std::io;

fn main() {
    // declare an array of 5 elements and using u8 type.
    let arr: [u8; 5] = [1, 2, 3, 4, 5];
    let mut index = String::new();

    println!("[!] Enter array index:");
    io::stdin()
        .read_line(&mut index)
        .expect("Error. Failed to read line.");

    let index: usize = index.trim().parse().expect("Error. Not an number.");
    let element = arr[index];

    // if element is greater equal array size, runtime error appears. Rust memory safety principle applied.
    println!("Element at index {index} is {element}");
}
