// P6: OWNERSHIP

/*
- STACK:
        * Last in, first out LIFO. Works like a pile of element, while you put cube over cube, last in will be first out.
        * Adding data: pushing onto the stack.
        * Removing data: popping off the stack.
        * Data on stack must be known with an fixed size. Heap it's for unknow data size or variable size.
- HEAP:
        * Less organized. Pointer it's the address location.
        * Adding data to heap: Allocating on the heap or allocating.
        * Pushing to stack it's faster than heap, cause allocator never search an exact place, just put over the stack.
        * Accesing data in heap it's slower than accessing data on stack cause it's following a pointer.
        * Main purpose of ownership it's manage heap data.
- Ownership:
        * Each value in Rust has an owner.
        * Can be only one owner at a time.
        * When the owner goes out of scope, the value will be dropped.
*/

fn main() {
    // Variables scope
    variables_scope();

    // Scope
    let s = String::from("Hello");
    // takes_ownership(s); => not passing as reference, changing ownership!
    // println!("String 2: {}", s); => Error appear, value moved to previus invoque function.
    takes_ownership(&s); // passing as reference, can use variable after function invoque.
    println!("String 2: {}", s); // works doing reference!

    let mut s = String::from("Hello");
    mutable_reference(&mut s); // must be just one mutable reference for a value at the time.
    println!("Mutable string: {s}"); // passing reference mutable change actual string value cause change data.
}

fn variables_scope() {
    // String lives at the heap (we can't know the size). Literal it's an static &str
    let mut name = String::from("Bob"); // mutable string, from request the memory needs.
    name.push_str(" Smith"); // push_str() method appends a literal to a String.
    println!("Name: {name}"); // Name: Bob Smith

    let s1 = String::from("Hello");
    let s2 = s1.clone(); // without clone, s1 will be deleted.
    println!("s1 = {s1}, s2 = {s2}");
}

fn takes_ownership(string: &String) {
    println!("String: {}", string);
}

fn mutable_reference(str_value: &mut String) {
    str_value.push_str(", world"); // modifying a mutable string passed as reference.
}