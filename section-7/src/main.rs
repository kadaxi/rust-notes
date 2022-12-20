// P7: List

fn main() {
    let mut s = String::from("Hello world.");
    let word = get_first_word(&s);
    println!("{word}"); // returns 5
    s.clear(); // bug, s word it's empty now, 5 index doesn't work for us.

    let s2 = String::from("hello world");
    let hello = &s2[..5]; // takes an reference string from 0 to 5 index.
    let world = &s2[6..11]; // from 6 to 11 string index. Also [6..] it's valid.
    println!("{hello} {world}"); // hello world

    let s3 = String::from("Bob Smith");
    let word_s3 = get_first_word_reference(&s3);
    println!("Word: {}", word_s3);

    // arrays
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3]; // takes from first to third element
    assert_eq!(slice, &[2, 3]); // macro
}

// bad solution, use instead slices.
fn get_first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // conver string to an array of bytes with as_bytes() method.
    // iter it's a method that returns each element in a collection
    // enumerate wraps result of iter() and return a tuple (index, reference to element).
    for (idx, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // if we find a space whe return an index, else a string len.
            return idx;
        }
    }
    return s.len();
}

fn get_first_word_reference(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (idx, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..idx]; // return word from 0 to index (when space appears).
        }
    }
    return &s[..]; // returns complete string.
}