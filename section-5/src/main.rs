// P5: Functions

// main function, entry point. Fn declares functions.
fn main() {
    println!("main function.");
    // parameters must respect type of passing value.
    foo_func(11, 'h');

    let y = {
        let x = 3;
        x + 1 // expression!
    };
    // let conditional, if result it's different from 11, x = 0.
    let x = if sum_val(5, 5) == 11 { 11 } else { 0 };

    // statements: no return, expression: return.
    println!("Y value: {y}"); // y value it's 4, cause we're retorning an expression.
    println!("X value: {x}"); // x value it's 0 if sum_val return other number than 11.

    let mut counter = 0;
    let result = loop {
        counter += 1; // increasing by 1 counter mutable variable (change outside variable).
        if counter == 10 {
            break counter * 2; // break loop and return 2 times counter.
        }
    };
    println!("Counter value: {result}");

    let mut count = 0;
    // labeled loop!
    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining = 10;

        loop {
            println!("Remaining = {remaining}");
            if remaining == 9 {
                break
            };
            if count == 2 {
                break 'counting_up; // breaking initial loop (father).
            };
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count: {count}");

    // while loop
    let mut lambda = 3;
    while lambda != 0 {
        println!("Lambda: {lambda}");
        lambda -= 1;
    }
    println!("Ready. Lambda: {lambda}");

    loop_arr();

    numbers_reverse();
}

// another function, convention: snake case "_".
// functions can be defined after or before main function.
fn foo_func(x: i32, unit: char) {
    println!("another function, x value: {x}{unit}");
}

// return an u32 value, can delete return and put the result without semicolon (implicitily).
fn sum_val(x: u32, y: u32) -> u32 {
    return x + y;
}

// two methods for loop inside an array.
fn loop_arr() {
    let arr:[u32;5] = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < 5 {
        println!("Arr value: {}", arr[index]);
        index += 1;
    }

    for element in arr {
        println!("Element: {element}")
    }
}

// method for looping numbers from n to n-k, using .rev() method.
fn numbers_reverse() {
    for num in (1..4).rev() {
        println!("Number: {num}");
    };
    println!("Finish.");
}